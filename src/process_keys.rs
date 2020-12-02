use heapless::{Vec}; // fixed capacity `std::Vec`
pub use typenum::{U24 as MatrixCap};  // Maximum capacities

use teensy3::util::{delay};
use teensy3::pins::{Pin, PinRow, PinMode};

use super::{full_vec};

// TODO scan also with sources and drains flipped to get more accuracy.

/// When keys are scanned, one voltage source GPIO pin can detect only one key press. This is
/// called "scan row". However if more than one key is pressed one scan row, then those presses
/// are not registered.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ScanState {
    UnPressed,
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

    fn swap_sources_and_drains(&mut self, changed: bool) {
        let (drain_pins, source_pins) = if changed {
            (&mut self.row_pins, &mut self.col_pins) // swapped order
        } else {
            (&mut self.col_pins, &mut self.row_pins) // original order
        };
            ;
        for drain in drain_pins.iter_mut().enumerate() {
            drain.set_mode(PinMode::OutputOpenDrain);
            drain.digital_write(true);
        }
        for source in source_pins.iter_mut().enumerate() {
            source.set_mode(PinMode::InputPullup);
        }

    }

    /// Return None if nothing is pressed. Keycode is None if that value is not in matrix
    pub fn scan_key_press(&mut self) -> Option<Vec<ScanState, MatrixCap>> {
        // Row scan
        let mut m: Vec<Vec<ScanState, MatrixCap>, MatrixCap> = full_vec(
            full_vec(ScanState::UnPressed, self.col_pins.len()),
            self.row_pins.len(),
        );
        let rows_activated: Vec<bool, MatrixCap> = full_vec(false, self.row_pins.len());
        let cols_activated: Vec<bool, MatrixCap> = full_vec(false, self.col_pins.len());
        let mut keys: Vec<ScanState, MatrixCap> = Vec::new();


        for iter in 0..2 {
            let original_order = iter == 0;
            let (drain_pins, source_pins) = if original_order {
                (&mut self.col_pins, &mut self.row_pins) // original order
            } else {
                core::mem::swap(&mut rows_activated, &mut cols_activated)
                (&mut self.row_pins, &mut self.col_pins) // swapped order
            };

            for (mut col, drain) in drain_pins.iter_mut().enumerate() {
                drain.digital_write(false);  // enable drain
                for (mut row, source) in source_pins.iter().enumerate() {
                    let pressed = !source.digital_read();  // check if connected
                    let (row, col) = if original_order { (row, col) } else {(col, row)};

                    if pressed {
                        let code = self.code_matrix[row][col];
                        // Test that if key scan row already been activated
                        m[row][col] = match m[row][col] {
                            ScanState::UnPressed => {

                                ScanState::Pressed(code)
                            },
                            ScanState::Pressed(_) => ScanState::TooManyKeysPressed,
                            ScanState::TooManyKeysPressed => ScanState::TooManyKeysPressed,
                        };
                    }
                }
                drain.digital_write(true);  // disable drain
                delay(1); // It takes time for pullup pin to charge back to full voltage
            }
            self.swap_sources_and_drains(original_order);
        }
        if v.iter().all(|state| *state == ScanState::UnPressed) {
            return None;
        } else {
            return Some(v);
        }
    }
}
