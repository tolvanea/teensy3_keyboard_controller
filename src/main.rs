#![no_std]
#![no_main]

#[macro_use]
extern crate teensy3;

use heapless::{Vec}; // fixed capacity `std::Vec`
use typenum::{Unsigned, U16 as MatrixSize, U64 as MaxPins, U256 as MaxKeys};

use teensy3::util::{delay};
use teensy3::pins::{PinMode, Pin, PinRow, NUM_PINS, LED_PIN};
use teensy3::bindings as b;

fn setup() -> PinRow {
    // It's unsafe because caller verifies that it's called only once
    unsafe{ PinRow::new_once()}
}

const ROWS: usize = 16;
const COLUMNS: usize = 8;

/// Find out what to pins are connected. This corresponds to button press. Scan pins
/// by iterating all possible pin combinations, which is not very efficient, if key matrix is
/// known. So use this only when you do not know how many columns and rows key matrix has.
/// TODO This panics if multible keys are pressed at the same time.
fn scan_key_press(pinrow: &mut PinRow) -> Option<(usize, usize)>{
    // Connected pins. There should be only ONE pin pair connected, but
    let mut connection: Option<(usize, usize)> = None;

    assert!(NUM_PINS <= MaxPins::to_usize(), "Allocated memory ran out, too many pins");
    // Set all pins to drain mode, but by default disable them. They will be turned on
    // only to check whether some particular connection exists
    let mut pins: Vec<Pin, MaxPins> = (0..NUM_PINS).filter(|&i| i != LED_PIN)
        .map(|i| {
            let mut p = pinrow.get_pin(i, PinMode::OutputOpenDrain);
            p.digital_write(true);  // By default disable drain
            p
        })
        .collect();
    // Check connections, and set drain pins one by one to source pins.
    for i in 0..pins.len() {
        // Pins [0..i+1] are source pins "i", and [i+1..NUM_PINS] are drain pins "j"
        let (i_pins, j_pins) = pins.split_at_mut(i+1);
        let pin_i = &mut i_pins[i];
        pin_i.set_mode(PinMode::InputPullup);  // Make `pin_i` voltage source
        delay(1);
        for (j, pin_j) in j_pins.iter_mut().enumerate() {
            pin_j.digital_write(false);  // enable drain
            let pressed = !pin_i.digital_read();  // check if `pin_i` and `pin_j` are connected
            pin_j.digital_write(true);  // disable drain
            if pressed {delay(1);}  // It takes time for pullup pin to charge back!

            let i_real_idx = if i < LED_PIN {i} else {i+1};
            let j_real_idx = if i+1+j < LED_PIN {i+1+j} else {i+2+j};
            if pressed {
                //println!("connection: {:?}", (i_real_idx, j_real_idx));
                if connection == None {
                    connection = Some((i_real_idx, j_real_idx));
                } else {
                    //println!("TODO panic"); // TODO
                    panic!("Multiple connections found: {:?} and {:?}",
                        connection.unwrap(), (i_real_idx, j_real_idx))
                }
            }
        }
    }
    pins.into_iter().for_each(|pin| pinrow.return_pin(pin));
    return connection;
}

fn wait_for_key(pinrow: &mut PinRow) -> (usize, usize) {
    let pair = loop {
        match scan_key_press(pinrow) {
            Some(pair) => {break pair;},
            None => {delay(5);},
        }
    };
    return pair;
}

