use heapless::Vec;
use crate::record_keyboard_matrix::figure_out_key_matrix;
use crate::process_keys::KeyMatrix;
use teensy3::{bindings as b, pins::PinRow};

// pub const KEY_NAMES_SHORT_TEST: &[&[&str]] = &[
//     &["b::KEY_ENTER", "b::KEY_SPACE"],
//     &["b::KEY_Q", "b::KEY_W", "b::KEY_R"],
//     &["b::KEY_A", "b::KEY_S", "b::KEY_D"],
// ];
//
// pub const KEY_CODES_SHORT_TEST: &[&[u32]] = &[
//     &[b::KEY_ENTER, b::KEY_SPACE],
//     &[b::KEY_Q, b::KEY_W, b::KEY_R],
//     &[b::KEY_A, b::KEY_S, b::KEY_D],
// ];

pub const MODIFIERKEY_FN:u32 = 0x8f;

/// This represents spatial configuration of my keyboard, row by row.
pub const KEY_CODES: &[&[u32]] = &[
    // Special keys
    &[b::KEY_BACKSPACE, b::KEY_DELETE],
    &[b::KEY_ESC, b::KEY_F1, b::KEY_F2, b::KEY_F3, b::KEY_F4, b::KEY_F5, b::KEY_F6, b::KEY_F7,
        b::KEY_F8, b::KEY_F9, b::KEY_F10, b::KEY_F11, b::KEY_F12, b::KEY_HOME, b::KEY_END,
        b::KEY_INSERT],
    &[b::KEY_TILDE, b::KEY_1, b::KEY_2, b::KEY_3, b::KEY_4, b::KEY_5, b::KEY_6,
        b::KEY_7, b::KEY_8, b::KEY_9, b::KEY_0, b::KEY_MINUS, b::KEY_EQUAL],
    &[b::KEY_TAB, b::KEY_Q, b::KEY_W, b::KEY_E, b::KEY_R, b::KEY_T, b::KEY_Y, b::KEY_U,
        b::KEY_I, b::KEY_O, b::KEY_P, b::KEY_LEFT_BRACE, b::KEY_RIGHT_BRACE, b::KEY_ENTER],
    &[b::KEY_CAPS_LOCK, b::KEY_A, b::KEY_S, b::KEY_D, b::KEY_F, b::KEY_G, b::KEY_H,
        b::KEY_J, b::KEY_K, b::KEY_L, b::KEY_SEMICOLON, b::KEY_QUOTE, b::KEY_BACKSLASH],
    &[b::MODIFIERKEY_LEFT_SHIFT, b::KEY_NON_US_BS, b::KEY_Z, b::KEY_X, b::KEY_C, b::KEY_V,
        b::KEY_B, b::KEY_N, b::KEY_M, b::KEY_COMMA, b::KEY_PERIOD, b::KEY_SLASH,
        b::MODIFIERKEY_LEFT_SHIFT],
    &[b::MODIFIERKEY_LEFT_CTRL, MODIFIERKEY_FN, b::MODIFIERKEY_LEFT_GUI, b::MODIFIERKEY_LEFT_ALT,
        b::KEY_SPACE, b::MODIFIERKEY_RIGHT_ALT, b::KEY_PRINTSCREEN, b::MODIFIERKEY_RIGHT_CTRL,
        b::KEY_PAGE_UP, b::KEY_UP, b::KEY_PAGE_DOWN, b::KEY_LEFT, b::KEY_DOWN, b::KEY_RIGHT],
];

pub const KEY_NAMES: &[&[&str]] = &[
    &["b::KEY_BACKSPACE", "b::KEY_DELETE"],
    &["b::KEY_ESC", "b::KEY_F1", "b::KEY_F2", "b::KEY_F3", "b::KEY_F4", "b::KEY_F5", "b::KEY_F6",
        "b::KEY_F7", "b::KEY_F8", "b::KEY_F9", "b::KEY_F10", "b::KEY_F11", "b::KEY_F12",
        "b::KEY_HOME", "b::KEY_END", "b::KEY_INSERT"],
    &["b::KEY_TILDE", "b::KEY_1", "b::KEY_2", "b::KEY_3", "b::KEY_4", "b::KEY_5", "b::KEY_6",
        "b::KEY_7", "b::KEY_8", "b::KEY_9", "b::KEY_0", "b::KEY_MINUS", "b::KEY_EQUAL"],
    &["b::KEY_TAB", "b::KEY_Q", "b::KEY_W", "b::KEY_E", "b::KEY_R", "b::KEY_T", "b::KEY_Y",
        "b::KEY_U", "b::KEY_I", "b::KEY_O", "b::KEY_P", "b::KEY_LEFT_BRACE", "b::KEY_RIGHT_BRACE",
        "b::KEY_ENTER"],
    &["b::KEY_CAPS_LOCK", "b::KEY_A", "b::KEY_S", "b::KEY_D", "b::KEY_F", "b::KEY_G", "b::KEY_H",
        "b::KEY_J", "b::KEY_K", "b::KEY_L", "b::KEY_SEMICOLON", "b::KEY_QUOTE", "b::KEY_BACKSLASH"],
    &["b::MODIFIERKEY_LEFT_SHIFT", "b::KEY_NON_US_BS", "b::KEY_Z", "b::KEY_X", "b::KEY_C",
        "b::KEY_V", "b::KEY_B", "b::KEY_N", "b::KEY_M", "b::KEY_COMMA", "b::KEY_PERIOD",
        "b::KEY_SLASH", "b::MODIFIERKEY_LEFT_SHIFT"],
    &["b::MODIFIERKEY_LEFT_CTRL", "MODIFIERKEY_FN", "b::MODIFIERKEY_LEFT_GUI",
        "b::MODIFIERKEY_LEFT_ALT", "b::KEY_SPACE", "b::MODIFIERKEY_RIGHT_ALT", "b::KEY_PRINTSCREEN",
        "b::MODIFIERKEY_RIGHT_CTRL", "b::KEY_PAGE_UP", "b::KEY_UP", "b::KEY_PAGE_DOWN",
        "b::KEY_LEFT", "b::KEY_DOWN", "b::KEY_RIGHT"]
];

