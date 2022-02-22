//! This file contains custom key layout configuration of my keyboard.
//! This is also good place to see how key matrix recording is done in practise.

use crate::process_keys::{ExtraKeyInfo, KeyMatrix};
use crate::record_keyboard_matrix::figure_out_key_matrix;
use crate::ShortVec;
use heapless::Vec;
use teensy3::{bindings as b, pins::PinRow};

const MODIFIERKEY_FN: u32 = 0xE800;

/// This represents spatial configuration of my keyboard, row by row.
const KEY_CODES: &[&[u32]] = &[
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
        b::MODIFIERKEY_RIGHT_SHIFT],
    &[b::MODIFIERKEY_LEFT_CTRL, MODIFIERKEY_FN, b::MODIFIERKEY_LEFT_GUI, b::MODIFIERKEY_LEFT_ALT,
        b::KEY_SPACE, b::MODIFIERKEY_RIGHT_ALT, b::KEY_PRINTSCREEN, b::MODIFIERKEY_RIGHT_CTRL,
        b::KEY_PAGE_UP, b::KEY_UP, b::KEY_PAGE_DOWN, b::KEY_LEFT, b::KEY_DOWN, b::KEY_RIGHT],
];

const KEY_NAMES: &[&[&str]] = &[
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
        "b::KEY_SLASH", "b::MODIFIERKEY_RIGHT_SHIFT"],
    &["b::MODIFIERKEY_LEFT_CTRL", "MODIFIERKEY_FN", "b::MODIFIERKEY_LEFT_GUI",
        "b::MODIFIERKEY_LEFT_ALT", "b::KEY_SPACE", "b::MODIFIERKEY_RIGHT_ALT", "b::KEY_PRINTSCREEN",
        "b::MODIFIERKEY_RIGHT_CTRL", "b::KEY_PAGE_UP", "b::KEY_UP", "b::KEY_PAGE_DOWN",
        "b::KEY_LEFT", "b::KEY_DOWN", "b::KEY_RIGHT"]
];

/// Use this function only the first time when key presses are recorded. Then copy paste the code
/// output to `get_stored_key_codes`.
#[allow(dead_code)]
pub fn ask_key_codes_and_print_them(pinrow: &mut PinRow) -> KeyMatrix {
    let info = extra_information_about_key_codes();
    let mat = figure_out_key_matrix(pinrow, KEY_CODES, KEY_NAMES, info);
    return mat;
}

/// This function contains key codes that are generated with `ask_key_codes_and_print_them`
pub fn get_stored_key_codes(pinrow: &mut PinRow) -> KeyMatrix {
    let info = extra_information_about_key_codes();
    // autogenerated with `ask_key_codes_and_print_them`
    let code_matrix = [
        [                       0,                       0,        0,                       0,              0,                0, b::MODIFIERKEY_RIGHT_SHIFT, b::MODIFIERKEY_LEFT_SHIFT,                        0, ],
        [                b::KEY_N,                b::KEY_7, b::KEY_U,                b::KEY_H,       b::KEY_6,         b::KEY_J,                   b::KEY_M,                  b::KEY_Y,                        0, ],
        [                b::KEY_B,                b::KEY_4, b::KEY_R,                b::KEY_G,       b::KEY_5,         b::KEY_F,                   b::KEY_V,                  b::KEY_T,                        0, ],
        [            b::KEY_SLASH,                b::KEY_0, b::KEY_P,            b::KEY_QUOTE,   b::KEY_MINUS, b::KEY_SEMICOLON,           b::KEY_BACKSLASH,         b::KEY_LEFT_BRACE,                        0, ],
        [                       0,                       0,        0,                       0, MODIFIERKEY_FN,                0,  b::MODIFIERKEY_RIGHT_CTRL,                         0,                        0, ],
        [            b::KEY_RIGHT,              b::KEY_F12,        0,                       0,  b::KEY_INSERT,                0,                          0,  b::MODIFIERKEY_RIGHT_ALT,                        0, ],
        [b::MODIFIERKEY_RIGHT_ALT, b::MODIFIERKEY_LEFT_GUI,        0, b::MODIFIERKEY_LEFT_ALT,              0,                0,                          0,                         0,                        0, ],
        [             b::KEY_DOWN,              b::KEY_F11,        0,                       0,  b::KEY_DELETE,                0,                          0,                         0,                        0, ],
        [             b::KEY_LEFT,              b::KEY_END,        0,               b::KEY_UP,    b::KEY_HOME,                0,                          0,                         0,                        0, ],
        [                       0,                b::KEY_8, b::KEY_I,               b::KEY_F6,   b::KEY_EQUAL,         b::KEY_K,               b::KEY_COMMA,        b::KEY_RIGHT_BRACE,                        0, ],
        [                       0,                b::KEY_9, b::KEY_O,                       0,      b::KEY_F8,         b::KEY_L,              b::KEY_PERIOD,                 b::KEY_F7,                        0, ],
        [                       0,                b::KEY_2, b::KEY_W,        b::KEY_NON_US_BS,      b::KEY_F1,         b::KEY_S,                   b::KEY_X,          b::KEY_CAPS_LOCK,                        0, ],
        [                       0,                b::KEY_3, b::KEY_E,               b::KEY_F4,      b::KEY_F2,         b::KEY_D,                   b::KEY_C,                 b::KEY_F3,                        0, ],
        [                       0,                b::KEY_1, b::KEY_Q,              b::KEY_ESC,   b::KEY_TILDE,         b::KEY_A,                   b::KEY_Z,                b::KEY_TAB,                        0, ],
        [                       0,              b::KEY_END,        0,                       0,    b::KEY_HOME,                0,                          0,                         0,                        0, ],
        [            b::KEY_SPACE,              b::KEY_F10,        0,               b::KEY_F5,      b::KEY_F9,                0,               b::KEY_ENTER,          b::KEY_BACKSPACE,                        0, ],
        [                       0,                       0,        0,                       0,              0,                0,                          0,                         0, b::MODIFIERKEY_LEFT_CTRL, ],
    ].iter()
        .map(|v| v.iter().map(|&k| if k==0 { None } else { Some(k) }).collect())
        .collect();
    let rows = Vec::from_slice(&[1, 5, 6, 7, 8, 9, 10, 11, 12, 14, 15, 16, 17, 19, 24, 25, 37]).unwrap();
    let cols = Vec::from_slice(&[0, 2, 3, 4, 18, 20, 21, 22, 28]).unwrap();
    let mat = KeyMatrix::new(pinrow, code_matrix, rows, cols, info);

    return mat;
}