/// Utility tool that finds out key matrix. User presses through every single key through in
/// keyboard. Keys 'Enter' and 'Space' are used to fix typos or other problems in typing prosess.
/// If typo is made, 'Enter' can be pressed and current row is restarted.
/// If some key does not seem to work, 'Space' can be pressed, which skips that key, and leaves
/// that out of matrix.
///
/// This function does not generate actual key matrix itself, textual representation of it, so
/// it can be copy-pasted to source code.
/// # Arguments
/// * pinrow
///     * PinRow singleton
/// * key_codes
///     * This contains names of all key codes in same order that they will be pressed. The
///       list is divided in parts, which makes key typing process easier, because key
///       pressing can is divided to many parts. For example, these parts correspond physical
///       rows in key board (capslock, a, s, d, f, ...).
///       MAKE SURE THAT THE FIRST LIST IN `key_codes` CONTAINS ONLY ENTER AND SPACE.
///       Like so:
///       ```
///       &[
///         &["KEY_ENTER", "KEY_SPACE"],
///         &["KEY_ESC", "KEY_F1", "KEY_F2", ...],
///         &["KEY_TILDE", "KEY_1", "KEY_2", ...],
///         &["KEY_TAB", "KEY_Q", "KEY_W", ...],
///         ...
///       ]
///       ```
///       (Actually first keys need not to be enter and space, but whatever they are, they
///       will be used as error handling keys described above.)
fn figure_out_key_matrix<'a>(pinrow: &mut PinRow, key_codes: &[&[&'a str]])
    -> Vec<Vec<Option<&'a str>, MatrixSize>, MatrixSize>
{
    assert!(key_codes[0].len() == 2,
        r#"First list in `key_codes` should contain only Enter and Space.\n\
        The list may look something like following:\n\
        `&[\
            &["KEY_ENTER", "KEY_SPACE"], \n\
            &["KEY_ESC", "KEY_F1", "KEY_F2", ...], \n\
            &["KEY_TILDE", "KEY_1", "KEY_2", ...],\n\
            &["KEY_TAB", "KEY_Q", "KEY_W", ...],\n\
            ...
         ]`"#);
    print!("Press enter. ");
    let enter = wait_for_key(pinrow);
    println!("Ok, enter found to correspond pair {:?}.", enter);
    delay(200);
    print!("Press space. ");
    let space = wait_for_key(pinrow);
    assert!(space != enter, "Space and enter can not be same key!");
    println!("Ok, space found to correspond pair {:?}.", space);
    delay(200);

    println!("Next, start typing keys one key at a time corresponding the input parameter\n\
        `key_codes`. If you misstyped some keys and want to start typing that row\n\
        again, press enter. If you want to skip that key, press space. (By the way, there\n\
        can be total maximum of 256 keys.)\n");

    //let mut total_idx = 0;
    let key_codes_len = key_codes.len();
    let mut key_codes_itr = key_codes.iter();
    let &enter_and_space = key_codes_itr.next().unwrap();
    let mut keys: Vec<(usize, usize, &str), MaxKeys> = Vec::new();
    keys.push((enter.0, enter.1,  enter_and_space[0]));
    keys.push((space.0, space.1,  enter_and_space[1]));
    println!("Row 1/{} of keycodes succesfully processed.", key_codes_len);

    for (row_idx, &row) in key_codes_itr.enumerate() {
        println!("Starting row {}/{}, consiting total of {} keys.", row_idx+2, key_codes_len, row.len());
        let old_len = keys.len();
        'outer: loop {
            'inner: for (key_idx, &key_code) in row.iter().enumerate() {
                delay(200);
                print!("     Press key {}/{}: {}. ", key_idx+1, row.len(), key_code);
                let pair = wait_for_key(pinrow);
                if pair == enter {
                    println!("Starting the same row again.");
                    keys.resize(old_len, (0,0,"")).unwrap();
                    break 'outer;
                } else if pair == space {
                    println!("Skipping that key.");
                } else {
                    if keys.iter().any(|&(i, j, _)| (i,j) == pair) {
                        println!("That key has been already pressed! Strarting the whole row again.");
                        keys.resize(old_len, (0,0,"")).unwrap();
                        break 'outer;
                    }
                    println!("Check.");
                    assert!(keys.len() < MaxKeys::to_usize(),
                        "Maximum number of keys {} exceeded.",
                        MaxKeys::to_usize());
                    keys.push((pair.0, pair.1, key_code)).unwrap();
                }
            }
            break;  // Typing a row succeeded.
        }
    }
    // Find out input pins and output pins
    let mut is: Vec<usize, MaxPins> = Vec::new();
    let mut js: Vec<usize, MaxPins> = Vec::new();
    for &(i, j, _code) in keys.iter() {
        if is.iter().position(|&ii| ii==i).is_none() {
            is.push(i).unwrap();
        }
        if js.iter().position(|&jj| jj==j).is_none() {
            js.push(j).unwrap();
        }
    }
    {
        let js: &mut [usize] = js.as_mut();
        js.sort_unstable()
    }

    assert!(is.len() <= MatrixSize::to_usize(), "Too many pins found (>16), allocated memory ran out.");
    assert!(js.len() <= MatrixSize::to_usize(), "Too many pins found (>16), allocated memory ran out.");
    let mut matrix: Vec<Vec<Option<&str>, MatrixSize>, MatrixSize> = Vec::new();
    let mut empty_row: Vec<Option<&str>, MatrixSize> = Vec::new();
    let mut column_max_width: Vec<usize, MatrixSize> = Vec::new();
    empty_row.resize(js.len(), None).unwrap();
    column_max_width.resize(js.len(), usize::MIN).unwrap();
    matrix.resize(is.len(), empty_row).unwrap();
    for &(i, j, code) in keys.iter() {
        let i_idx = is.iter().position(|&a| a==i).unwrap();
        let j_idx = js.iter().position(|&a| a==j).unwrap();
        let cell = &mut matrix[i_idx][j_idx];
        assert!(cell.is_none(), "Multiple keys for same matrix cell ({},{}) {} and {}",
                i, j, cell.unwrap(), code);
        *cell = Some(code);
        column_max_width[j_idx] = column_max_width[j_idx].max(code.len())
    }

    println!("let matrix = [");

    for (i, row) in matrix.iter().enumerate() {
        print!("    [");
        for (name, width) in row.iter().zip(column_max_width.iter()) {
            print!("{name:>width$}, ", name=name.unwrap_or("0"), width=width);
        }
        println!("],");
    }
    println!("];");

    return matrix;
}


