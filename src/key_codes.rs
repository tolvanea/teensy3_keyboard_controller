use teensy3::bindings as b;

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
//];

pub const MODIFIERKEY_FN:u32 = 0x8f;

pub const KEY_CODES: &[&[u32]] = &[
    // Special keys
    &[b::KEY_ENTER, b::KEY_SPACE],
    &[b::KEY_ESC, b::KEY_F1, b::KEY_F2, b::KEY_F3, b::KEY_F4, b::KEY_F5, b::KEY_F6, b::KEY_F7,
        b::KEY_F8, b::KEY_F9, b::KEY_F10, b::KEY_F11, b::KEY_F12, b::KEY_HOME, b::KEY_END,
        b::KEY_INSERT, b::KEY_DELETE],
    &[b::KEY_TILDE, b::KEY_1, b::KEY_2, b::KEY_3, b::KEY_4, b::KEY_5, b::KEY_6,
        b::KEY_7, b::KEY_8, b::KEY_9, b::KEY_0, b::KEY_MINUS, b::KEY_EQUAL, b::KEY_BACKSPACE],
    &[b::KEY_TAB, b::KEY_Q, b::KEY_W, b::KEY_E, b::KEY_R, b::KEY_T, b::KEY_Y, b::KEY_U,
        b::KEY_I, b::KEY_O, b::KEY_P, b::KEY_LEFT_BRACE, b::KEY_RIGHT_BRACE],
    &[b::KEY_CAPS_LOCK, b::KEY_A, b::KEY_S, b::KEY_D, b::KEY_F, b::KEY_G, b::KEY_H,
        b::KEY_J, b::KEY_K, b::KEY_L, b::KEY_SEMICOLON, b::KEY_QUOTE, b::KEY_BACKSLASH],
    &[b::MODIFIERKEY_LEFT_SHIFT, b::KEY_NON_US_BS, b::KEY_Z, b::KEY_X, b::KEY_C, b::KEY_V,
        b::KEY_B, b::KEY_N, b::KEY_M, b::KEY_COMMA, b::KEY_PERIOD, b::KEY_SLASH,
        b::MODIFIERKEY_LEFT_SHIFT],
    &[b::MODIFIERKEY_LEFT_CTRL, MODIFIERKEY_FN, b::MODIFIERKEY_LEFT_GUI, b::MODIFIERKEY_LEFT_ALT,
        b::MODIFIERKEY_RIGHT_ALT, b::KEY_PRINTSCREEN, b::MODIFIERKEY_RIGHT_CTRL,
        b::KEY_PAGE_UP, b::KEY_UP, b::KEY_PAGE_DOWN, b::KEY_LEFT, b::KEY_DOWN, b::KEY_RIGHT],
];

pub const KEY_NAMES: &[&[&str]] = &[
    &["b::KEY_ENTER", "b::KEY_SPACE"],
    &["b::KEY_ESC", "b::KEY_F1", "b::KEY_F2", "b::KEY_F3", "b::KEY_F4", "b::KEY_F5", "b::KEY_F6",
        "b::KEY_F7", "b::KEY_F8", "b::KEY_F9", "b::KEY_F10", "b::KEY_F11", "b::KEY_F12",
        "b::KEY_HOME", "b::KEY_END", "b::KEY_INSERT", "b::KEY_DELETE"],
    &["b::KEY_TILDE", "b::KEY_1", "b::KEY_2", "b::KEY_3", "b::KEY_4", "b::KEY_5", "b::KEY_6",
        "b::KEY_7", "b::KEY_8", "b::KEY_9", "b::KEY_0", "b::KEY_MINUS", "b::KEY_EQUAL",
        "b::KEY_BACKSPACE"],
    &["b::KEY_TAB", "b::KEY_Q", "b::KEY_W", "b::KEY_E", "b::KEY_R", "b::KEY_T", "b::KEY_Y",
        "b::KEY_U", "b::KEY_I", "b::KEY_O", "b::KEY_P", "b::KEY_LEFT_BRACE", "b::KEY_RIGHT_BRACE"],
    &["b::KEY_CAPS_LOCK", "b::KEY_A", "b::KEY_S", "b::KEY_D", "b::KEY_F", "b::KEY_G", "b::KEY_H",
        "b::KEY_J", "b::KEY_K", "b::KEY_L", "b::KEY_SEMICOLON", "b::KEY_QUOTE", "b::KEY_BACKSLASH"],
    &["b::MODIFIERKEY_LEFT_SHIFT", "b::KEY_NON_US_BS", "b::KEY_Z", "b::KEY_X", "b::KEY_C",
        "b::KEY_V", "b::KEY_B", "b::KEY_N", "b::KEY_M", "b::KEY_COMMA", "b::KEY_PERIOD",
        "b::KEY_SLASH", "b::MODIFIERKEY_LEFT_SHIFT"],
    &["b::MODIFIERKEY_LEFT_CTRL", "MODIFIERKEY_FN", "b::MODIFIERKEY_LEFT_GUI",
        "b::MODIFIERKEY_LEFT_ALT", "b::MODIFIERKEY_RIGHT_ALT", "b::KEY_PRINTSCREEN",
        "b::MODIFIERKEY_RIGHT_CTRL", "b::KEY_PAGE_UP", "b::KEY_UP", "b::KEY_PAGE_DOWN",
        "b::KEY_LEFT", "b::KEY_DOWN", "b::KEY_RIGHT"]
];
