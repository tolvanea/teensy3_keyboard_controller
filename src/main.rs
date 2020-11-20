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
/// * `pinrow` :    PinRow singleton
/// * `key_codes` : This contains names of all key codes in same order that they will be pressed.
///                 The list is divided in parts, which makes key typing process easier, because key
///                 pressing can is divided to many parts. For example, these parts correspond
///                 physical rows in key board: (capslock, a, s, d, f, ...).
///                 MAKE SURE THAT THE FIRST LIST IN `key_codes` CONTAINS ONLY ENTER AND SPACE.
///                 Like so:
///                 ```
///                 &[
///                   &[KEY_ENTER, KEY_SPACE],
///                   &[KEY_ESC, KEY_F1, KEY_F2, ...],
///                   &[KEY_TILDE, KEY_1, KEY_2, ...],
///                   &[KEY_TAB, KEY_Q, KEY_W, ...],
///                   ...
///                 ]
///                 ```
///                 (Actually first keys need not to be enter and space, but whatever they are, they
///                 will be used as error handling keys described above.)
/// * `key_names`:  Similar list to `key_codes`, but contains names of keys. These names are
///                 printed for each key that will be typed. Also, key matrix is printed, so if
///                 key names are literal strings of corresponding key codes, then that that matrix
///                 can be directly copy-pasted to source code.