#[no_mangle]
pub extern fn main() {
    let mut pinrow = setup();
    let mut led = pinrow.get_led();
    for _ in 0..3 {
        alive(&mut led);
    }
    println!("Hellouu!");
    // THIS!
    //let _p3 = pinrow.get_pin(3, PinMode::OutputOpenDrain);
    //let p4 = pinrow.get_pin(4, PinMode::InputPullup);

    let key_codes: &[&[&str]] = &[
        &["b::KEY_ENTER", "b::KEY_SPACE"],
        &["b::KEY_Q", "b::KEY_W", "b::KEY_R"],
        &["b::KEY_A", "b::KEY_S", "b::KEY_D"],
    ];

    let matrix = figure_out_key_matrix(&mut pinrow, key_codes);
    println!("{:?}", matrix);

    for _j in 0.. {
        //println!("j {}", j);
        let _pair = wait_for_key(&mut pinrow);
        delay(100)
    }

    led.digital_write(false); // Set led off
    let mut i = 0;

    // Blink Loop
    loop {
        i += 1;
        println!("{}", i);
        // Show we are alive by blinking
        alive(&mut led);
        // unsafe {
        //     println!("{} 3-4: {}{}", i,
        //         bindings::digitalRead(3), bindings::digitalRead(4));
        // }
        // if !p4.digital_read() {
        //     println!("#############################################");
        // }

        // Keep 2 second pause in blinking the led, also don't spam the console
        //delay(1000);


    }
}

/// Blink the light twice to know we're alive
pub fn alive(led: &mut Pin) {
    // Blink led with custom wrapper
    for i in 0..6 {
        led.digital_write(i%2 == 0);
        delay(50);
    }
    delay(200)
}


