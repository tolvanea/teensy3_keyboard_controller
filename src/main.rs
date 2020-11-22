#![no_std]
#![no_main]

#[macro_use]
extern crate teensy3;

use heapless::{Vec, ArrayLength}; // fixed capacity `std::Vec`
use typenum::{Unsigned, U16 as MatrixCap, U64 as PinsCap, U256 as KeysCap};  // Maximum capacities

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

    assert!(NUM_PINS <= PinsCap::to_usize(), "Allocated memory ran out, too many pins");
    // Set all pins to drain mode, but by default disable them. They will be turned on
    // only to check whether some particular connection exists
    let mut pins: Vec<Pin, PinsCap> = (0..NUM_PINS).filter(|&i| i != LED_PIN)
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

fn full_vec<T, U>(value: T, len: usize) -> Vec<T,U>
where T: Clone, U: ArrayLength<T>
{
    let mut a = Vec::<T, U>::new();
    a.resize(len, value).unwrap();
    return a;
}

#[derive(Debug, Clone)]
struct KeyMatrix {
    code_matrix: Vec<Vec<Option<u32>, MatrixCap>, MatrixCap>,
    row_to_pin: Vec<usize, MatrixCap>,
    col_to_pin: Vec<usize, MatrixCap>,
    pin_to_row: Vec<Option<usize>, PinsCap>,
    pin_to_col: Vec<Option<usize>, PinsCap>,
}

impl KeyMatrix {
    /// Fetch the keycode corresponding connection of some pin indices. If there is no keycode
    /// for that pair, panic.
    fn get(&self, i: usize, j: usize) -> Option<u32> {
        let (i, j) = if i < j {(i, j)} else {(j, i)};  // put in order
        if i < self.pin_to_row.len() || j < self.pin_to_col.len() {
            self.code_matrix[self.pin_to_row[i]?][self.pin_to_col[j]?]
        } else {
            None  // Out of bounds
        }
    }
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
///
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
///
/// * `key_names`:  Similar list to `key_codes`, but contains names of keys. These names
///                 are printed for each key that will be typed. Also, key matrix is printed, so if
///                 key names are literal strings corresponding CONSTS in source code, then that
///                 that matrix can be directly copy-pasted to source code.
fn figure_out_key_matrix<'a>(
    pinrow: &mut PinRow,
    key_codes: &[&[u32]],
    key_names: &[&[&'a str]]
) -> KeyMatrix
{
    assert!(key_codes[0].len() == 2,
        r#"First list in `key_codes` should contain only Enter and Space.\n\
        The list may look something like the following:\n\
        `&[\
            &[KEY_ENTER, KEY_SPACE], \n\
            &[KEY_ESC, KEY_F1, KEY_F2, ...], \n\
            &[KEY_TILDE, KEY_1, KEY_2, ...],\n\
            &[KEY_TAB, KEY_Q, KEY_W, ...],\n\
            ...
         ]`"#);
    let mut keys: Vec<(usize, usize, u32, &str), KeysCap> = Vec::new();

    let key_codes_len = key_codes.len();
    let mut key_codes_itr = key_codes.iter();
    let mut key_names_itr= key_names.iter();

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
                 row_idx+2, key_codes_len, row_code.len());
        let old_len = keys.len();
        'ask: loop {  // Ask same row of keys again if user makes a typo writing it
            for (key_idx, (&key_code, &key_name)) in row_code.iter()
                .zip(row_name).enumerate() {
                delay(100);
                print!("     Press key {}/{}: {}. ", key_idx+1, row_code.len(), key_name);
                let pair = wait_for_key(pinrow);
                if pair == space {  // skip broken key
                    println!("Skipping that key.");
                } else if pair == enter {  // Fix typing mistake
                    println!("Starting the same row again.");
                    keys.resize(old_len, (0,0,0,"")).unwrap();
                    continue 'ask;
                } else {  // The correct path for key press
                    if keys.iter().any(|&(i, j, _, _)| (i,j) == pair) {
                        println!("That key has been already pressed! Strarting the whole row again.");
                        keys.resize(old_len, (0,0,0,"")).unwrap();
                        continue 'ask;
                    }
                    println!("Check.");
                    assert!(keys.len() < KeysCap::to_usize(),
                        "Maximum number of keys {} exceeded.",
                        KeysCap::to_usize());
                    keys.push((pair.0, pair.1, key_code, key_name)).unwrap();
                }
            }
        }
    }
    // Find out input pins and output pins
    // row_to_pin: Index is row in matrix and value is pin number
    let mut row_to_pin: Vec<usize, MatrixCap> = Vec::new();
    // col_to_pin: Index is column in matrix and value is pin number
    let mut col_to_pin: Vec<usize, MatrixCap> = Vec::new();