fn figure_out_key_matrix<'a>(pinrow: &mut PinRow, key_codes: &[&[u32]], key_names: &[&[&'a str]])
                             -> (Vec<Vec<Option<u32>, MatrixSize>, MatrixSize>, Vec<usize, MaxPins>, Vec<usize, MaxPins>)
{
    assert!(key_names[0].len() == 2,
            r#"First list in `key_codes` should contain only Enter and Space.\n\
        The list may look something like the following:\n\
        `&[\
            &[KEY_ENTER, KEY_SPACE], \n\
            &[KEY_ESC, KEY_F1, KEY_F2, ...], \n\
            &[KEY_TILDE, KEY_1, KEY_2, ...],\n\
            &[KEY_TAB, KEY_Q, KEY_W, ...],\n\
            ...
         ]`"#);
    let mut keys: Vec<(usize, usize, u32, &str), MaxKeys> = Vec::new();

    let key_codes_len = key_names.len();
    let mut key_codes_itr = key_codes.iter();
    let mut key_names_itr = key_names.iter();

    // Get pins corresponding enter and space
    let (enter, space) = {
        let &a = key_codes_itr.next().unwrap();
        let (enter_code, space_code) = (a[0], a[1]);
        let &b = key_names_itr.next().unwrap();
        let (enter_name, space_name) = (b[0], b[1]);

        print!("Press enter or '{}'. ", enter_name);
        let enter = wait_for_key(pinrow);
        println!("Ok, found it to correspond pair {:?}.", enter);
        delay(200);
        keys.push((enter.0, enter.1, enter_code, enter_name)).unwrap();

        print!("Press space or '{}'. ", space_name);
        let space = wait_for_key(pinrow);
        println!("Ok, space found to correspond pair {:?}.", space);
        assert!(space != enter, "Space and enter can not be same key!");
        delay(200);
        keys.push((space.0, space.1, space_code, space_name)).unwrap();
        println!("Row 1/{} of keycodes succesfully processed.", key_codes_len);

        (enter, space)
    };

    println!("Next, start typing keys one key at a time corresponding the input parameter\n\
        `key_codes`. If you misstyped some keys, the row can be restarted by pressing enter. \n\
        If you want to skip that key and not include that to matrix, press space.\n");


    for (row_idx, (&row_code, &row_name)) in key_codes_itr
        .zip(key_names_itr).enumerate() {
        println!("Starting row {}/{}, consiting total of {} keys.",
                 row_idx+2, key_codes_len, row_name.len());
        let old_len = keys.len();
        'outer: loop {
            for (key_idx, (&key_code, &key_name)) in row_code.iter()
                .zip(row_name).enumerate()
            {
                delay(100);
                print!("     Press key {}/{}: {}. ", key_idx+1, row_name.len(), key_name);
                let pair = wait_for_key(pinrow);
                if pair == enter {
                    println!("Starting the same row again.");
                    keys.resize(old_len, (0,0,0,"")).unwrap();
                    break 'outer;
                } else if pair == space {
                    println!("Skipping that key.");
                } else {
                    if keys.iter().any(|&(i, j, _, _)| (i,j) == pair) {
                        println!("That key has been already pressed! Strarting the whole row again.");
                        keys.resize(old_len, (0,0,0,"")).unwrap();
                        break 'outer;
                    }
                    println!("Check.");
                    assert!(keys.len() < MaxKeys::to_usize(),
                        "Maximum number of keys {} exceeded.",
                        MaxKeys::to_usize());
                    keys.push((pair.0, pair.1, key_code, key_name)).unwrap();
                }
            }
            break;  // Typing a row succeeded.
        }
    }
    // Find out input pins and output pins
    let mut is: Vec<usize, MaxPins> = Vec::new();
    let mut js: Vec<usize, MaxPins> = Vec::new();
    for &(i, j, _, _) in keys.iter() {
        if is.iter().position(|&ii| ii==i).is_none() {
            is.push(i).unwrap();
        }
        if js.iter().position(|&jj| jj==j).is_none() {
            js.push(j).unwrap();
        }
    }
    {
        let is: &mut [usize] = is.as_mut();
        is.sort_unstable();
        let js: &mut [usize] = js.as_mut();
        js.sort_unstable();
    }

    assert!(is.len() <= MatrixSize::to_usize(), "Too many pins found (>16), allocated memory ran out.");
    assert!(js.len() <= MatrixSize::to_usize(), "Too many pins found (>16), allocated memory ran out.");
    let mut matrix_codes: Vec<Vec<Option<u32>, MatrixSize>, MatrixSize> = is.iter()
        .map(|_| js.iter().map(|_| None).collect()).collect();
    let mut matrix_names: Vec<Vec<Option<&str>, MatrixSize>, MatrixSize> = is.iter()
        .map(|_| js.iter().map(|_| None).collect()).collect();
    // String max width for each column for pretty printing
    let mut column_max_width: Vec<usize, MatrixSize> = js.iter().map(|_| usize::MIN).collect();

    for &(i, j, code, name) in keys.iter() {
        let i_idx = is.iter().position(|&a| a==i).unwrap();
        let j_idx = js.iter().position(|&a| a==j).unwrap();
        let code_cell = &mut matrix_codes[i_idx][j_idx];
        let name_cell = &mut matrix_names[i_idx][j_idx];
        assert!(name_cell.is_none(), "Clash for same matrix item! ({},{}) {} and {}",
                i, j, name_cell.unwrap(), name);  // This is checked before, should never happen
        *code_cell = Some(code);
        *name_cell = Some(name);
        column_max_width[j_idx] = usize::max(column_max_width[j_idx], name.len());
    }

    println!("let matrix = [");
    for row in matrix_names.iter() {
        print!("    [");
        for (name, width) in row.iter().zip(column_max_width.iter()) {
            print!("{name:>width$}, ", name=name.unwrap_or("0"), width=width);
        }
        println!("],");
    }
    println!("];");

    return (matrix_codes, is, js);
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

    let key_codes: &[&[u32]] = &[
        &[b::KEY_ENTER, b::KEY_SPACE],
        &[b::KEY_Q, b::KEY_W, b::KEY_R],
        &[b::KEY_A, b::KEY_S, b::KEY_D],
    ];

    let key_names: &[&[&str]] = &[
        &["b::KEY_ENTER", "b::KEY_SPACE"],
        &["b::KEY_Q", "b::KEY_W", "b::KEY_R"],
        &["b::KEY_A", "b::KEY_S", "b::KEY_D"],
    ];

    let (code_matrix, is, js) = figure_out_key_matrix(
        &mut pinrow, key_codes, key_names
    );
    // let matrix: &[&[usize]] = [
    //     [       0,        0,        0, b::KEY_ENTER,        0,            0, ],
    //     [       0,        0,        0,            0,        0, b::KEY_SPACE, ],
    //     [       0,        0,        0,            0, b::KEY_Q,            0, ],
    //     [       0,        0,        0,            0, b::KEY_W,            0, ],
    //     [       0,        0,        0,            0, b::KEY_R,            0, ],
    //     [b::KEY_A, b::KEY_S, b::KEY_D,            0,        0,            0, ],
    // ];


    let mut keyboard = unsafe{b::Keyboard};
    for _ in 0..10000 {
        let pair = wait_for_key(&mut pinrow);
        let i_idx = match is.iter().position(|&a| a==pair.0) {
            Some(i) => i,
            None => {println!("Unknown key!"); continue;}
        };
        let j_idx = match js.iter().position(|&a| a==pair.1) {
            Some(i) => i,
            None => {println!("Unknown key!"); continue;}
        };
        match code_matrix[i_idx][j_idx] {
            Some(code) => {
                println!("Code: {}", code);
                //unsafe{keyboard.set_key1(code as u8);}
                if code != b::KEY_W {
                    unsafe { keyboard.press(code as u16); }
                    delay(100);
                    unsafe { keyboard.release(code as u16); }
                }
                else {
                    unsafe { keyboard.press(b::KEY_MINUS as u16); }
                    unsafe { keyboard.press(b::MODIFIERKEY_SHIFT as u16); }
                    unsafe { keyboard.press(b::KEY_COMMA as u16); }
                    delay(100);
                    unsafe { keyboard.release(b::KEY_COMMA as u16); }
                    unsafe { keyboard.release(b::MODIFIERKEY_SHIFT as u16); }
                    unsafe { keyboard.release(b::KEY_MINUS as u16); }
                }
            },
            None => {
                println!("No matrix item for this combination! {:?}", (i_idx, j_idx));
                continue;
            },
        }
    }


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