/// This function is my custom configuration, for some small details about key codes.
/// This contains information about Fn key, media keys, and the byte masks of key codes.
/// The only thing that should need configuration is `media_key_bindings`. All others
/// are effectively the same for everybody.
pub fn extra_information_about_key_codes() -> ExtraKeyInfo {

    // Media key bindings when fn is pressed. That is, if "Fn + F2" is pressed, then volume
    // is decreased. This is the only thing one should need to configure in this function!
    let media_key_bindings: ShortVec<(u32, u32)> = [
        (b::KEY_F1, b::KEY_MEDIA_MUTE),
        (b::KEY_F2, b::KEY_MEDIA_VOLUME_DEC),
        (b::KEY_F3, b::KEY_MEDIA_VOLUME_INC),
    ].iter().copied().collect();

    // Fn key code is chosen here to be similar to media key masks in core/teensy3/keylayouts.h
    // Whatever you choose Fn-key to be, make sure that its second byte differs from regular keys or
    // modifier keys. Here that second byte of Fn is "0xE8", which differs from "0xF0" and "0xE0",
    // which are defined below.
    let fn_key: u32 = MODIFIERKEY_FN;

    // Key codes are defined in `core/teensy3/keylayouts.h` like following:
    //     KEY_A             =    4 | 0xF000
    //     KEY_B             =    5 | 0xF000
    //     ...
    //     MODIFIERKEY_CTRL  = 0x01 | 0xE000
    //     MODIFIERKEY_SHIFT = 0x02 | 0xE000
    //     ...
    // Here regular keys are separated from modifier with hexadecimal mask in the second byte.
    let regular_key_mask: u8 = 0xF0;
    let modifier_key_mask: u8 = 0xE0;
    // Fn key mask must be different to regular keys and modifiers
    let fn_key_mask = fn_key.to_le_bytes()[1];
    assert!((fn_key_mask != regular_key_mask) && (fn_key_mask != modifier_key_mask));

    return ExtraKeyInfo{fn_key, media_key_bindings, regular_key_mask, modifier_key_mask};
}

/*
For no specific reason, here's packed up version of my key matrix above:

          0     0         0       0         0 RIGHT_SHIFT _LEFT_SHIFT         0         0
   b::KEY_7 KEY_U  b::KEY_H  :KEY_6  b::KEY_J    b::KEY_M    b::KEY_Y         0  b::KEY_N
   b::KEY_4 KEY_R  b::KEY_G  :KEY_5  b::KEY_F    b::KEY_V    b::KEY_T         0  b::KEY_B
   b::KEY_0 KEY_P KEY_QUOTE  _MINUS SEMICOLON  _BACKSLASH _LEFT_BRACE         0 KEY_SLASH
          0     0         0  KEY_FN         0  RIGHT_CTRL           0         0         0
  ::KEY_F12     0         0  INSERT         0           0 EY_LEFT_GUI         0 KEY_RIGHT
  ::KEY_F11     0         0  DELETE         0           0           0         0 :KEY_DOWN
  ::KEY_END     0  ::KEY_UP  Y_HOME         0           0           0         0 :KEY_LEFT
  PAGE_DOWN     0         0 PAGE_UP         0           0           0         0         0
  ::KEY_F10     0  ::KEY_F5  KEY_F9         0  :KEY_ENTER Y_BACKSPACE         0 KEY_SPACE
PRINTSCREEN     0  LEFT_ALT       0         0           0           0         0 RIGHT_ALT
   b::KEY_8 KEY_I  ::KEY_F6  _EQUAL  b::KEY_K  :KEY_COMMA RIGHT_BRACE         0         0
   b::KEY_9 KEY_O         0  KEY_F8  b::KEY_L  KEY_PERIOD   b::KEY_F7         0         0
   b::KEY_2 KEY_W NON_US_BS  KEY_F1  b::KEY_S    b::KEY_X Y_CAPS_LOCK         0         0
   b::KEY_3 KEY_E  ::KEY_F4  KEY_F2  b::KEY_D    b::KEY_C   b::KEY_F3         0         0
   b::KEY_1 KEY_Q  :KEY_ESC  _TILDE  b::KEY_A    b::KEY_Z  b::KEY_TAB         0         0
          0     0         0       0         0           0           0 LEFT_CTRL         0

*/



