//! Keebart's stock keymap for the Sofle Choc Pro BT, as the keyboard reports it.
//!
//! Generated from `config/sofle_choc_pro.keymap` in
//! [Keebart/zmk-config](https://github.com/Keebart/zmk-config) at `98c5a11` (MIT, © The ZMK
//! Contributors). Its macros were expanded with ZMK v0.3.0's headers, so the parameters are
//! exactly what the firmware stores:
//!
//! ```sh
//! clang -E -P -x c -undef -nostdinc -I zmk/app/include sofle_choc_pro.keymap
//! ```
//!
//! (with the `#include <behaviors.dtsi>` line removed, since only the bindings matter). Each
//! layer has 60 bindings, for 58 keys and 2 knob presses. Each binding is (behavior display
//! name, param1, param2), followed by its source.

/// A binding: the behavior's display name, then its two parameters.
pub type StockBinding = (&'static str, u32, u32);

/// A layer: its name, then its bindings by key position.
pub type StockLayer = (&'static str, &'static [StockBinding]);

/// The layers, in order.
pub const LAYERS: [StockLayer; 4] = [
    (
        "default",
        &[
            ("Key Press", 0x0007_0035, 0),       // &kp GRAVE
            ("Key Press", 0x0007_001e, 0),       // &kp N1
            ("Key Press", 0x0007_001f, 0),       // &kp N2
            ("Key Press", 0x0007_0020, 0),       // &kp N3
            ("Key Press", 0x0007_0021, 0),       // &kp N4
            ("Key Press", 0x0007_0022, 0),       // &kp N5
            ("Key Press", 0x0007_0023, 0),       // &kp N6
            ("Key Press", 0x0007_0024, 0),       // &kp N7
            ("Key Press", 0x0007_0025, 0),       // &kp N8
            ("Key Press", 0x0007_0026, 0),       // &kp N9
            ("Key Press", 0x0007_0027, 0),       // &kp N0
            ("Key Press", 0x0007_0035, 0),       // &kp GRAVE
            ("Key Press", 0x0007_0029, 0),       // &kp ESC
            ("Key Press", 0x0007_0014, 0),       // &kp Q
            ("Key Press", 0x0007_001a, 0),       // &kp W
            ("Key Press", 0x0007_0008, 0),       // &kp E
            ("Key Press", 0x0007_0015, 0),       // &kp R
            ("Key Press", 0x0007_0017, 0),       // &kp T
            ("Key Press", 0x0007_001c, 0),       // &kp Y
            ("Key Press", 0x0007_0018, 0),       // &kp U
            ("Key Press", 0x0007_000c, 0),       // &kp I
            ("Key Press", 0x0007_0012, 0),       // &kp O
            ("Key Press", 0x0007_0013, 0),       // &kp P
            ("Key Press", 0x0007_002a, 0),       // &kp BSPC
            ("Key Press", 0x0007_002b, 0),       // &kp TAB
            ("Key Press", 0x0007_0004, 0),       // &kp A
            ("Key Press", 0x0007_0016, 0),       // &kp S
            ("Key Press", 0x0007_0007, 0),       // &kp D
            ("Key Press", 0x0007_0009, 0),       // &kp F
            ("Key Press", 0x0007_000a, 0),       // &kp G
            ("Key Press", 0x0007_000b, 0),       // &kp H
            ("Key Press", 0x0007_000d, 0),       // &kp J
            ("Key Press", 0x0007_000e, 0),       // &kp K
            ("Key Press", 0x0007_000f, 0),       // &kp L
            ("Key Press", 0x0007_0033, 0),       // &kp SEMI
            ("Key Press", 0x0007_0034, 0),       // &kp SQT
            ("Key Press", 0x0007_00e1, 0),       // &kp LSHFT
            ("Key Press", 0x0007_001d, 0),       // &kp Z
            ("Key Press", 0x0007_001b, 0),       // &kp X
            ("Key Press", 0x0007_0006, 0),       // &kp C
            ("Key Press", 0x0007_0019, 0),       // &kp V
            ("Key Press", 0x0007_0005, 0),       // &kp B
            ("Key Press", 0x000c_00e2, 0),       // &kp C_MUTE
            ("Key Press", 0x000c_00b0, 0),       // &kp C_PLAY
            ("Key Press", 0x0007_0011, 0),       // &kp N
            ("Key Press", 0x0007_0010, 0),       // &kp M
            ("Key Press", 0x0007_0036, 0),       // &kp COMMA
            ("Key Press", 0x0007_0037, 0),       // &kp DOT
            ("Key Press", 0x0007_0038, 0),       // &kp FSLH
            ("Key Press", 0x0007_00e5, 0),       // &kp RSHFT
            ("Key Press", 0x0007_00e3, 0),       // &kp LGUI
            ("Key Press", 0x0007_00e2, 0),       // &kp LALT
            ("Key Press", 0x0007_00e0, 0),       // &kp LCTRL
            ("Momentary Layer", 0x0000_0001, 0), // &mo LOWER
            ("Key Press", 0x0007_0028, 0),       // &kp RET
            ("Key Press", 0x0007_002c, 0),       // &kp SPACE
            ("Momentary Layer", 0x0000_0002, 0), // &mo RAISE
            ("Key Press", 0x0007_00e4, 0),       // &kp RCTRL
            ("Key Press", 0x0007_00e6, 0),       // &kp RALT
            ("Key Press", 0x0007_00e7, 0),       // &kp RGUI
        ],
    ),
    (
        "lower",
        &[
            ("Transparent", 0x0000_0000, 0), // &trans
            ("Key Press", 0x0007_003a, 0),   // &kp F1
            ("Key Press", 0x0007_003b, 0),   // &kp F2
            ("Key Press", 0x0007_003c, 0),   // &kp F3
            ("Key Press", 0x0007_003d, 0),   // &kp F4
            ("Key Press", 0x0007_003e, 0),   // &kp F5
            ("Key Press", 0x0007_003f, 0),   // &kp F6
            ("Key Press", 0x0007_0040, 0),   // &kp F7
            ("Key Press", 0x0007_0041, 0),   // &kp F8
            ("Key Press", 0x0007_0042, 0),   // &kp F9
            ("Key Press", 0x0007_0043, 0),   // &kp F10
            ("Key Press", 0x0007_0044, 0),   // &kp F11
            ("Key Press", 0x0007_0035, 0),   // &kp GRAVE
            ("Key Press", 0x0007_001e, 0),   // &kp N1
            ("Key Press", 0x0007_001f, 0),   // &kp N2
            ("Key Press", 0x0007_0020, 0),   // &kp N3
            ("Key Press", 0x0007_0021, 0),   // &kp N4
            ("Key Press", 0x0007_0022, 0),   // &kp N5
            ("Key Press", 0x0007_0023, 0),   // &kp N6
            ("Key Press", 0x0007_0024, 0),   // &kp N7
            ("Key Press", 0x0007_0025, 0),   // &kp N8
            ("Key Press", 0x0007_0026, 0),   // &kp N9
            ("Key Press", 0x0007_0027, 0),   // &kp N0
            ("Key Press", 0x0007_0045, 0),   // &kp F12
            ("Transparent", 0x0000_0000, 0), // &trans
            ("Key Press", 0x0207_001e, 0),   // &kp EXCL
            ("Key Press", 0x0207_001f, 0),   // &kp AT
            ("Key Press", 0x0207_0020, 0),   // &kp HASH
            ("Key Press", 0x0207_0021, 0),   // &kp DLLR
            ("Key Press", 0x0207_0022, 0),   // &kp PRCNT
            ("Key Press", 0x0207_0023, 0),   // &kp CARET
            ("Key Press", 0x0207_0024, 0),   // &kp AMPS
            ("Key Press", 0x0207_0025, 0),   // &kp ASTRK
            ("Key Press", 0x0207_0026, 0),   // &kp LPAR
            ("Key Press", 0x0207_0027, 0),   // &kp RPAR
            ("Key Press", 0x0207_0031, 0),   // &kp PIPE
            ("Transparent", 0x0000_0000, 0), // &trans
            ("Key Press", 0x0007_002e, 0),   // &kp EQUAL
            ("Key Press", 0x0007_002d, 0),   // &kp MINUS
            ("Key Press", 0x0207_002e, 0),   // &kp PLUS
            ("Key Press", 0x0207_002f, 0),   // &kp LBRC
            ("Key Press", 0x0207_0030, 0),   // &kp RBRC
            ("Transparent", 0x0000_0000, 0), // &trans
            ("Transparent", 0x0000_0000, 0), // &trans
            ("Key Press", 0x0007_002f, 0),   // &kp LBKT
            ("Key Press", 0x0007_0030, 0),   // &kp RBKT
            ("Key Press", 0x0007_0033, 0),   // &kp SEMI
            ("Key Press", 0x0207_0033, 0),   // &kp COLON
            ("Key Press", 0x0007_0031, 0),   // &kp BSLH
            ("Transparent", 0x0000_0000, 0), // &trans
            ("Transparent", 0x0000_0000, 0), // &trans
            ("Transparent", 0x0000_0000, 0), // &trans
            ("Transparent", 0x0000_0000, 0), // &trans
            ("Transparent", 0x0000_0000, 0), // &trans
            ("Transparent", 0x0000_0000, 0), // &trans
            ("Transparent", 0x0000_0000, 0), // &trans
            ("Transparent", 0x0000_0000, 0), // &trans
            ("Transparent", 0x0000_0000, 0), // &trans
            ("Transparent", 0x0000_0000, 0), // &trans
            ("Transparent", 0x0000_0000, 0), // &trans
        ],
    ),
    (
        "raise",
        &[
            ("Bluetooth", 0x0000_0000, 0),     // &bt BT_CLR
            ("Bluetooth", 0x0000_0003, 0),     // &bt BT_SEL 0
            ("Bluetooth", 0x0000_0003, 1),     // &bt BT_SEL 1
            ("Bluetooth", 0x0000_0003, 2),     // &bt BT_SEL 2
            ("Bluetooth", 0x0000_0003, 3),     // &bt BT_SEL 3
            ("Bluetooth", 0x0000_0003, 4),     // &bt BT_SEL 4
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Key Press", 0x0007_0049, 0),     // &kp INS
            ("Key Press", 0x0007_0046, 0),     // &kp PSCRN
            ("Key Press", 0x0007_0065, 0),     // &kp K_CMENU
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Key Press", 0x0007_004b, 0),     // &kp PG_UP
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Key Press", 0x0007_0052, 0),     // &kp UP
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Key Press", 0x0007_0027, 0),     // &kp N0
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Key Press", 0x0007_00e2, 0),     // &kp LALT
            ("Key Press", 0x0007_00e0, 0),     // &kp LCTRL
            ("Key Press", 0x0007_00e1, 0),     // &kp LSHFT
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Key Press", 0x0007_0039, 0),     // &kp CLCK
            ("Key Press", 0x0007_004e, 0),     // &kp PG_DN
            ("Key Press", 0x0007_0050, 0),     // &kp LEFT
            ("Key Press", 0x0007_0051, 0),     // &kp DOWN
            ("Key Press", 0x0007_004f, 0),     // &kp RIGHT
            ("Key Press", 0x0007_004c, 0),     // &kp DEL
            ("Key Press", 0x0007_002a, 0),     // &kp BSPC
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Key Press", 0x0007_007a, 0),     // &kp K_UNDO
            ("Key Press", 0x0007_007b, 0),     // &kp K_CUT
            ("Key Press", 0x0007_007c, 0),     // &kp K_COPY
            ("Key Press", 0x0007_007d, 0),     // &kp K_PASTE
            ("Studio Unlock", 0x0000_0000, 0), // &studio_unlock
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
            ("Transparent", 0x0000_0000, 0),   // &trans
        ],
    ),
    (
        "adjust",
        &[
            ("Bluetooth", 0x0000_0000, 0),      // &bt BT_CLR
            ("Bluetooth", 0x0000_0003, 0),      // &bt BT_SEL 0
            ("Bluetooth", 0x0000_0003, 1),      // &bt BT_SEL 1
            ("Bluetooth", 0x0000_0003, 2),      // &bt BT_SEL 2
            ("Bluetooth", 0x0000_0003, 3),      // &bt BT_SEL 3
            ("Bluetooth", 0x0000_0003, 4),      // &bt BT_SEL 4
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("External Power", 0x0000_0002, 0), // &ext_power EP_TOG
            ("Underglow", 0x0000_0004, 0),      // &rgb_ug RGB_HUD
            ("Underglow", 0x0000_0003, 0),      // &rgb_ug RGB_HUI
            ("Underglow", 0x0000_0006, 0),      // &rgb_ug RGB_SAD
            ("Underglow", 0x0000_0005, 0),      // &rgb_ug RGB_SAI
            ("Underglow", 0x0000_000b, 0),      // &rgb_ug RGB_EFF
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("Underglow", 0x0000_0008, 0),      // &rgb_ug RGB_BRD
            ("Underglow", 0x0000_0007, 0),      // &rgb_ug RGB_BRI
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("Underglow", 0x0000_0000, 0),      // &rgb_ug RGB_TOG
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
            ("None", 0x0000_0000, 0),           // &none
        ],
    ),
];
