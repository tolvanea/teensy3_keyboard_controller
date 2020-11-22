use heapless::{Vec}; // fixed capacity `std::Vec`
pub use typenum::{U16 as MatrixCap};  // Maximum capacities

use teensy3::util::{delay};
use teensy3::pins::{Pin, PinRow, PinMode};

use super::{full_vec};

/// When keys are scanned, one voltage source GPIO pin can detect only one key press. This is
/// called "scan row". However if more than one key is pressed one scan row, then those presses
/// are not registered.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ScanRowState {
    NotPressed,
    Pressed(Option<u32>),
    TooManyKeysPressed,
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
        cols:Vec<usize, MatrixCap>,
    ) -> KeyMatrix {
        let row_pins: Vec<Pin, MatrixCap> = rows.iter().map(|&i| {
                pinrow.get_pin(i, PinMode::InputPullup)
            }).collect();
        let col_pins: Vec<Pin, MatrixCap> = cols.iter().map(|&j| {
                let mut p = pinrow.get_pin(j, PinMode::OutputOpenDrain);
                p.digital_write(true);  // By default disable drain
                p
            }).collect();
        return KeyMatrix{code_matrix, row_pins, col_pins};
    }

    /// Return None if nothing is pressed. Keycode is None if that value is not in matrix
    pub fn scan_key_press(&mut self) -> Option<Vec<ScanRowState, MatrixCap>> {
        let mut v: Vec<ScanRowState, MatrixCap> = full_vec(ScanRowState::NotPressed, self.row_pins.len());
        for (col, drain) in self.col_pins.iter_mut().enumerate() {
            drain.digital_write(false);  // enable drain
            for (row, source) in self.row_pins.iter().enumerate() {
                let pressed = !source.digital_read();  // check if connected
                if pressed {
                    let code = self.code_matrix[row][col];
                    v[row] = match v[row] {  // Test that if key scan row already been activated
                        ScanRowState::NotPressed => ScanRowState::Pressed(code),
                        ScanRowState::Pressed(_) => ScanRowState::TooManyKeysPressed,
                        ScanRowState::TooManyKeysPressed => ScanRowState::TooManyKeysPressed,
                    };
                }
            }
            drain.digital_write(true);  // disable drain
            delay(1); // It takes time for pullup pin to charge back to full voltage
        }
        if v.iter().all(|state| *state == ScanRowState::NotPressed) {
            return None;
        } else {
            return Some(v);
        }
    }
}
