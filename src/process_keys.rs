use heapless::{Vec}; // fixed capacity `std::Vec`

use teensy3::util::{delay};
use teensy3::pins::{Pin, PinRow, PinMode};

use super::{full_vec, ShortVec};

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
    pub fn into_inner(self) -> T {
        match self {
            Certain(code) => code,
            Uncertain(code) => code,
        }
    }
    pub fn into_option(self) -> Option<T> {
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
    pub code_matrix: ShortVec<ShortVec<Option<u32>>>,
    /// Voltage source pins
    pub row_pins: ShortVec<Pin>,
    /// Voltage drain pins
    pub col_pins: ShortVec<Pin>,
}

impl KeyMatrix {
    /// # Arguments
    /// * `mat`  Matrix of key codes
    /// * `rows` Vector, index corresponds row in matrix, and value corresponds GPIO port number
    /// * `cols` Vector, index corresponds column in matrix and, value corresponds GPIO port number
    pub fn new(
        pinrow: &mut PinRow,
        code_matrix: ShortVec<ShortVec<Option<u32>>>,
        rows: ShortVec<usize>,
        cols: ShortVec<usize>,
    ) -> KeyMatrix {
        let row_pins: ShortVec<Pin> = rows.iter().map(|&i| {
            pinrow.get_pin(i, PinMode::InputPullup)
        }).collect();
        let col_pins: ShortVec<Pin> = cols.iter().map(|&j| {
            let mut p = pinrow.get_pin(j, PinMode::OutputOpenDrain);
            p.digital_write(true);  // By default disable drain
            p
        }).collect();
        return KeyMatrix { code_matrix, row_pins, col_pins };
    }


    /// Return None if nothing is pressed. Keycode is None if that value is not in matrix
    pub fn scan_key_press(&mut self) -> Option<ShortVec<KeyCode<u32>>> {
        let mut mat: ShortVec<ShortVec<KeyState>>
            = full_vec(full_vec(Free, self.col_pins.len()), self.row_pins.len());
        // Performance: Delays takes about 9000 us and conflict detection about 50-100 us
        for (col, drain) in self.col_pins.iter_mut().enumerate() {
            drain.digital_write(false);  // enable drain
            for (row, source) in self.row_pins.iter().enumerate() {
                let pressed = !source.digital_read();  // check if connected
                if pressed {
                    let conflict = scan_for_conflicts(&mut mat, row, col, &self.code_matrix);
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
        let keys: ShortVec<KeyCode<u32>> = mat.iter()
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
    mat: &mut ShortVec<ShortVec<KeyState>>,
    row: usize,
    col: usize,
    code_mat: &ShortVec<ShortVec<Option<u32>>>
) -> bool {
    // TODO be certain if there is no 4th key
    assert!(mat[row][col] == Free);
    let reserved_cols: ShortVec<usize> = mat[row].iter().enumerate()
        .filter(|(_, k)| **k != Free)
        .map(|(r_col, _)| r_col)
        .collect();
    let mut reserved_rows: ShortVec<usize> = Vec::new();
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
