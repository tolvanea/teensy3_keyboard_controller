use heapless::{Vec}; // fixed capacity `std::Vec`
pub use typenum::{U24 as MatrixCap};  // Maximum capacities

use teensy3::util::{delay};
use teensy3::pins::{Pin, PinRow, PinMode};

use super::{full_vec};

// TODO scan also with sources and drains flipped to get more accuracy.

///// When keys are scanned, one voltage source GPIO pin can detect only one key press. This is
///// called "scan row". However if more than one key is pressed one scan row, then those presses
///// are not registered.
// #[derive(Debug, Copy, Clone, Eq, PartialEq)]
// pub enum KeyState {
//     Free,
//     Pressed(Option<u32>),
//     Maybe(Option<u32>),
// }

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

    // fn swap_sources_and_drains(&mut self, changed: bool) {
    //     let (drain_pins, source_pins) = if changed {
    //         (&mut self.row_pins, &mut self.col_pins) // swapped order
    //     } else {
    //         (&mut self.col_pins, &mut self.row_pins) // original order
    //     };
    //         ;
    //     for drain in drain_pins.iter_mut().enumerate() {
    //         drain.set_mode(PinMode::OutputOpenDrain);
    //         drain.digital_write(true);
    //     }
    //     for source in source_pins.iter_mut().enumerate() {
    //         source.set_mode(PinMode::InputPullup);
    //     }
    //
    // }

    /// Return None if nothing is pressed. Keycode is None if that value is not in matrix
    pub fn scan_key_press(&mut self) -> Option<Vec<Option<u32>, MatrixCap>> {
        //let mut v: Vec<ScanRowState, MatrixCap> = full_vec(ScanRowState::NotPressed, self.row_pins.len());
        let mut keys: Vec<Option<u32>, MatrixCap> = Vec::new();
        for (col, drain) in self.col_pins.iter_mut().enumerate() {
            drain.digital_write(false);  // enable drain
            for (row, source) in self.row_pins.iter().enumerate() {
                let pressed = !source.digital_read();  // check if connected
                if pressed {
                    keys.push(self.code_matrix[row][col]).unwrap_or(());
                }
            }
            drain.digital_write(true);  // disable drain
            delay(1); // It takes time for pullup pin to charge back to full voltage
        }
        return if keys.len() > 0 {
            Some(keys)
        } else {
            None
        };
    }
}

//     /// Return None if nothing is pressed. Keycode is None if that value is not in matrix
//     pub fn scan_key_press_too_fucking_complicated(&mut self) -> Option<Vec<Key, MatrixCap>> {
//         // Row scan
//         let mut m: Vec<Vec<Key, MatrixCap>, MatrixCap> = full_vec(
//             full_vec(Key::Free, self.col_pins.len()),
//             self.row_pins.len(),
//         );
//         #[derive(Copy, Clone)]
//         enum SourcePin {
//             Free,
//             Activated((usize, usize,)),
//             OverLap(u16)
//         }
//         use SourcePin::Free;
//         // Index to drain or col that has been arleady activated
//         let mut rows_activated: Vec<SourcePin, MatrixCap> = full_vec(Free, self.row_pins.len());
//         let mut cols_activated: Vec<SourcePin, MatrixCap> = full_vec(Free, self.col_pins.len());
//         let mut keys: Vec<Key, MatrixCap> = Vec::new();
//
//
//         for iter in 0..2 {
//             let swapped = iter == 1;
//             let (drain_pins, source_pins) = if swapped {
//                 (&mut self.row_pins, &mut self.col_pins) // swapped order
//             } else {
//                 (&mut self.col_pins, &mut self.row_pins) // original order
//             };
//             let (drains_activated, sources_activated) = if swapped {
//                 (&mut rows_activated, &mut cols_activated) // swapped order
//             } else {
//                 (&mut cols_activated, &mut rows_activated) // original order
//             };
//
//             for (mut i_d, drain) in drain_pins.iter_mut().enumerate() {
//                 drain.digital_write(false);  // enable drain
//                 for (mut i_s, source) in source_pins.iter().enumerate() {
//                     let pressed = !source.digital_read();  // check if connected
//                     let (row, col) = if swapped { (i_s, i_d) } else {(i_d, i_s)};
//
//                     if pressed {
//                         let code = self.code_matrix[row][col];
//                         // Test that if key scan row already been activated
//
//                         match m[row][col] {
//                             Key::Free => {
//                                 assert!(!swapped);
//                                 // TODO borrow checker
//                                 let source_state = sources_activated[i_s];
//                                 match source_state {
//                                     SourcePin::Free => {
//                                         // Ok, everything fine so far
//                                         m[row][col] = Key::Pressed(code);
//                                         sources_activated[i_s] = SourcePin::Activated((row, col));
//                                     },
//                                     SourcePin::Activated((prev_row, prev_col,)) => {
//                                         // Uh oh! Corresponding drain was already activated before!
//                                         sources_activated[i_s] = SourcePin::OverLap(2);
//                                         m[prev_row][prev_col] = Key::Overlap;
//                                         m[row][col] = Key::Overlap;
//                                     },
//                                     SourcePin::OverLap(count) => {
//                                         sources_activated[i_s] = SourcePin::OverLap(count + 1)
//                                     },
//                                 }
//                             },
//                             Key::Pressed(_) => {
//                                 assert!(swapped);
//                             },
//                             Key::Overlap => {
//                                 assert!(swapped);
//                                 if Some drains_activated[i_d]
//                                 if let Some((prev_row, prev_col)) = sources_activated[i_s] {
//                                     // Uh oh! Corresponding drain was already activated before!
//                                     m[prev_row][prev_col] = Key::Overlap;
//                                     m[row][col] = Key::Overlap;
//                                 } else {
//                                     // Ok, everything fine so far
//                                     m[row][col] = Key::Pressed(code);
//                                 }
//                             },
//                         }
//                     }
//                 }
//                 drain.digital_write(true);  // disable drain
//                 delay(1); // It takes time for pullup pin to charge back to full voltage
//             }
//             self.swap_sources_and_drains(swapped);
//         }
//         if v.iter().all(|state| *state == Key::Free) {
//             return None;
//         } else {
//             return Some(v);
//         }
//     }
// }
