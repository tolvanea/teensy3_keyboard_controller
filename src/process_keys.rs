//! This file contains methods to reading GPIO pin connections to valid key presses.
//! This does not contain any logic separating modifier keys from regular keys.
//! Main function is `scan_key_press` which returns list of currently pressed keys.
//!
//! Quite a lot of logic is implemented for that case that multiple keys are pressed.
//! Naive implementation would register ghost presses, but this does not. Also this
//! implementation is quite sophisticated, and it detects all keys that are possible
//! to be deteceted.
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

/// This is one central object of whole project. It is used to read GPIO pin connections and to
/// output a list of pressed keys. The central function for that purpose is `scan_key_press`.
#[derive(Debug)]
pub struct KeyMatrix {
    /// Key code matrix
    pub code_matrix: ShortVec<ShortVec<Option<u32>>>,
    /// Voltage source pins. Index corresponds row index in matrix.
    pub row_pins: ShortVec<Pin>,
    /// Voltage drain pins. Index corresponds column index in matrix.
    pub col_pins: ShortVec<Pin>,
    /// Other less important fields in Key matrix
    pub info: ExtraKeyInfo,
}

#[derive(Debug)]
/// Some extra information about key codes. This is not-so-interesting field of `KeyMatrix`
pub struct ExtraKeyInfo {
    /// Fn key code
    pub fn_key: u32,
    /// Media key bindings when Fn is pressed
    pub media_key_bindings: ShortVec<(u32, u32)>,
    /// Byte masks for regular keys. The byte mask is second byte of key code (u32)
    pub regular_key_mask: u8,
    /// Byte masks for modifier keys.
    pub modifier_key_mask: u8,
}

impl KeyMatrix {
    /// It's highly recommended to create key matrix as in `custom_key_codes::get_stored_key_codes`.
    /// # Arguments
    /// * `mat`  Matrix of key codes
    /// * `rows` Vector, index corresponds row in matrix, and value corresponds GPIO port number
    /// * `cols` Vector, index corresponds column in matrix and, value corresponds GPIO port number
    /// * `info` Information about key codes
    pub fn new(
        pinrow: &mut PinRow,
        code_matrix: ShortVec<ShortVec<Option<u32>>>,
        rows: ShortVec<usize>,
        cols: ShortVec<usize>,
        info: ExtraKeyInfo,
    ) -> KeyMatrix {
        let row_pins: ShortVec<Pin> = rows.iter().map(|&i| {
            pinrow.get_pin(i, PinMode::InputPullup)
        }).collect();
        let col_pins: ShortVec<Pin> = cols.iter().map(|&j| {
            let mut p = pinrow.get_pin(j, PinMode::OutputOpenDrain);
            p.digital_write(true);  // By default disable drain
            p
        }).collect();
        return KeyMatrix { code_matrix, row_pins, col_pins, info};
    }


    /// Scan key matrix GPIO connections, and return list of currently pressed keys.
    /// Return None if nothing is pressed.
    pub fn scan_key_press(&mut self) -> Option<ShortVec<KeyCode<u32>>> {
        // `mat` keeps book about what keys may be conflicted with other key presses.
        // Conflicts may occur if multiple keys are pressed at the same time.
        // By knowing what presses are "ghost" artifacts, they can be dropped out. Those that
        // can not be discarded, will be informed to caller that they are uncertain
        let mut mat: ShortVec<ShortVec<KeyState>>
            = full_vec(full_vec(Free, self.col_pins.len()), self.row_pins.len());
        let mut erroneous_keys: ShortVec<(usize, usize)> = Vec::new();  // *potentially erroneous
        // Performance: Delays takes about 9000 us and conflict detection about 50-100 us
        for (col, drain) in self.col_pins.iter_mut().enumerate() {
            drain.digital_write(false);  // enable drain
            for (row, source) in self.row_pins.iter().enumerate() {
                let pressed = !source.digital_read();  // check if connected
                if pressed {
                    mat[row][col] = match self.code_matrix[row][col] {
                        Some(c) => {
                            let conflict = scan_for_conflicts(&mut mat, row, col, true);
                            if conflict { Maybe(c) } else { Pressed(c) }
                        },
                        None => {  // Uh oh, such connection should not exists for any key.
                            // One can not be yet sure whether error originates from multiple key
                            // presses or poorly configured key matrix. That's why `erroneous_keys`
                            // is checked later.
                            erroneous_keys.push((row, col)).unwrap_or(());
                            Free
                        },
                    }
                }
            }
            drain.digital_write(true);  // disable drain
            delay(1); // It takes time for pullup pin to charge back to full voltage
        }
        erroneous_keys.into_iter()
            .filter(|&(i, j)| !scan_for_conflicts(&mut mat, i, j, false))
            .for_each(|(i, j)| {
                println!("Warning! Detected pin connection correspondin to matrix element ({}, {}),\
                    \nwhich does not have any key assigned in the key matrix.", i, j);
            });
        let keys: ShortVec<KeyCode<u32>> = mat.iter()
            .flatten()
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
    update: bool
) -> bool {
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
        // for fourth corner also. So all potentially conflicting keys are marked
        // as "Maybe". However if opposing corner is has not valid key, then we know
        // that it can not be pressed. In that case these three keys can be pressed
        // without ambiguities.
        let mut conflict = false;
        for &r_row in reserved_rows.iter() {
            for &r_col in reserved_cols.iter() {
                let opposing_corner_is_reserved = mat[r_row][r_col] != Free;
                if opposing_corner_is_reserved {
                    conflict = true;
                    if !update {
                        return conflict;
                    }
                    if let Pressed(c) = mat[r_row][r_col] {
                        mat[r_row][r_col] = Maybe(c);
                    }
                    if let Pressed(c) = mat[r_row][col] {
                        mat[r_row][col] = Maybe(c);
                    }
                    if let Pressed(c) = mat[row][r_col] {
                        mat[row][r_col] = Maybe(c);
                    }
                }
            }
        }
        return conflict;
    }

}
