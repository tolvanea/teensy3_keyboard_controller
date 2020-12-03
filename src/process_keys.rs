use heapless::{Vec}; // fixed capacity `std::Vec`
pub use typenum::{U24 as MatrixCap};  // Maximum capacities

use teensy3::util::{delay};
use teensy3::pins::{Pin, PinRow, PinMode};

use super::{full_vec};

/// KeyState corresponds to scan state of GPIO, accompanied with some extra information.
/// If three or more keys are pressed, it is not sure whether all registered key
/// presses are real or ghost artifacts.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum KeyState {
    /// Key is not pressed with certainty.
    Free,
    /// Key is pressed with certainty. Inner value corresponds to the key code.
    Pressed(u32),
    /// Key may or may not be pressed. Inner value corresponds to the key code.
    Maybe(u32),
    /// Unknown key is pressed which is not registered in key matrix
    Error,
}
use KeyState::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// KeyCode is state of potentially valid key press. This is subset of `KeyState`.
pub enum KeyCode<T> {
    /// Key is pressed with certainty. Inner value corresponds to the key code.
    Certain(T),
    /// Key may or may not be pressed. Inner value corresponds to the key code.
    Uncertain(T)
}
use KeyCode::*;
impl<T> KeyCode<T> {
    pub fn to_code(self: Self) -> T {
        match self {
            Certain(code) => code,
            Uncertain(code) => code,
        }
    }
    pub fn to_option(self: Self) -> Option<T> {
        match self {
            Certain(code) => Some(code),
            Uncertain(_) => None,
        }
    }
}

/// Translate GPIO pin port connection to key codes that will be send over usb
#[derive(Debug)]
pub struct KeyMatrix {
    /// Key code matrix
    pub code_matrix: Vec<Vec<Option<u32>, MatrixCap>, MatrixCap>,
    /// Voltage source pins
    pub row_pins: Vec<Pin, MatrixCap>,
    /// Voltage drain pins
    pub col_pins: Vec<Pin, MatrixCap>,
}

impl KeyMatrix {
    /// # Arguments
    /// * `mat`  Matrix of key codes
    /// * `rows` Vector, index corresponds row in matrix, and value corresponds GPIO port number
    /// * `cols` Vector, index corresponds column in matrix and, value corresponds GPIO port number
    pub fn new(
        pinrow: &mut PinRow,
        code_matrix: Vec<Vec<Option<u32>, MatrixCap>, MatrixCap>,
        rows: Vec<usize, MatrixCap>,
        cols: Vec<usize, MatrixCap>,
    ) -> KeyMatrix {
        let row_pins: Vec<Pin, MatrixCap> = rows.iter().map(|&i| {
            pinrow.get_pin(i, PinMode::InputPullup)
        }).collect();
        let col_pins: Vec<Pin, MatrixCap> = cols.iter().map(|&j| {
            let mut p = pinrow.get_pin(j, PinMode::OutputOpenDrain);
            p.digital_write(true);  // By default disable drain
            p
        }).collect();
        return KeyMatrix { code_matrix, row_pins, col_pins };
    }


    /// Return None if nothing is pressed. Keycode is None if that value is not in matrix
    pub fn scan_key_press(&mut self) -> Option<Vec<KeyCode<u32>, MatrixCap>> {
        let mut mat: Vec<Vec<KeyState, MatrixCap>, MatrixCap>
            = full_vec(full_vec(Free, self.col_pins.len()), self.row_pins.len());
        for (col, drain) in self.col_pins.iter_mut().enumerate() {
            drain.digital_write(false);  // enable drain
            for (row, source) in self.row_pins.iter().enumerate() {
                let pressed = !source.digital_read();  // check if connected
                if pressed {
                    let conflict = scan_for_conflicts(&mut mat, row, col);
                    mat[row][col] = match self.code_matrix[row][col] {
                        Some(c) => {
                            if conflict { Maybe(c) } else { Pressed(c) }
                        },
                        None => Error,
                    }
                }
            }
            drain.digital_write(true);  // disable drain
            delay(1); // It takes time for pullup pin to charge back to full voltage
        }
        let keys: Vec<KeyCode<u32>, MatrixCap> = mat.iter()
            .flatten()
            .inspect(|k| if **k==Error { println!("Warning! Unknown key in matrix."); })
            .filter_map(|k| match *k {
                Pressed(c) => Some(Certain(c)),
                Maybe(c) => Some(Uncertain(c)),
                _ => None
            }).collect();

        return if keys.len() > 0 {
            Some(keys)
        } else {
            None
        };
    }
}

fn scan_for_conflicts(
    mat: &mut Vec<Vec<KeyState, MatrixCap>, MatrixCap>,
    row: usize,
    col: usize,
) -> bool {
    assert!(mat[row][col] == Free);
    let reserved_cols: Vec<usize, MatrixCap> = mat[row].iter().enumerate()
        .filter(|(_, k)| **k != Free)
        .map(|(r_col, _)| r_col)
        .collect();
    let mut reserved_rows: Vec<usize, MatrixCap> = Vec::new();
    for r_row in 0..mat.len() {
        if mat[r_row][col] != Free {
            reserved_rows.push(r_row).unwrap();
        }
    }

    if reserved_rows.len() == 0 || reserved_cols.len() == 0 {
        // Everything ok, pressing key normally
        return false;
    } else {
        // Uh oh keyboard can not handle this situation! Now 2+1 corners of
        // some rectangle(s) in matrix are pressed, which would create ghost press
        // for fourth corner also. So all potentially conflicting keys are also
        // marked as "Maybe"
        for &r_row in reserved_rows.iter() {
            match mat[r_row][col] { //TODO .clone() ?
                Free => unreachable!(),
                Pressed(c) => { mat[r_row][col] = Maybe(c) },
                _ => {},
            };
        }
        for &r_col in reserved_cols.iter() {
            match mat[row][r_col] { //TODO .clone() ?
                Free => unreachable!(),
                Pressed(c) => { mat[row][r_col] = Maybe(c) },
                _ => {},
            };
        }
        for &r_row in reserved_rows.iter() {
            for &r_col in reserved_cols.iter() {
                match mat[r_row][r_col] { //TODO .clone() ?
                    Free => {},  // Even though not yet scanned, it will be written later to `Maybe`
                    Pressed(c) => { mat[r_row][r_col] = Maybe(c) },
                    _ => {},
                };
            }
        }
        return true;
    }

}