/// Use this function only the first time when key presses are recorded. Then copy paste the code
/// output to `get_stored_key_codes`.
#[allow(dead_code)]
pub fn ask_key_codes_and_print_them(pinrow: &mut PinRow) -> KeyMatrix {
    let mat = figure_out_key_matrix(
        pinrow, KEY_CODES, KEY_NAMES
        //pinrow, KEY_CODES_SHORT_TEST, KEY_NAMES_SHORT_TEST
    );
    return mat
}

/// This function contains key codes that are generated with `ask_key_codes_and_print_them`
pub fn get_stored_key_codes(pinrow: &mut PinRow) -> KeyMatrix {
    let code_matrix = [
        [                 0,        0,                       0,              0,                0, b::MODIFIERKEY_LEFT_SHIFT, b::MODIFIERKEY_LEFT_SHIFT,                        0,                        0, ],
        [          b::KEY_7, b::KEY_U,                b::KEY_H,       b::KEY_6,         b::KEY_J,                  b::KEY_M,                  b::KEY_Y,                        0,                 b::KEY_N, ],
        [          b::KEY_4, b::KEY_R,                b::KEY_G,       b::KEY_5,         b::KEY_F,                  b::KEY_V,                  b::KEY_T,                        0,                 b::KEY_B, ],
        [          b::KEY_0, b::KEY_P,            b::KEY_QUOTE,   b::KEY_MINUS, b::KEY_SEMICOLON,          b::KEY_BACKSLASH,         b::KEY_LEFT_BRACE,                        0,             b::KEY_SLASH, ],
        [                 0,        0,                       0, MODIFIERKEY_FN,                0, b::MODIFIERKEY_RIGHT_CTRL,                         0,                        0,                        0, ],
        [        b::KEY_F12,        0,                       0,  b::KEY_INSERT,                0,                         0,   b::MODIFIERKEY_LEFT_GUI,                        0,             b::KEY_RIGHT, ],
        [        b::KEY_F11,        0,                       0,  b::KEY_DELETE,                0,                         0,                         0,                        0,              b::KEY_DOWN, ],
        [        b::KEY_END,        0,               b::KEY_UP,    b::KEY_HOME,                0,                         0,                         0,                        0,              b::KEY_LEFT, ],
        [  b::KEY_PAGE_DOWN,        0,                       0, b::KEY_PAGE_UP,                0,                         0,                         0,                        0,                        0, ],
        [        b::KEY_F10,        0,               b::KEY_F5,      b::KEY_F9,                0,              b::KEY_ENTER,          b::KEY_BACKSPACE,                        0,             b::KEY_SPACE, ],
        [b::KEY_PRINTSCREEN,        0, b::MODIFIERKEY_LEFT_ALT,              0,                0,                         0,                         0,                        0, b::MODIFIERKEY_RIGHT_ALT, ],
        [          b::KEY_8, b::KEY_I,               b::KEY_F6,   b::KEY_EQUAL,         b::KEY_K,              b::KEY_COMMA,        b::KEY_RIGHT_BRACE,                        0,                        0, ],
        [          b::KEY_9, b::KEY_O,                       0,      b::KEY_F8,         b::KEY_L,             b::KEY_PERIOD,                 b::KEY_F7,                        0,                        0, ],
        [          b::KEY_2, b::KEY_W,        b::KEY_NON_US_BS,      b::KEY_F1,         b::KEY_S,                  b::KEY_X,          b::KEY_CAPS_LOCK,                        0,                        0, ],
        [          b::KEY_3, b::KEY_E,               b::KEY_F4,      b::KEY_F2,         b::KEY_D,                  b::KEY_C,                 b::KEY_F3,                        0,                        0, ],
        [          b::KEY_1, b::KEY_Q,              b::KEY_ESC,   b::KEY_TILDE,         b::KEY_A,                  b::KEY_Z,                b::KEY_TAB,                        0,                        0, ],
        [                 0,        0,                       0,              0,                0,                         0,                         0, b::MODIFIERKEY_LEFT_CTRL,                        0, ],

    ].iter()
        .map(|v| v.iter().map(|&k| if k==0 { None } else { Some(k) }).collect())
        .collect();
    let rows = Vec::from_slice(&[0, 3, 4, 6, 7, 8, 9, 10, 11, 12, 14, 15, 16, 17, 18, 20, 38]).unwrap();
    let cols = Vec::from_slice(&[1, 2, 5, 19, 21, 22, 23, 27, 40]).unwrap();
    let mat = KeyMatrix::new(pinrow, code_matrix, rows, cols);

    return mat;
}