    for &(i, j, _, _) in keys.iter() {
        assert!(
            row_to_pin.len() <= MatrixCap::to_usize() && col_to_pin.len() <= MatrixCap::to_usize()
            , "Too many pins found (>16), allocated memory ran out."
        );
        if row_to_pin.iter().position(|&ii| ii==i).is_none() {
            row_to_pin.push(i).unwrap();
        }
        if col_to_pin.iter().position(|&jj| jj==j).is_none() {
            col_to_pin.push(j).unwrap();
        }
    }
    // Sort them (Yes, syntax is ugly as vector is sorted by using slice cast)
    AsMut::<[usize]>::as_mut(&mut row_to_pin).sort_unstable();  // useless, already ordered
    AsMut::<[usize]>::as_mut(&mut col_to_pin).sort_unstable();

    // The inverses of `row_to_pin` and `col_to_pin`.
    // That is, index corresponds index of pin, and value corresponds the row/column in matrix
    let mut pin_to_row: Vec<Option<usize>, PinsCap> = full_vec(None, row_to_pin.len());
    let mut pin_to_col: Vec<Option<usize>, PinsCap> = full_vec(None, col_to_pin.len());

    for (row, &pin) in  row_to_pin.iter().enumerate() {
        pin_to_row[pin] = Some(row);
    }
    for (col, &pin) in  col_to_pin.iter().enumerate() {
        pin_to_col[pin] = Some(col);
    }

    let mut code_matrix: Vec<Vec<Option<u32>, MatrixCap>, MatrixCap>
        = full_vec(full_vec(None, col_to_pin.len()), row_to_pin.len());
    let mut name_matrix: Vec<Vec<Option<&str>, MatrixCap>, MatrixCap>
        = full_vec(full_vec(None, col_to_pin.len()), row_to_pin.len());
    let mut column_max_width: Vec<usize, MatrixCap>  // Width for each column for pretty printing
        = full_vec(usize::MIN, col_to_pin.len());

    for &(i, j, code, name) in keys.iter() {
        let i_idx = pin_to_row[i].unwrap();
        let j_idx = pin_to_col[j].unwrap();
        let code_cell = &mut code_matrix[i_idx][j_idx];
        let name_cell = &mut name_matrix[i_idx][j_idx];
        assert!(name_cell.is_none(), "Clash for same matrix item! ({},{}) {} and {}",
                i, j, name_cell.unwrap(), name);  // This is checked before, should never happen
        *code_cell = Some(code);
        *name_cell = Some(name);
        column_max_width[j_idx] = usize::max(column_max_width[j_idx], name.len());
    }

    println!("Here's key matrix. You can copy-paste it to source code.");
    println!("let matrix = [");
    for row in name_matrix.iter() {
        print!("    [");
        for (name, width) in row.iter().zip(column_max_width.iter()) {
            print!("{name:>width$}, ", name=name.unwrap_or("0"), width=width);
        }
        println!("],");
    }
    println!("];\n");
    let mat = KeyMatrix{code_matrix, row_to_pin, col_to_pin, pin_to_row, pin_to_col};
    println!("Here's, raw representation of key matrix. This can be too copy-pasted to source.");
    println!("{:#?}", mat);
    return mat;
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

    let mat = figure_out_key_matrix(
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
        let (i,j) = wait_for_key(&mut pinrow);
        if let Some(code) = mat.get(i,j) {
            println!("Code: {}", code);
            //unsafe{keyboard.set_key1(code as u8);}
            unsafe { keyboard.press(code as u16); }
            delay(100);
            unsafe { keyboard.release(code as u16); }
        } else {
            println!("No matrix item for this combination! {:?}", (i, j));
            continue;
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


