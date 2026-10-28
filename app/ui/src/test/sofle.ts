// The simulated Sofle's stock keymap, as `session_keymap` gives it, the key
// tables, as `key_catalog` and `dom_codes` do, the Mac actions on macOS 27, as
// `mac_actions` does there, and its guide, as `keyboard_guide` does, generated
// from keywright-app (src/ipc.rs) for the UI's tests and gallery. Don't edit it:
// run `mise run generate:bindings`.
import type { Choice, DomCode, KeyboardGuide, KeyEntry, Keymap } from "../ipc/bindings.ts";

export const sofleKeymap: Keymap = {
  "keys": [
    {
      "x": 0,
      "y": 37,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 1, key 1"
    },
    {
      "x": 100,
      "y": 37,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 1, key 2"
    },
    {
      "x": 200,
      "y": 12,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 1, key 3"
    },
    {
      "x": 300,
      "y": 0,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 1, key 4"
    },
    {
      "x": 400,
      "y": 12,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 1, key 5"
    },
    {
      "x": 500,
      "y": 24,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 1, key 6"
    },
    {
      "x": 900,
      "y": 24,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 1, key 1"
    },
    {
      "x": 1000,
      "y": 12,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 1, key 2"
    },
    {
      "x": 1100,
      "y": 0,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 1, key 3"
    },
    {
      "x": 1200,
      "y": 12,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 1, key 4"
    },
    {
      "x": 1300,
      "y": 37,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 1, key 5"
    },
    {
      "x": 1400,
      "y": 37,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 1, key 6"
    },
    {
      "x": 0,
      "y": 137,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 2, key 1"
    },
    {
      "x": 100,
      "y": 137,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 2, key 2"
    },
    {
      "x": 200,
      "y": 112,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 2, key 3"
    },
    {
      "x": 300,
      "y": 100,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 2, key 4"
    },
    {
      "x": 400,
      "y": 112,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 2, key 5"
    },
    {
      "x": 500,
      "y": 124,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 2, key 6"
    },
    {
      "x": 900,
      "y": 124,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 2, key 1"
    },
    {
      "x": 1000,
      "y": 112,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 2, key 2"
    },
    {
      "x": 1100,
      "y": 100,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 2, key 3"
    },
    {
      "x": 1200,
      "y": 112,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 2, key 4"
    },
    {
      "x": 1300,
      "y": 137,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 2, key 5"
    },
    {
      "x": 1400,
      "y": 137,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 2, key 6"
    },
    {
      "x": 0,
      "y": 237,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 3, key 1"
    },
    {
      "x": 100,
      "y": 237,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 3, key 2"
    },
    {
      "x": 200,
      "y": 212,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 3, key 3"
    },
    {
      "x": 300,
      "y": 200,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 3, key 4"
    },
    {
      "x": 400,
      "y": 212,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 3, key 5"
    },
    {
      "x": 500,
      "y": 224,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 3, key 6"
    },
    {
      "x": 900,
      "y": 224,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 3, key 1"
    },
    {
      "x": 1000,
      "y": 212,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 3, key 2"
    },
    {
      "x": 1100,
      "y": 200,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 3, key 3"
    },
    {
      "x": 1200,
      "y": 212,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 3, key 4"
    },
    {
      "x": 1300,
      "y": 237,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 3, key 5"
    },
    {
      "x": 1400,
      "y": 237,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 3, key 6"
    },
    {
      "x": 0,
      "y": 337,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 4, key 1"
    },
    {
      "x": 100,
      "y": 337,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 4, key 2"
    },
    {
      "x": 200,
      "y": 312,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 4, key 3"
    },
    {
      "x": 300,
      "y": 300,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 4, key 4"
    },
    {
      "x": 400,
      "y": 312,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 4, key 5"
    },
    {
      "x": 500,
      "y": 324,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 4, key 6"
    },
    {
      "x": 600,
      "y": 274,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left knob"
    },
    {
      "x": 800,
      "y": 274,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right knob"
    },
    {
      "x": 900,
      "y": 324,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 4, key 1"
    },
    {
      "x": 1000,
      "y": 312,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 4, key 2"
    },
    {
      "x": 1100,
      "y": 300,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 4, key 3"
    },
    {
      "x": 1200,
      "y": 312,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 4, key 4"
    },
    {
      "x": 1300,
      "y": 337,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 4, key 5"
    },
    {
      "x": 1400,
      "y": 337,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 4, key 6"
    },
    {
      "x": 175,
      "y": 437,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 5, key 1"
    },
    {
      "x": 275,
      "y": 412,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 5, key 2"
    },
    {
      "x": 375,
      "y": 412,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 5, key 3"
    },
    {
      "x": 490,
      "y": 412,
      "width": 100,
      "height": 100,
      "rotation": 1200,
      "rotationX": 490,
      "rotationY": 412,
      "place": "Left half, row 5, key 4"
    },
    {
      "x": 600,
      "y": 383,
      "width": 100,
      "height": 150,
      "rotation": 2400,
      "rotationX": 600,
      "rotationY": 433,
      "place": "Left half, row 5, key 5"
    },
    {
      "x": 800,
      "y": 383,
      "width": 100,
      "height": 150,
      "rotation": -2400,
      "rotationX": 900,
      "rotationY": 433,
      "place": "Right half, row 5, key 1"
    },
    {
      "x": 910,
      "y": 412,
      "width": 100,
      "height": 100,
      "rotation": -1200,
      "rotationX": 1010,
      "rotationY": 412,
      "place": "Right half, row 5, key 2"
    },
    {
      "x": 1025,
      "y": 412,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 5, key 3"
    },
    {
      "x": 1125,
      "y": 412,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 5, key 4"
    },
    {
      "x": 1225,
      "y": 437,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 5, key 5"
    }
  ],
  "knobs": [
    {
      "position": 42,
      "turn": [
        "Volume Down",
        "Volume Up"
      ]
    },
    {
      "position": 43,
      "turn": [
        "Previous Track",
        "Next Track"
      ]
    }
  ],
  "layers": [
    {
      "id": 0,
      "name": "default",
      "keys": [
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 53
            }
          },
          "keycap": {
            "kind": "label",
            "text": "`"
          },
          "inherited": false,
          "sentence": "Types `.",
          "spoken": "Types Grave Accent.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp GRAVE"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 30
            }
          },
          "keycap": {
            "kind": "label",
            "text": "1"
          },
          "inherited": false,
          "sentence": "Types 1.",
          "spoken": "Types 1.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N1"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 31
            }
          },
          "keycap": {
            "kind": "label",
            "text": "2"
          },
          "inherited": false,
          "sentence": "Types 2.",
          "spoken": "Types 2.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N2"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 32
            }
          },
          "keycap": {
            "kind": "label",
            "text": "3"
          },
          "inherited": false,
          "sentence": "Types 3.",
          "spoken": "Types 3.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N3"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 33
            }
          },
          "keycap": {
            "kind": "label",
            "text": "4"
          },
          "inherited": false,
          "sentence": "Types 4.",
          "spoken": "Types 4.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N4"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 34
            }
          },
          "keycap": {
            "kind": "label",
            "text": "5"
          },
          "inherited": false,
          "sentence": "Types 5.",
          "spoken": "Types 5.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N5"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 35
            }
          },
          "keycap": {
            "kind": "label",
            "text": "6"
          },
          "inherited": false,
          "sentence": "Types 6.",
          "spoken": "Types 6.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N6"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 36
            }
          },
          "keycap": {
            "kind": "label",
            "text": "7"
          },
          "inherited": false,
          "sentence": "Types 7.",
          "spoken": "Types 7.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N7"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 37
            }
          },
          "keycap": {
            "kind": "label",
            "text": "8"
          },
          "inherited": false,
          "sentence": "Types 8.",
          "spoken": "Types 8.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N8"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 38
            }
          },
          "keycap": {
            "kind": "label",
            "text": "9"
          },
          "inherited": false,
          "sentence": "Types 9.",
          "spoken": "Types 9.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N9"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 39
            }
          },
          "keycap": {
            "kind": "label",
            "text": "0"
          },
          "inherited": false,
          "sentence": "Types 0.",
          "spoken": "Types 0.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N0"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 53
            }
          },
          "keycap": {
            "kind": "label",
            "text": "`"
          },
          "inherited": false,
          "sentence": "Types `.",
          "spoken": "Types Grave Accent.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp GRAVE"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 41
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⎋ Esc"
          },
          "inherited": false,
          "sentence": "Types Escape.",
          "spoken": "Types Escape.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp ESC"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 20
            }
          },
          "keycap": {
            "kind": "label",
            "text": "Q"
          },
          "inherited": false,
          "sentence": "Types Q.",
          "spoken": "Types Q.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp Q"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 26
            }
          },
          "keycap": {
            "kind": "label",
            "text": "W"
          },
          "inherited": false,
          "sentence": "Types W.",
          "spoken": "Types W.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp W"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 8
            }
          },
          "keycap": {
            "kind": "label",
            "text": "E"
          },
          "inherited": false,
          "sentence": "Types E.",
          "spoken": "Types E.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp E"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 21
            }
          },
          "keycap": {
            "kind": "label",
            "text": "R"
          },
          "inherited": false,
          "sentence": "Types R.",
          "spoken": "Types R.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp R"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 23
            }
          },
          "keycap": {
            "kind": "label",
            "text": "T"
          },
          "inherited": false,
          "sentence": "Types T.",
          "spoken": "Types T.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp T"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 28
            }
          },
          "keycap": {
            "kind": "label",
            "text": "Y"
          },
          "inherited": false,
          "sentence": "Types Y.",
          "spoken": "Types Y.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp Y"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 24
            }
          },
          "keycap": {
            "kind": "label",
            "text": "U"
          },
          "inherited": false,
          "sentence": "Types U.",
          "spoken": "Types U.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp U"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 12
            }
          },
          "keycap": {
            "kind": "label",
            "text": "I"
          },
          "inherited": false,
          "sentence": "Types I.",
          "spoken": "Types I.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp I"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 18
            }
          },
          "keycap": {
            "kind": "label",
            "text": "O"
          },
          "inherited": false,
          "sentence": "Types O.",
          "spoken": "Types O.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp O"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 19
            }
          },
          "keycap": {
            "kind": "label",
            "text": "P"
          },
          "inherited": false,
          "sentence": "Types P.",
          "spoken": "Types P.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp P"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 42
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⌫"
          },
          "inherited": false,
          "sentence": "Types Delete.",
          "spoken": "Types Delete.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp BSPC"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 43
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⇥"
          },
          "inherited": false,
          "sentence": "Types Tab.",
          "spoken": "Types Tab.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp TAB"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 4
            }
          },
          "keycap": {
            "kind": "label",
            "text": "A"
          },
          "inherited": false,
          "sentence": "Types A.",
          "spoken": "Types A.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp A"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 22
            }
          },
          "keycap": {
            "kind": "label",
            "text": "S"
          },
          "inherited": false,
          "sentence": "Types S.",
          "spoken": "Types S.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp S"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 7
            }
          },
          "keycap": {
            "kind": "label",
            "text": "D"
          },
          "inherited": false,
          "sentence": "Types D.",
          "spoken": "Types D.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp D"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 9
            }
          },
          "keycap": {
            "kind": "label",
            "text": "F"
          },
          "inherited": false,
          "sentence": "Types F.",
          "spoken": "Types F.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp F"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 10
            }
          },
          "keycap": {
            "kind": "label",
            "text": "G"
          },
          "inherited": false,
          "sentence": "Types G.",
          "spoken": "Types G.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp G"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 11
            }
          },
          "keycap": {
            "kind": "label",
            "text": "H"
          },
          "inherited": false,
          "sentence": "Types H.",
          "spoken": "Types H.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp H"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 13
            }
          },
          "keycap": {
            "kind": "label",
            "text": "J"
          },
          "inherited": false,
          "sentence": "Types J.",
          "spoken": "Types J.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp J"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 14
            }
          },
          "keycap": {
            "kind": "label",
            "text": "K"
          },
          "inherited": false,
          "sentence": "Types K.",
          "spoken": "Types K.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp K"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 15
            }
          },
          "keycap": {
            "kind": "label",
            "text": "L"
          },
          "inherited": false,
          "sentence": "Types L.",
          "spoken": "Types L.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp L"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 51
            }
          },
          "keycap": {
            "kind": "label",
            "text": ";"
          },
          "inherited": false,
          "sentence": "Types ;.",
          "spoken": "Types Semicolon.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp SEMI"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 52
            }
          },
          "keycap": {
            "kind": "label",
            "text": "'"
          },
          "inherited": false,
          "sentence": "Types '.",
          "spoken": "Types Apostrophe.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp SQT"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 225
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⇧"
          },
          "inherited": false,
          "sentence": "Acts as ⇧ Shift.",
          "spoken": "Acts as Shift.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp LSHFT"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 29
            }
          },
          "keycap": {
            "kind": "label",
            "text": "Z"
          },
          "inherited": false,
          "sentence": "Types Z.",
          "spoken": "Types Z.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp Z"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 27
            }
          },
          "keycap": {
            "kind": "label",
            "text": "X"
          },
          "inherited": false,
          "sentence": "Types X.",
          "spoken": "Types X.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp X"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 6
            }
          },
          "keycap": {
            "kind": "label",
            "text": "C"
          },
          "inherited": false,
          "sentence": "Types C.",
          "spoken": "Types C.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp C"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 25
            }
          },
          "keycap": {
            "kind": "label",
            "text": "V"
          },
          "inherited": false,
          "sentence": "Types V.",
          "spoken": "Types V.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp V"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 5
            }
          },
          "keycap": {
            "kind": "label",
            "text": "B"
          },
          "inherited": false,
          "sentence": "Types B.",
          "spoken": "Types B.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp B"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "consumer",
              "id": 226
            }
          },
          "keycap": {
            "kind": "label",
            "text": "Mute"
          },
          "inherited": false,
          "sentence": "Presses Mute.",
          "spoken": "Presses Mute.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp C_MUTE"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "consumer",
              "id": 176
            }
          },
          "keycap": {
            "kind": "label",
            "text": "Play"
          },
          "inherited": false,
          "sentence": "Presses Play.",
          "spoken": "Presses Play.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp C_PLAY"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 17
            }
          },
          "keycap": {
            "kind": "label",
            "text": "N"
          },
          "inherited": false,
          "sentence": "Types N.",
          "spoken": "Types N.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 16
            }
          },
          "keycap": {
            "kind": "label",
            "text": "M"
          },
          "inherited": false,
          "sentence": "Types M.",
          "spoken": "Types M.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp M"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 54
            }
          },
          "keycap": {
            "kind": "label",
            "text": ","
          },
          "inherited": false,
          "sentence": "Types ,.",
          "spoken": "Types Comma.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp COMMA"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 55
            }
          },
          "keycap": {
            "kind": "label",
            "text": "."
          },
          "inherited": false,
          "sentence": "Types ..",
          "spoken": "Types Period.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp DOT"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 56
            }
          },
          "keycap": {
            "kind": "label",
            "text": "/"
          },
          "inherited": false,
          "sentence": "Types /.",
          "spoken": "Types Slash.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp FSLH"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 229
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⇧"
          },
          "inherited": false,
          "sentence": "Acts as ⇧ Shift.",
          "spoken": "Acts as Shift.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp RSHFT"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 227
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⌘"
          },
          "inherited": false,
          "sentence": "Acts as ⌘ Command.",
          "spoken": "Acts as Command.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp LGUI"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 226
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⌥"
          },
          "inherited": false,
          "sentence": "Acts as ⌥ Option.",
          "spoken": "Acts as Option.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp LALT"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 224
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⌃"
          },
          "inherited": false,
          "sentence": "Acts as ⌃ Control.",
          "spoken": "Acts as Control.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp LCTRL"
        },
        {
          "action": {
            "kind": "momentary",
            "layer": 1
          },
          "keycap": {
            "kind": "layer",
            "layer": 1,
            "name": "lower",
            "mode": "hold"
          },
          "inherited": false,
          "sentence": "Hold to use the lower layer.",
          "spoken": "Hold to use the lower layer.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&mo 1"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 40
            }
          },
          "keycap": {
            "kind": "label",
            "text": "↩"
          },
          "inherited": false,
          "sentence": "Types Return.",
          "spoken": "Types Return.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp RET"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 44
            }
          },
          "keycap": {
            "kind": "label",
            "text": "␣"
          },
          "inherited": false,
          "sentence": "Types Space.",
          "spoken": "Types Space.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp SPACE"
        },
        {
          "action": {
            "kind": "momentary",
            "layer": 2
          },
          "keycap": {
            "kind": "layer",
            "layer": 2,
            "name": "raise",
            "mode": "hold"
          },
          "inherited": false,
          "sentence": "Hold to use the raise layer.",
          "spoken": "Hold to use the raise layer.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&mo 2"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 228
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⌃"
          },
          "inherited": false,
          "sentence": "Acts as ⌃ Control.",
          "spoken": "Acts as Control.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp RCTRL"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 230
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⌥"
          },
          "inherited": false,
          "sentence": "Acts as ⌥ Option.",
          "spoken": "Acts as Option.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp RALT"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 231
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⌘"
          },
          "inherited": false,
          "sentence": "Acts as ⌘ Command.",
          "spoken": "Acts as Command.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp RGUI"
        }
      ],
      "reach": [
        {
          "text": "The base layer, always on",
          "position": null,
          "place": null
        }
      ],
      "changed": false,
      "warnings": [],
      "restorable": false,
      "macFindings": []
    },
    {
      "id": 1,
      "name": "lower",
      "keys": [
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "`"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types `.",
          "spoken": "Same as the layer below: types Grave Accent.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 58
            }
          },
          "keycap": {
            "kind": "label",
            "text": "F1"
          },
          "inherited": false,
          "sentence": "Types F1.",
          "spoken": "Types F1.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp F1"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 59
            }
          },
          "keycap": {
            "kind": "label",
            "text": "F2"
          },
          "inherited": false,
          "sentence": "Types F2.",
          "spoken": "Types F2.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp F2"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 60
            }
          },
          "keycap": {
            "kind": "label",
            "text": "F3"
          },
          "inherited": false,
          "sentence": "Types F3.",
          "spoken": "Types F3.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp F3"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 61
            }
          },
          "keycap": {
            "kind": "label",
            "text": "F4"
          },
          "inherited": false,
          "sentence": "Types F4.",
          "spoken": "Types F4.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp F4"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 62
            }
          },
          "keycap": {
            "kind": "label",
            "text": "F5"
          },
          "inherited": false,
          "sentence": "Types F5.",
          "spoken": "Types F5.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp F5"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 63
            }
          },
          "keycap": {
            "kind": "label",
            "text": "F6"
          },
          "inherited": false,
          "sentence": "Types F6.",
          "spoken": "Types F6.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp F6"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 64
            }
          },
          "keycap": {
            "kind": "label",
            "text": "F7"
          },
          "inherited": false,
          "sentence": "Types F7.",
          "spoken": "Types F7.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp F7"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 65
            }
          },
          "keycap": {
            "kind": "label",
            "text": "F8"
          },
          "inherited": false,
          "sentence": "Types F8.",
          "spoken": "Types F8.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp F8"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 66
            }
          },
          "keycap": {
            "kind": "label",
            "text": "F9"
          },
          "inherited": false,
          "sentence": "Types F9.",
          "spoken": "Types F9.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp F9"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 67
            }
          },
          "keycap": {
            "kind": "label",
            "text": "F10"
          },
          "inherited": false,
          "sentence": "Types F10.",
          "spoken": "Types F10.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp F10"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 68
            }
          },
          "keycap": {
            "kind": "label",
            "text": "F11"
          },
          "inherited": false,
          "sentence": "Types F11.",
          "spoken": "Types F11.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp F11"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 53
            }
          },
          "keycap": {
            "kind": "label",
            "text": "`"
          },
          "inherited": false,
          "sentence": "Types `.",
          "spoken": "Types Grave Accent.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp GRAVE"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 30
            }
          },
          "keycap": {
            "kind": "label",
            "text": "1"
          },
          "inherited": false,
          "sentence": "Types 1.",
          "spoken": "Types 1.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N1"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 31
            }
          },
          "keycap": {
            "kind": "label",
            "text": "2"
          },
          "inherited": false,
          "sentence": "Types 2.",
          "spoken": "Types 2.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N2"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 32
            }
          },
          "keycap": {
            "kind": "label",
            "text": "3"
          },
          "inherited": false,
          "sentence": "Types 3.",
          "spoken": "Types 3.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N3"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 33
            }
          },
          "keycap": {
            "kind": "label",
            "text": "4"
          },
          "inherited": false,
          "sentence": "Types 4.",
          "spoken": "Types 4.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N4"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 34
            }
          },
          "keycap": {
            "kind": "label",
            "text": "5"
          },
          "inherited": false,
          "sentence": "Types 5.",
          "spoken": "Types 5.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N5"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 35
            }
          },
          "keycap": {
            "kind": "label",
            "text": "6"
          },
          "inherited": false,
          "sentence": "Types 6.",
          "spoken": "Types 6.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N6"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 36
            }
          },
          "keycap": {
            "kind": "label",
            "text": "7"
          },
          "inherited": false,
          "sentence": "Types 7.",
          "spoken": "Types 7.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N7"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 37
            }
          },
          "keycap": {
            "kind": "label",
            "text": "8"
          },
          "inherited": false,
          "sentence": "Types 8.",
          "spoken": "Types 8.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N8"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 38
            }
          },
          "keycap": {
            "kind": "label",
            "text": "9"
          },
          "inherited": false,
          "sentence": "Types 9.",
          "spoken": "Types 9.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N9"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 39
            }
          },
          "keycap": {
            "kind": "label",
            "text": "0"
          },
          "inherited": false,
          "sentence": "Types 0.",
          "spoken": "Types 0.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N0"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 69
            }
          },
          "keycap": {
            "kind": "label",
            "text": "F12"
          },
          "inherited": false,
          "sentence": "Types F12.",
          "spoken": "Types F12.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp F12"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⇥"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types Tab.",
          "spoken": "Same as the layer below: types Tab.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 2,
              "page": "keyboard",
              "id": 30
            }
          },
          "keycap": {
            "kind": "label",
            "text": "!"
          },
          "inherited": false,
          "sentence": "Types !.",
          "spoken": "Types Exclamation Mark.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp EXCL"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 2,
              "page": "keyboard",
              "id": 31
            }
          },
          "keycap": {
            "kind": "label",
            "text": "@"
          },
          "inherited": false,
          "sentence": "Types @.",
          "spoken": "Types At Sign.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp AT"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 2,
              "page": "keyboard",
              "id": 32
            }
          },
          "keycap": {
            "kind": "label",
            "text": "#"
          },
          "inherited": false,
          "sentence": "Types #.",
          "spoken": "Types Number Sign.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp HASH"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 2,
              "page": "keyboard",
              "id": 33
            }
          },
          "keycap": {
            "kind": "label",
            "text": "$"
          },
          "inherited": false,
          "sentence": "Types $.",
          "spoken": "Types Dollar Sign.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp DLLR"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 2,
              "page": "keyboard",
              "id": 34
            }
          },
          "keycap": {
            "kind": "label",
            "text": "%"
          },
          "inherited": false,
          "sentence": "Types %.",
          "spoken": "Types Percent Sign.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp PRCNT"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 2,
              "page": "keyboard",
              "id": 35
            }
          },
          "keycap": {
            "kind": "label",
            "text": "^"
          },
          "inherited": false,
          "sentence": "Types ^.",
          "spoken": "Types Caret.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp CARET"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 2,
              "page": "keyboard",
              "id": 36
            }
          },
          "keycap": {
            "kind": "label",
            "text": "&"
          },
          "inherited": false,
          "sentence": "Types &.",
          "spoken": "Types Ampersand.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp AMPS"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 2,
              "page": "keyboard",
              "id": 37
            }
          },
          "keycap": {
            "kind": "label",
            "text": "*"
          },
          "inherited": false,
          "sentence": "Types *.",
          "spoken": "Types Asterisk.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp STAR"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 2,
              "page": "keyboard",
              "id": 38
            }
          },
          "keycap": {
            "kind": "label",
            "text": "("
          },
          "inherited": false,
          "sentence": "Types (.",
          "spoken": "Types Left Parenthesis.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp LPAR"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 2,
              "page": "keyboard",
              "id": 39
            }
          },
          "keycap": {
            "kind": "label",
            "text": ")"
          },
          "inherited": false,
          "sentence": "Types ).",
          "spoken": "Types Right Parenthesis.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp RPAR"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 2,
              "page": "keyboard",
              "id": 49
            }
          },
          "keycap": {
            "kind": "label",
            "text": "|"
          },
          "inherited": false,
          "sentence": "Types |.",
          "spoken": "Types Vertical Bar.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp PIPE"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⇧"
          },
          "inherited": true,
          "sentence": "Same as the layer below: acts as ⇧ Shift.",
          "spoken": "Same as the layer below: acts as Shift.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 46
            }
          },
          "keycap": {
            "kind": "label",
            "text": "="
          },
          "inherited": false,
          "sentence": "Types =.",
          "spoken": "Types Equals Sign.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp EQUAL"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 45
            }
          },
          "keycap": {
            "kind": "label",
            "text": "-"
          },
          "inherited": false,
          "sentence": "Types -.",
          "spoken": "Types Minus.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp MINUS"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 2,
              "page": "keyboard",
              "id": 46
            }
          },
          "keycap": {
            "kind": "label",
            "text": "+"
          },
          "inherited": false,
          "sentence": "Types +.",
          "spoken": "Types Plus Sign.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp PLUS"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 2,
              "page": "keyboard",
              "id": 47
            }
          },
          "keycap": {
            "kind": "label",
            "text": "{"
          },
          "inherited": false,
          "sentence": "Types {.",
          "spoken": "Types Left Brace.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp LBRC"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 2,
              "page": "keyboard",
              "id": 48
            }
          },
          "keycap": {
            "kind": "label",
            "text": "}"
          },
          "inherited": false,
          "sentence": "Types }.",
          "spoken": "Types Right Brace.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp RBRC"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "Mute"
          },
          "inherited": true,
          "sentence": "Same as the layer below: presses Mute.",
          "spoken": "Same as the layer below: presses Mute.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "Play"
          },
          "inherited": true,
          "sentence": "Same as the layer below: presses Play.",
          "spoken": "Same as the layer below: presses Play.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 47
            }
          },
          "keycap": {
            "kind": "label",
            "text": "["
          },
          "inherited": false,
          "sentence": "Types [.",
          "spoken": "Types Left Bracket.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp LBKT"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 48
            }
          },
          "keycap": {
            "kind": "label",
            "text": "]"
          },
          "inherited": false,
          "sentence": "Types ].",
          "spoken": "Types Right Bracket.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp RBKT"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 51
            }
          },
          "keycap": {
            "kind": "label",
            "text": ";"
          },
          "inherited": false,
          "sentence": "Types ;.",
          "spoken": "Types Semicolon.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp SEMI"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 2,
              "page": "keyboard",
              "id": 51
            }
          },
          "keycap": {
            "kind": "label",
            "text": ":"
          },
          "inherited": false,
          "sentence": "Types :.",
          "spoken": "Types Colon.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp COLON"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 49
            }
          },
          "keycap": {
            "kind": "label",
            "text": "\\"
          },
          "inherited": false,
          "sentence": "Types \\.",
          "spoken": "Types Backslash.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp BSLH"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⇧"
          },
          "inherited": true,
          "sentence": "Same as the layer below: acts as ⇧ Shift.",
          "spoken": "Same as the layer below: acts as Shift.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⌘"
          },
          "inherited": true,
          "sentence": "Same as the layer below: acts as ⌘ Command.",
          "spoken": "Same as the layer below: acts as Command.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⌥"
          },
          "inherited": true,
          "sentence": "Same as the layer below: acts as ⌥ Option.",
          "spoken": "Same as the layer below: acts as Option.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⌃"
          },
          "inherited": true,
          "sentence": "Same as the layer below: acts as ⌃ Control.",
          "spoken": "Same as the layer below: acts as Control.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "layer",
            "layer": 1,
            "name": "lower",
            "mode": "hold"
          },
          "inherited": true,
          "sentence": "Same as the layer below: hold to use the lower layer.",
          "spoken": "Same as the layer below: hold to use the lower layer.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "↩"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types Return.",
          "spoken": "Same as the layer below: types Return.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "␣"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types Space.",
          "spoken": "Same as the layer below: types Space.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "layer",
            "layer": 2,
            "name": "raise",
            "mode": "hold"
          },
          "inherited": true,
          "sentence": "Same as the layer below: hold to use the raise layer.",
          "spoken": "Same as the layer below: hold to use the raise layer.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⌃"
          },
          "inherited": true,
          "sentence": "Same as the layer below: acts as ⌃ Control.",
          "spoken": "Same as the layer below: acts as Control.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⌥"
          },
          "inherited": true,
          "sentence": "Same as the layer below: acts as ⌥ Option.",
          "spoken": "Same as the layer below: acts as Option.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⌘"
          },
          "inherited": true,
          "sentence": "Same as the layer below: acts as ⌘ Command.",
          "spoken": "Same as the layer below: acts as Command.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        }
      ],
      "reach": [
        {
          "text": "Hold lower",
          "position": 53,
          "place": "Left half, row 5, key 4"
        }
      ],
      "changed": false,
      "warnings": [],
      "restorable": false,
      "macFindings": []
    },
    {
      "id": 2,
      "name": "raise",
      "keys": [
        {
          "action": {
            "kind": "bluetooth",
            "command": "clear",
            "profile": 0
          },
          "keycap": {
            "kind": "label",
            "text": "ᛒ Clear"
          },
          "inherited": false,
          "sentence": "Forgets the pairing of the Bluetooth device in use.",
          "spoken": "Forgets the pairing of the Bluetooth device in use.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&bt BT_CLR"
        },
        {
          "action": {
            "kind": "bluetooth",
            "command": "select",
            "profile": 0
          },
          "keycap": {
            "kind": "label",
            "text": "ᛒ 1"
          },
          "inherited": false,
          "sentence": "Uses Bluetooth device 1.",
          "spoken": "Uses Bluetooth device 1.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&bt BT_SEL 0"
        },
        {
          "action": {
            "kind": "bluetooth",
            "command": "select",
            "profile": 1
          },
          "keycap": {
            "kind": "label",
            "text": "ᛒ 2"
          },
          "inherited": false,
          "sentence": "Uses Bluetooth device 2.",
          "spoken": "Uses Bluetooth device 2.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&bt BT_SEL 1"
        },
        {
          "action": {
            "kind": "bluetooth",
            "command": "select",
            "profile": 2
          },
          "keycap": {
            "kind": "label",
            "text": "ᛒ 3"
          },
          "inherited": false,
          "sentence": "Uses Bluetooth device 3.",
          "spoken": "Uses Bluetooth device 3.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&bt BT_SEL 2"
        },
        {
          "action": {
            "kind": "bluetooth",
            "command": "select",
            "profile": 3
          },
          "keycap": {
            "kind": "label",
            "text": "ᛒ 4"
          },
          "inherited": false,
          "sentence": "Uses Bluetooth device 4.",
          "spoken": "Uses Bluetooth device 4.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&bt BT_SEL 3"
        },
        {
          "action": {
            "kind": "bluetooth",
            "command": "select",
            "profile": 4
          },
          "keycap": {
            "kind": "label",
            "text": "ᛒ 5"
          },
          "inherited": false,
          "sentence": "Uses Bluetooth device 5.",
          "spoken": "Uses Bluetooth device 5.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&bt BT_SEL 4"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "6"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types 6.",
          "spoken": "Same as the layer below: types 6.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "7"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types 7.",
          "spoken": "Same as the layer below: types 7.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "8"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types 8.",
          "spoken": "Same as the layer below: types 8.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "9"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types 9.",
          "spoken": "Same as the layer below: types 9.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "0"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types 0.",
          "spoken": "Same as the layer below: types 0.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "`"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types `.",
          "spoken": "Same as the layer below: types Grave Accent.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⎋ Esc"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types Escape.",
          "spoken": "Same as the layer below: types Escape.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 73
            }
          },
          "keycap": {
            "kind": "label",
            "text": "Insert"
          },
          "inherited": false,
          "sentence": "Types Insert.",
          "spoken": "Types Insert.",
          "secondary": null,
          "changed": false,
          "macWarning": true,
          "restorable": false,
          "zmk": "&kp INS"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 70
            }
          },
          "keycap": {
            "kind": "label",
            "text": "Print Screen"
          },
          "inherited": false,
          "sentence": "Types Print Screen.",
          "spoken": "Types Print Screen.",
          "secondary": null,
          "changed": false,
          "macWarning": true,
          "restorable": false,
          "zmk": "&kp PSCRN"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 101
            }
          },
          "keycap": {
            "kind": "label",
            "text": "Menu"
          },
          "inherited": false,
          "sentence": "Types Menu.",
          "spoken": "Types Menu.",
          "secondary": null,
          "changed": false,
          "macWarning": true,
          "restorable": false,
          "zmk": "&kp K_APP"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "R"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types R.",
          "spoken": "Same as the layer below: types R.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "T"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types T.",
          "spoken": "Same as the layer below: types T.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 75
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⇞"
          },
          "inherited": false,
          "sentence": "Types Page Up.",
          "spoken": "Types Page Up.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp PG_UP"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "U"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types U.",
          "spoken": "Same as the layer below: types U.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 82
            }
          },
          "keycap": {
            "kind": "label",
            "text": "↑"
          },
          "inherited": false,
          "sentence": "Types Up Arrow.",
          "spoken": "Types Up Arrow.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp UP"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "O"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types O.",
          "spoken": "Same as the layer below: types O.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 39
            }
          },
          "keycap": {
            "kind": "label",
            "text": "0"
          },
          "inherited": false,
          "sentence": "Types 0.",
          "spoken": "Types 0.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp N0"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⌫"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types Delete.",
          "spoken": "Same as the layer below: types Delete.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⇥"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types Tab.",
          "spoken": "Same as the layer below: types Tab.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 226
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⌥"
          },
          "inherited": false,
          "sentence": "Acts as ⌥ Option.",
          "spoken": "Acts as Option.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp LALT"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 224
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⌃"
          },
          "inherited": false,
          "sentence": "Acts as ⌃ Control.",
          "spoken": "Acts as Control.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp LCTRL"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 225
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⇧"
          },
          "inherited": false,
          "sentence": "Acts as ⇧ Shift.",
          "spoken": "Acts as Shift.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp LSHFT"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "F"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types F.",
          "spoken": "Same as the layer below: types F.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 57
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⇪"
          },
          "inherited": false,
          "sentence": "Types Caps Lock.",
          "spoken": "Types Caps Lock.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp CAPS"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 78
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⇟"
          },
          "inherited": false,
          "sentence": "Types Page Down.",
          "spoken": "Types Page Down.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp PG_DN"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 80
            }
          },
          "keycap": {
            "kind": "label",
            "text": "←"
          },
          "inherited": false,
          "sentence": "Types Left Arrow.",
          "spoken": "Types Left Arrow.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp LEFT"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 81
            }
          },
          "keycap": {
            "kind": "label",
            "text": "↓"
          },
          "inherited": false,
          "sentence": "Types Down Arrow.",
          "spoken": "Types Down Arrow.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp DOWN"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 79
            }
          },
          "keycap": {
            "kind": "label",
            "text": "→"
          },
          "inherited": false,
          "sentence": "Types Right Arrow.",
          "spoken": "Types Right Arrow.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp RIGHT"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 76
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⌦"
          },
          "inherited": false,
          "sentence": "Types Forward Delete.",
          "spoken": "Types Forward Delete.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp DEL"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 42
            }
          },
          "keycap": {
            "kind": "label",
            "text": "⌫"
          },
          "inherited": false,
          "sentence": "Types Delete.",
          "spoken": "Types Delete.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&kp BSPC"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⇧"
          },
          "inherited": true,
          "sentence": "Same as the layer below: acts as ⇧ Shift.",
          "spoken": "Same as the layer below: acts as Shift.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 122
            }
          },
          "keycap": {
            "kind": "label",
            "text": "Undo"
          },
          "inherited": false,
          "sentence": "Types Undo.",
          "spoken": "Types Undo.",
          "secondary": null,
          "changed": false,
          "macWarning": true,
          "restorable": false,
          "zmk": "&kp K_UNDO"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 123
            }
          },
          "keycap": {
            "kind": "label",
            "text": "Cut"
          },
          "inherited": false,
          "sentence": "Types Cut.",
          "spoken": "Types Cut.",
          "secondary": null,
          "changed": false,
          "macWarning": true,
          "restorable": false,
          "zmk": "&kp K_CUT"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 124
            }
          },
          "keycap": {
            "kind": "label",
            "text": "Copy"
          },
          "inherited": false,
          "sentence": "Types Copy.",
          "spoken": "Types Copy.",
          "secondary": null,
          "changed": false,
          "macWarning": true,
          "restorable": false,
          "zmk": "&kp K_COPY"
        },
        {
          "action": {
            "kind": "key-press",
            "key": {
              "mods": 0,
              "page": "keyboard",
              "id": 125
            }
          },
          "keycap": {
            "kind": "label",
            "text": "Paste"
          },
          "inherited": false,
          "sentence": "Types Paste.",
          "spoken": "Types Paste.",
          "secondary": null,
          "changed": false,
          "macWarning": true,
          "restorable": false,
          "zmk": "&kp K_PASTE"
        },
        {
          "action": {
            "kind": "unlock"
          },
          "keycap": {
            "kind": "label",
            "text": "Unlock"
          },
          "inherited": false,
          "sentence": "Unlocks the keyboard, so Keywright can change it.",
          "spoken": "Unlocks the keyboard, so Keywright can change it.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&studio_unlock"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "Mute"
          },
          "inherited": true,
          "sentence": "Same as the layer below: presses Mute.",
          "spoken": "Same as the layer below: presses Mute.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "Play"
          },
          "inherited": true,
          "sentence": "Same as the layer below: presses Play.",
          "spoken": "Same as the layer below: presses Play.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "N"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types N.",
          "spoken": "Same as the layer below: types N.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "M"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types M.",
          "spoken": "Same as the layer below: types M.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": ","
          },
          "inherited": true,
          "sentence": "Same as the layer below: types ,.",
          "spoken": "Same as the layer below: types Comma.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "."
          },
          "inherited": true,
          "sentence": "Same as the layer below: types ..",
          "spoken": "Same as the layer below: types Period.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "/"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types /.",
          "spoken": "Same as the layer below: types Slash.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⇧"
          },
          "inherited": true,
          "sentence": "Same as the layer below: acts as ⇧ Shift.",
          "spoken": "Same as the layer below: acts as Shift.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⌘"
          },
          "inherited": true,
          "sentence": "Same as the layer below: acts as ⌘ Command.",
          "spoken": "Same as the layer below: acts as Command.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⌥"
          },
          "inherited": true,
          "sentence": "Same as the layer below: acts as ⌥ Option.",
          "spoken": "Same as the layer below: acts as Option.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⌃"
          },
          "inherited": true,
          "sentence": "Same as the layer below: acts as ⌃ Control.",
          "spoken": "Same as the layer below: acts as Control.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "layer",
            "layer": 1,
            "name": "lower",
            "mode": "hold"
          },
          "inherited": true,
          "sentence": "Same as the layer below: hold to use the lower layer.",
          "spoken": "Same as the layer below: hold to use the lower layer.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "↩"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types Return.",
          "spoken": "Same as the layer below: types Return.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "␣"
          },
          "inherited": true,
          "sentence": "Same as the layer below: types Space.",
          "spoken": "Same as the layer below: types Space.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "layer",
            "layer": 2,
            "name": "raise",
            "mode": "hold"
          },
          "inherited": true,
          "sentence": "Same as the layer below: hold to use the raise layer.",
          "spoken": "Same as the layer below: hold to use the raise layer.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⌃"
          },
          "inherited": true,
          "sentence": "Same as the layer below: acts as ⌃ Control.",
          "spoken": "Same as the layer below: acts as Control.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⌥"
          },
          "inherited": true,
          "sentence": "Same as the layer below: acts as ⌥ Option.",
          "spoken": "Same as the layer below: acts as Option.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        },
        {
          "action": {
            "kind": "transparent"
          },
          "keycap": {
            "kind": "label",
            "text": "⌘"
          },
          "inherited": true,
          "sentence": "Same as the layer below: acts as ⌘ Command.",
          "spoken": "Same as the layer below: acts as Command.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&trans"
        }
      ],
      "reach": [
        {
          "text": "Hold raise",
          "position": 56,
          "place": "Right half, row 5, key 2"
        }
      ],
      "changed": false,
      "warnings": [],
      "restorable": false,
      "macFindings": [
        {
          "finding": {
            "layer": 2,
            "position": 13,
            "page": "keyboard",
            "id": 73
          },
          "place": "Left half, row 2, key 2",
          "name": "Insert (Windows style)",
          "reason": "Macs have no Insert key, so it does nothing.",
          "fix": null
        },
        {
          "finding": {
            "layer": 2,
            "position": 14,
            "page": "keyboard",
            "id": 70
          },
          "place": "Left half, row 2, key 3",
          "name": "Print Screen (Windows style)",
          "reason": "It doesn't take a screenshot on a Mac.",
          "fix": "⇧⌘3"
        },
        {
          "finding": {
            "layer": 2,
            "position": 15,
            "page": "keyboard",
            "id": 101
          },
          "place": "Left half, row 2, key 4",
          "name": "Menu key (Windows style)",
          "reason": "Macs open menus with a Control-click, not a key, so it does nothing.",
          "fix": null
        },
        {
          "finding": {
            "layer": 2,
            "position": 37,
            "page": "keyboard",
            "id": 122
          },
          "place": "Left half, row 4, key 2",
          "name": "Undo (Linux style)",
          "reason": "It doesn't do anything on a Mac.",
          "fix": "⌘Z"
        },
        {
          "finding": {
            "layer": 2,
            "position": 38,
            "page": "keyboard",
            "id": 123
          },
          "place": "Left half, row 4, key 3",
          "name": "Cut (Linux style)",
          "reason": "It doesn't do anything on a Mac.",
          "fix": "⌘X"
        },
        {
          "finding": {
            "layer": 2,
            "position": 39,
            "page": "keyboard",
            "id": 124
          },
          "place": "Left half, row 4, key 4",
          "name": "Copy (Linux style)",
          "reason": "It doesn't do anything on a Mac.",
          "fix": "⌘C"
        },
        {
          "finding": {
            "layer": 2,
            "position": 40,
            "page": "keyboard",
            "id": 125
          },
          "place": "Left half, row 4, key 5",
          "name": "Paste (Linux style)",
          "reason": "It doesn't do anything on a Mac.",
          "fix": "⌘V"
        }
      ]
    },
    {
      "id": 3,
      "name": "adjust",
      "keys": [
        {
          "action": {
            "kind": "bluetooth",
            "command": "clear",
            "profile": 0
          },
          "keycap": {
            "kind": "label",
            "text": "ᛒ Clear"
          },
          "inherited": false,
          "sentence": "Forgets the pairing of the Bluetooth device in use.",
          "spoken": "Forgets the pairing of the Bluetooth device in use.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&bt BT_CLR"
        },
        {
          "action": {
            "kind": "bluetooth",
            "command": "select",
            "profile": 0
          },
          "keycap": {
            "kind": "label",
            "text": "ᛒ 1"
          },
          "inherited": false,
          "sentence": "Uses Bluetooth device 1.",
          "spoken": "Uses Bluetooth device 1.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&bt BT_SEL 0"
        },
        {
          "action": {
            "kind": "bluetooth",
            "command": "select",
            "profile": 1
          },
          "keycap": {
            "kind": "label",
            "text": "ᛒ 2"
          },
          "inherited": false,
          "sentence": "Uses Bluetooth device 2.",
          "spoken": "Uses Bluetooth device 2.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&bt BT_SEL 1"
        },
        {
          "action": {
            "kind": "bluetooth",
            "command": "select",
            "profile": 2
          },
          "keycap": {
            "kind": "label",
            "text": "ᛒ 3"
          },
          "inherited": false,
          "sentence": "Uses Bluetooth device 3.",
          "spoken": "Uses Bluetooth device 3.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&bt BT_SEL 2"
        },
        {
          "action": {
            "kind": "bluetooth",
            "command": "select",
            "profile": 3
          },
          "keycap": {
            "kind": "label",
            "text": "ᛒ 4"
          },
          "inherited": false,
          "sentence": "Uses Bluetooth device 4.",
          "spoken": "Uses Bluetooth device 4.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&bt BT_SEL 3"
        },
        {
          "action": {
            "kind": "bluetooth",
            "command": "select",
            "profile": 4
          },
          "keycap": {
            "kind": "label",
            "text": "ᛒ 5"
          },
          "inherited": false,
          "sentence": "Uses Bluetooth device 5.",
          "spoken": "Uses Bluetooth device 5.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&bt BT_SEL 4"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "external-power",
            "command": "toggle"
          },
          "keycap": {
            "kind": "label",
            "text": "Power"
          },
          "inherited": false,
          "sentence": "Turns the power to the lights and display on or off.",
          "spoken": "Turns the power to the lights and display on or off.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&ext_power EP_TOG"
        },
        {
          "action": {
            "kind": "lighting",
            "command": "hue-down"
          },
          "keycap": {
            "kind": "label",
            "text": "Hue −"
          },
          "inherited": false,
          "sentence": "Shifts the lights' colour the other way.",
          "spoken": "Shifts the lights' colour the other way.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&rgb_ug RGB_HUD"
        },
        {
          "action": {
            "kind": "lighting",
            "command": "hue-up"
          },
          "keycap": {
            "kind": "label",
            "text": "Hue +"
          },
          "inherited": false,
          "sentence": "Shifts the lights' colour one way.",
          "spoken": "Shifts the lights' colour one way.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&rgb_ug RGB_HUI"
        },
        {
          "action": {
            "kind": "lighting",
            "command": "saturation-down"
          },
          "keycap": {
            "kind": "label",
            "text": "Saturation −"
          },
          "inherited": false,
          "sentence": "Makes the lights' colour paler.",
          "spoken": "Makes the lights' colour paler.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&rgb_ug RGB_SAD"
        },
        {
          "action": {
            "kind": "lighting",
            "command": "saturation-up"
          },
          "keycap": {
            "kind": "label",
            "text": "Saturation +"
          },
          "inherited": false,
          "sentence": "Makes the lights' colour stronger.",
          "spoken": "Makes the lights' colour stronger.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&rgb_ug RGB_SAI"
        },
        {
          "action": {
            "kind": "lighting",
            "command": "next-effect"
          },
          "keycap": {
            "kind": "label",
            "text": "Next effect"
          },
          "inherited": false,
          "sentence": "Switches the lights to the next effect.",
          "spoken": "Switches the lights to the next effect.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&rgb_ug RGB_EFF"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "lighting",
            "command": "brightness-down"
          },
          "keycap": {
            "kind": "label",
            "text": "Brightness −"
          },
          "inherited": false,
          "sentence": "Makes the lights dimmer.",
          "spoken": "Makes the lights dimmer.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&rgb_ug RGB_BRD"
        },
        {
          "action": {
            "kind": "lighting",
            "command": "brightness-up"
          },
          "keycap": {
            "kind": "label",
            "text": "Brightness +"
          },
          "inherited": false,
          "sentence": "Makes the lights brighter.",
          "spoken": "Makes the lights brighter.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&rgb_ug RGB_BRI"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "lighting",
            "command": "toggle"
          },
          "keycap": {
            "kind": "label",
            "text": "Lights"
          },
          "inherited": false,
          "sentence": "Turns the lights on or off.",
          "spoken": "Turns the lights on or off.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&rgb_ug RGB_TOG"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        },
        {
          "action": {
            "kind": "nothing"
          },
          "keycap": {
            "kind": "nothing"
          },
          "inherited": false,
          "sentence": "Does nothing.",
          "spoken": "Does nothing.",
          "secondary": null,
          "changed": false,
          "macWarning": false,
          "restorable": false,
          "zmk": "&none"
        }
      ],
      "reach": [
        {
          "text": "lower + raise, built into the firmware",
          "position": null,
          "place": null
        }
      ],
      "changed": false,
      "warnings": [],
      "restorable": false,
      "macFindings": []
    }
  ],
  "freeLayerSlots": 0,
  "layerNameMax": 18,
  "unlockable": true,
  "choices": [
    {
      "category": "media",
      "label": "Play/Pause",
      "sentence": "Presses Play/Pause.",
      "action": {
        "kind": "key-press",
        "key": {
          "mods": 0,
          "page": "consumer",
          "id": 205
        }
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "media",
      "label": "Next Track",
      "sentence": "Presses Next Track.",
      "action": {
        "kind": "key-press",
        "key": {
          "mods": 0,
          "page": "consumer",
          "id": 181
        }
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "media",
      "label": "Previous Track",
      "sentence": "Presses Previous Track.",
      "action": {
        "kind": "key-press",
        "key": {
          "mods": 0,
          "page": "consumer",
          "id": 182
        }
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "media",
      "label": "Volume Up",
      "sentence": "Presses Volume Up.",
      "action": {
        "kind": "key-press",
        "key": {
          "mods": 0,
          "page": "consumer",
          "id": 233
        }
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "media",
      "label": "Volume Down",
      "sentence": "Presses Volume Down.",
      "action": {
        "kind": "key-press",
        "key": {
          "mods": 0,
          "page": "consumer",
          "id": 234
        }
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "media",
      "label": "Mute",
      "sentence": "Presses Mute.",
      "action": {
        "kind": "key-press",
        "key": {
          "mods": 0,
          "page": "consumer",
          "id": 226
        }
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "media",
      "label": "Brightness Up",
      "sentence": "Presses Brightness Up.",
      "action": {
        "kind": "key-press",
        "key": {
          "mods": 0,
          "page": "consumer",
          "id": 111
        }
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "media",
      "label": "Brightness Down",
      "sentence": "Presses Brightness Down.",
      "action": {
        "kind": "key-press",
        "key": {
          "mods": 0,
          "page": "consumer",
          "id": 112
        }
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "bluetooth",
      "label": "ᛒ 1",
      "sentence": "Uses Bluetooth device 1.",
      "action": {
        "kind": "bluetooth",
        "command": "select",
        "profile": 0
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "bluetooth",
      "label": "ᛒ 2",
      "sentence": "Uses Bluetooth device 2.",
      "action": {
        "kind": "bluetooth",
        "command": "select",
        "profile": 1
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "bluetooth",
      "label": "ᛒ 3",
      "sentence": "Uses Bluetooth device 3.",
      "action": {
        "kind": "bluetooth",
        "command": "select",
        "profile": 2
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "bluetooth",
      "label": "ᛒ 4",
      "sentence": "Uses Bluetooth device 4.",
      "action": {
        "kind": "bluetooth",
        "command": "select",
        "profile": 3
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "bluetooth",
      "label": "ᛒ 5",
      "sentence": "Uses Bluetooth device 5.",
      "action": {
        "kind": "bluetooth",
        "command": "select",
        "profile": 4
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "bluetooth",
      "label": "ᛒ Next",
      "sentence": "Uses the next Bluetooth device.",
      "action": {
        "kind": "bluetooth",
        "command": "next",
        "profile": 0
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "bluetooth",
      "label": "ᛒ Previous",
      "sentence": "Uses the previous Bluetooth device.",
      "action": {
        "kind": "bluetooth",
        "command": "previous",
        "profile": 0
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "bluetooth",
      "label": "ᛒ Clear",
      "sentence": "Forgets the pairing of the Bluetooth device in use.",
      "action": {
        "kind": "bluetooth",
        "command": "clear",
        "profile": 0
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "bluetooth",
      "label": "ᛒ Clear all",
      "sentence": "Forgets every Bluetooth pairing.",
      "action": {
        "kind": "bluetooth",
        "command": "clear-all",
        "profile": 0
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "bluetooth",
      "label": "Use USB",
      "sentence": "Types over USB.",
      "action": {
        "kind": "output",
        "command": "usb"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "bluetooth",
      "label": "Use ᛒ",
      "sentence": "Types over Bluetooth.",
      "action": {
        "kind": "output",
        "command": "bluetooth"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "bluetooth",
      "label": "USB or ᛒ",
      "sentence": "Switches between typing over USB and over Bluetooth.",
      "action": {
        "kind": "output",
        "command": "toggle"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "lighting",
      "label": "Lights",
      "sentence": "Turns the lights on or off.",
      "action": {
        "kind": "lighting",
        "command": "toggle"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "lighting",
      "label": "Lights on",
      "sentence": "Turns the lights on.",
      "action": {
        "kind": "lighting",
        "command": "on"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "lighting",
      "label": "Lights off",
      "sentence": "Turns the lights off.",
      "action": {
        "kind": "lighting",
        "command": "off"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "lighting",
      "label": "Brightness +",
      "sentence": "Makes the lights brighter.",
      "action": {
        "kind": "lighting",
        "command": "brightness-up"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "lighting",
      "label": "Brightness −",
      "sentence": "Makes the lights dimmer.",
      "action": {
        "kind": "lighting",
        "command": "brightness-down"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "lighting",
      "label": "Hue +",
      "sentence": "Shifts the lights' colour one way.",
      "action": {
        "kind": "lighting",
        "command": "hue-up"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "lighting",
      "label": "Hue −",
      "sentence": "Shifts the lights' colour the other way.",
      "action": {
        "kind": "lighting",
        "command": "hue-down"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "lighting",
      "label": "Saturation +",
      "sentence": "Makes the lights' colour stronger.",
      "action": {
        "kind": "lighting",
        "command": "saturation-up"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "lighting",
      "label": "Saturation −",
      "sentence": "Makes the lights' colour paler.",
      "action": {
        "kind": "lighting",
        "command": "saturation-down"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "lighting",
      "label": "Next effect",
      "sentence": "Switches the lights to the next effect.",
      "action": {
        "kind": "lighting",
        "command": "next-effect"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "lighting",
      "label": "Previous effect",
      "sentence": "Switches the lights to the previous effect.",
      "action": {
        "kind": "lighting",
        "command": "previous-effect"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "lighting",
      "label": "Speed +",
      "sentence": "Speeds up the lights' effect.",
      "action": {
        "kind": "lighting",
        "command": "speed-up"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "lighting",
      "label": "Speed −",
      "sentence": "Slows down the lights' effect.",
      "action": {
        "kind": "lighting",
        "command": "speed-down"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "special",
      "label": "Caps Word",
      "sentence": "Types capitals until the end of the word.",
      "action": {
        "kind": "caps-word"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "special",
      "label": "Repeat",
      "sentence": "Repeats the last key.",
      "action": {
        "kind": "key-repeat"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "special",
      "label": "Unlock",
      "sentence": "Unlocks the keyboard, so Keywright can change it.",
      "action": {
        "kind": "unlock"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "special",
      "label": "Update mode",
      "sentence": "Puts the keyboard into update mode, for new firmware.",
      "action": {
        "kind": "bootloader"
      },
      "icon": null,
      "defaultShortcut": null
    },
    {
      "category": "special",
      "label": "Restart",
      "sentence": "Restarts the keyboard.",
      "action": {
        "kind": "reset"
      },
      "icon": null,
      "defaultShortcut": null
    }
  ],
  "others": [
    {
      "id": 4,
      "name": "Grave/Escape",
      "parameters": []
    },
    {
      "id": 6,
      "name": "Key Toggle",
      "parameters": [
        {
          "param1": [
            {
              "name": "Key",
              "value": {
                "kind": "key"
              }
            }
          ],
          "param2": []
        }
      ]
    },
    {
      "id": 20,
      "name": "enc_key_press",
      "parameters": []
    }
  ],
  "kinds": [
    "key-press",
    "mod-tap",
    "layer-tap",
    "momentary",
    "to-layer",
    "toggle",
    "sticky-layer",
    "sticky-key",
    "transparent",
    "nothing"
  ],
  "zmkNames": false
};

export const keyCatalog: KeyEntry[] = [
  {
    "page": "keyboard",
    "id": 4,
    "cap": "A",
    "name": "A",
    "spoken": "A",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "A"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 5,
    "cap": "B",
    "name": "B",
    "spoken": "B",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "B"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 6,
    "cap": "C",
    "name": "C",
    "spoken": "C",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 7,
    "cap": "D",
    "name": "D",
    "spoken": "D",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "D"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 8,
    "cap": "E",
    "name": "E",
    "spoken": "E",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "E"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 9,
    "cap": "F",
    "name": "F",
    "spoken": "F",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 10,
    "cap": "G",
    "name": "G",
    "spoken": "G",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "G"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 11,
    "cap": "H",
    "name": "H",
    "spoken": "H",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "H"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 12,
    "cap": "I",
    "name": "I",
    "spoken": "I",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "I"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 13,
    "cap": "J",
    "name": "J",
    "spoken": "J",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "J"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 14,
    "cap": "K",
    "name": "K",
    "spoken": "K",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 15,
    "cap": "L",
    "name": "L",
    "spoken": "L",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "L"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 16,
    "cap": "M",
    "name": "M",
    "spoken": "M",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "M"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 17,
    "cap": "N",
    "name": "N",
    "spoken": "N",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "N"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 18,
    "cap": "O",
    "name": "O",
    "spoken": "O",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "O"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 19,
    "cap": "P",
    "name": "P",
    "spoken": "P",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "P"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 20,
    "cap": "Q",
    "name": "Q",
    "spoken": "Q",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "Q"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 21,
    "cap": "R",
    "name": "R",
    "spoken": "R",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "R"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 22,
    "cap": "S",
    "name": "S",
    "spoken": "S",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "S"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 23,
    "cap": "T",
    "name": "T",
    "spoken": "T",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "T"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 24,
    "cap": "U",
    "name": "U",
    "spoken": "U",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "U"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 25,
    "cap": "V",
    "name": "V",
    "spoken": "V",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "V"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 26,
    "cap": "W",
    "name": "W",
    "spoken": "W",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "W"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 27,
    "cap": "X",
    "name": "X",
    "spoken": "X",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "X"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 28,
    "cap": "Y",
    "name": "Y",
    "spoken": "Y",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "Y"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 29,
    "cap": "Z",
    "name": "Z",
    "spoken": "Z",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "Z"
    ],
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 30,
    "cap": "1",
    "name": "1",
    "spoken": "1",
    "shifted": "!",
    "shiftedSpoken": "Exclamation Mark",
    "shiftedZmk": "EXCL",
    "zmk": [
      "N1",
      "NUMBER_1",
      "NUM_1"
    ],
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 31,
    "cap": "2",
    "name": "2",
    "spoken": "2",
    "shifted": "@",
    "shiftedSpoken": "At Sign",
    "shiftedZmk": "AT",
    "zmk": [
      "N2",
      "NUMBER_2",
      "NUM_2"
    ],
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 32,
    "cap": "3",
    "name": "3",
    "spoken": "3",
    "shifted": "#",
    "shiftedSpoken": "Number Sign",
    "shiftedZmk": "HASH",
    "zmk": [
      "N3",
      "NUMBER_3",
      "NUM_3"
    ],
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 33,
    "cap": "4",
    "name": "4",
    "spoken": "4",
    "shifted": "$",
    "shiftedSpoken": "Dollar Sign",
    "shiftedZmk": "DLLR",
    "zmk": [
      "N4",
      "NUMBER_4",
      "NUM_4"
    ],
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 34,
    "cap": "5",
    "name": "5",
    "spoken": "5",
    "shifted": "%",
    "shiftedSpoken": "Percent Sign",
    "shiftedZmk": "PRCNT",
    "zmk": [
      "N5",
      "NUMBER_5",
      "NUM_5"
    ],
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 35,
    "cap": "6",
    "name": "6",
    "spoken": "6",
    "shifted": "^",
    "shiftedSpoken": "Caret",
    "shiftedZmk": "CARET",
    "zmk": [
      "N6",
      "NUMBER_6",
      "NUM_6"
    ],
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 36,
    "cap": "7",
    "name": "7",
    "spoken": "7",
    "shifted": "&",
    "shiftedSpoken": "Ampersand",
    "shiftedZmk": "AMPS",
    "zmk": [
      "N7",
      "NUMBER_7",
      "NUM_7"
    ],
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 37,
    "cap": "8",
    "name": "8",
    "spoken": "8",
    "shifted": "*",
    "shiftedSpoken": "Asterisk",
    "shiftedZmk": "STAR",
    "zmk": [
      "N8",
      "NUMBER_8",
      "NUM_8"
    ],
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 38,
    "cap": "9",
    "name": "9",
    "spoken": "9",
    "shifted": "(",
    "shiftedSpoken": "Left Parenthesis",
    "shiftedZmk": "LPAR",
    "zmk": [
      "N9",
      "NUMBER_9",
      "NUM_9"
    ],
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 39,
    "cap": "0",
    "name": "0",
    "spoken": "0",
    "shifted": ")",
    "shiftedSpoken": "Right Parenthesis",
    "shiftedZmk": "RPAR",
    "zmk": [
      "N0",
      "NUMBER_0",
      "NUM_0"
    ],
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 40,
    "cap": "↩",
    "name": "Return",
    "spoken": "Return",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "RET",
      "RETURN",
      "ENTER"
    ],
    "group": "editing"
  },
  {
    "page": "keyboard",
    "id": 41,
    "cap": "⎋ Esc",
    "name": "Escape",
    "spoken": "Escape",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "ESC",
      "ESCAPE"
    ],
    "group": "editing"
  },
  {
    "page": "keyboard",
    "id": 42,
    "cap": "⌫",
    "name": "Delete",
    "spoken": "Delete",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "BSPC",
      "BACKSPACE",
      "BKSP"
    ],
    "group": "editing"
  },
  {
    "page": "keyboard",
    "id": 43,
    "cap": "⇥",
    "name": "Tab",
    "spoken": "Tab",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "TAB"
    ],
    "group": "editing"
  },
  {
    "page": "keyboard",
    "id": 44,
    "cap": "␣",
    "name": "Space",
    "spoken": "Space",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "SPACE",
      "SPC"
    ],
    "group": "editing"
  },
  {
    "page": "keyboard",
    "id": 45,
    "cap": "-",
    "name": "-",
    "spoken": "Minus",
    "shifted": "_",
    "shiftedSpoken": "Underscore",
    "shiftedZmk": "UNDER",
    "zmk": [
      "MINUS"
    ],
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 46,
    "cap": "=",
    "name": "=",
    "spoken": "Equals Sign",
    "shifted": "+",
    "shiftedSpoken": "Plus Sign",
    "shiftedZmk": "PLUS",
    "zmk": [
      "EQUAL",
      "EQL"
    ],
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 47,
    "cap": "[",
    "name": "[",
    "spoken": "Left Bracket",
    "shifted": "{",
    "shiftedSpoken": "Left Brace",
    "shiftedZmk": "LBRC",
    "zmk": [
      "LBKT",
      "LEFT_BRACKET"
    ],
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 48,
    "cap": "]",
    "name": "]",
    "spoken": "Right Bracket",
    "shifted": "}",
    "shiftedSpoken": "Right Brace",
    "shiftedZmk": "RBRC",
    "zmk": [
      "RBKT",
      "RIGHT_BRACKET"
    ],
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 49,
    "cap": "\\",
    "name": "\\",
    "spoken": "Backslash",
    "shifted": "|",
    "shiftedSpoken": "Vertical Bar",
    "shiftedZmk": "PIPE",
    "zmk": [
      "BSLH",
      "BACKSLASH"
    ],
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 50,
    "cap": "#",
    "name": "ISO #",
    "spoken": "ISO Number Sign",
    "shifted": "~",
    "shiftedSpoken": "Tilde",
    "shiftedZmk": "TILDE2",
    "zmk": [
      "NUHS",
      "NON_US_HASH"
    ],
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 51,
    "cap": ";",
    "name": ";",
    "spoken": "Semicolon",
    "shifted": ":",
    "shiftedSpoken": "Colon",
    "shiftedZmk": "COLON",
    "zmk": [
      "SEMI",
      "SEMICOLON",
      "SCLN"
    ],
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 52,
    "cap": "'",
    "name": "'",
    "spoken": "Apostrophe",
    "shifted": "\"",
    "shiftedSpoken": "Quotation Mark",
    "shiftedZmk": "DQT",
    "zmk": [
      "SQT",
      "SINGLE_QUOTE",
      "APOSTROPHE",
      "APOS",
      "QUOT"
    ],
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 53,
    "cap": "`",
    "name": "`",
    "spoken": "Grave Accent",
    "shifted": "~",
    "shiftedSpoken": "Tilde",
    "shiftedZmk": "TILDE",
    "zmk": [
      "GRAVE",
      "GRAV"
    ],
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 54,
    "cap": ",",
    "name": ",",
    "spoken": "Comma",
    "shifted": "<",
    "shiftedSpoken": "Less-Than Sign",
    "shiftedZmk": "LT",
    "zmk": [
      "COMMA",
      "CMMA"
    ],
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 55,
    "cap": ".",
    "name": ".",
    "spoken": "Period",
    "shifted": ">",
    "shiftedSpoken": "Greater-Than Sign",
    "shiftedZmk": "GT",
    "zmk": [
      "DOT",
      "PERIOD"
    ],
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 56,
    "cap": "/",
    "name": "/",
    "spoken": "Slash",
    "shifted": "?",
    "shiftedSpoken": "Question Mark",
    "shiftedZmk": "QMARK",
    "zmk": [
      "FSLH",
      "SLASH"
    ],
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 57,
    "cap": "⇪",
    "name": "Caps Lock",
    "spoken": "Caps Lock",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "CAPS",
      "CAPSLOCK",
      "CLCK"
    ],
    "group": "modifier"
  },
  {
    "page": "keyboard",
    "id": 58,
    "cap": "F1",
    "name": "F1",
    "spoken": "F1",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F1"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 59,
    "cap": "F2",
    "name": "F2",
    "spoken": "F2",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F2"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 60,
    "cap": "F3",
    "name": "F3",
    "spoken": "F3",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F3"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 61,
    "cap": "F4",
    "name": "F4",
    "spoken": "F4",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F4"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 62,
    "cap": "F5",
    "name": "F5",
    "spoken": "F5",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F5"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 63,
    "cap": "F6",
    "name": "F6",
    "spoken": "F6",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F6"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 64,
    "cap": "F7",
    "name": "F7",
    "spoken": "F7",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F7"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 65,
    "cap": "F8",
    "name": "F8",
    "spoken": "F8",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F8"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 66,
    "cap": "F9",
    "name": "F9",
    "spoken": "F9",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F9"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 67,
    "cap": "F10",
    "name": "F10",
    "spoken": "F10",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F10"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 68,
    "cap": "F11",
    "name": "F11",
    "spoken": "F11",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F11"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 69,
    "cap": "F12",
    "name": "F12",
    "spoken": "F12",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F12"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 70,
    "cap": "Print Screen",
    "name": "Print Screen",
    "spoken": "Print Screen",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "PSCRN",
      "PRINTSCREEN",
      "PRSC"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 71,
    "cap": "Scroll Lock",
    "name": "Scroll Lock",
    "spoken": "Scroll Lock",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "SLCK",
      "SCROLLLOCK",
      "SCLK"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 72,
    "cap": "Pause",
    "name": "Pause",
    "spoken": "Pause",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "PAUSE_BREAK",
      "PAUS"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 73,
    "cap": "Insert",
    "name": "Insert",
    "spoken": "Insert",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "INS",
      "INSERT"
    ],
    "group": "editing"
  },
  {
    "page": "keyboard",
    "id": 74,
    "cap": "↖",
    "name": "Home",
    "spoken": "Home",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "HOME"
    ],
    "group": "navigation"
  },
  {
    "page": "keyboard",
    "id": 75,
    "cap": "⇞",
    "name": "Page Up",
    "spoken": "Page Up",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "PG_UP",
      "PAGE_UP",
      "PGUP"
    ],
    "group": "navigation"
  },
  {
    "page": "keyboard",
    "id": 76,
    "cap": "⌦",
    "name": "Forward Delete",
    "spoken": "Forward Delete",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "DEL",
      "DELETE"
    ],
    "group": "editing"
  },
  {
    "page": "keyboard",
    "id": 77,
    "cap": "↘",
    "name": "End",
    "spoken": "End",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "END"
    ],
    "group": "navigation"
  },
  {
    "page": "keyboard",
    "id": 78,
    "cap": "⇟",
    "name": "Page Down",
    "spoken": "Page Down",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "PG_DN",
      "PAGE_DOWN",
      "PGDN"
    ],
    "group": "navigation"
  },
  {
    "page": "keyboard",
    "id": 79,
    "cap": "→",
    "name": "Right Arrow",
    "spoken": "Right Arrow",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "RIGHT",
      "RIGHT_ARROW",
      "RARW"
    ],
    "group": "navigation"
  },
  {
    "page": "keyboard",
    "id": 80,
    "cap": "←",
    "name": "Left Arrow",
    "spoken": "Left Arrow",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "LEFT",
      "LEFT_ARROW",
      "LARW"
    ],
    "group": "navigation"
  },
  {
    "page": "keyboard",
    "id": 81,
    "cap": "↓",
    "name": "Down Arrow",
    "spoken": "Down Arrow",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "DOWN",
      "DOWN_ARROW",
      "DARW"
    ],
    "group": "navigation"
  },
  {
    "page": "keyboard",
    "id": 82,
    "cap": "↑",
    "name": "Up Arrow",
    "spoken": "Up Arrow",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "UP",
      "UP_ARROW",
      "UARW"
    ],
    "group": "navigation"
  },
  {
    "page": "keyboard",
    "id": 83,
    "cap": "⌧",
    "name": "Clear",
    "spoken": "Clear",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_NUM",
      "KP_NUMLOCK",
      "KP_NLCK"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 84,
    "cap": "/",
    "name": "Keypad /",
    "spoken": "Keypad Slash",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_SLASH",
      "KP_DIVIDE",
      "KDIV"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 85,
    "cap": "*",
    "name": "Keypad *",
    "spoken": "Keypad Asterisk",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_MULTIPLY",
      "KP_ASTERISK",
      "KMLT"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 86,
    "cap": "-",
    "name": "Keypad -",
    "spoken": "Keypad Minus",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_MINUS",
      "KP_SUBTRACT",
      "KMIN"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 87,
    "cap": "+",
    "name": "Keypad +",
    "spoken": "Keypad Plus Sign",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_PLUS",
      "KPLS"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 88,
    "cap": "⌤",
    "name": "Keypad Enter",
    "spoken": "Keypad Enter",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_ENTER"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 89,
    "cap": "1",
    "name": "Keypad 1",
    "spoken": "Keypad 1",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_N1",
      "KP_NUMBER_1"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 90,
    "cap": "2",
    "name": "Keypad 2",
    "spoken": "Keypad 2",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_N2",
      "KP_NUMBER_2"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 91,
    "cap": "3",
    "name": "Keypad 3",
    "spoken": "Keypad 3",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_N3",
      "KP_NUMBER_3"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 92,
    "cap": "4",
    "name": "Keypad 4",
    "spoken": "Keypad 4",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_N4",
      "KP_NUMBER_4"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 93,
    "cap": "5",
    "name": "Keypad 5",
    "spoken": "Keypad 5",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_N5",
      "KP_NUMBER_5"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 94,
    "cap": "6",
    "name": "Keypad 6",
    "spoken": "Keypad 6",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_N6",
      "KP_NUMBER_6"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 95,
    "cap": "7",
    "name": "Keypad 7",
    "spoken": "Keypad 7",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_N7",
      "KP_NUMBER_7"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 96,
    "cap": "8",
    "name": "Keypad 8",
    "spoken": "Keypad 8",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_N8",
      "KP_NUMBER_8"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 97,
    "cap": "9",
    "name": "Keypad 9",
    "spoken": "Keypad 9",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_N9",
      "KP_NUMBER_9"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 98,
    "cap": "0",
    "name": "Keypad 0",
    "spoken": "Keypad 0",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_N0",
      "KP_NUMBER_0"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 99,
    "cap": ".",
    "name": "Keypad .",
    "spoken": "Keypad Period",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_DOT"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 100,
    "cap": "\\",
    "name": "ISO \\",
    "spoken": "ISO Backslash",
    "shifted": "|",
    "shiftedSpoken": "Vertical Bar",
    "shiftedZmk": "PIPE2",
    "zmk": [
      "NUBS",
      "NON_US_BACKSLASH",
      "NON_US_BSLH"
    ],
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 101,
    "cap": "Menu",
    "name": "Menu",
    "spoken": "Menu",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_APP",
      "K_APPLICATION",
      "K_CONTEXT_MENU",
      "K_CMENU",
      "GUI"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 102,
    "cap": "Power",
    "name": "Power",
    "spoken": "Power",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_PWR",
      "K_POWER"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 103,
    "cap": "=",
    "name": "Keypad =",
    "spoken": "Keypad Equals Sign",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_EQUAL"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 104,
    "cap": "F13",
    "name": "F13",
    "spoken": "F13",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F13"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 105,
    "cap": "F14",
    "name": "F14",
    "spoken": "F14",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F14"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 106,
    "cap": "F15",
    "name": "F15",
    "spoken": "F15",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F15"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 107,
    "cap": "F16",
    "name": "F16",
    "spoken": "F16",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F16"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 108,
    "cap": "F17",
    "name": "F17",
    "spoken": "F17",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F17"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 109,
    "cap": "F18",
    "name": "F18",
    "spoken": "F18",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F18"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 110,
    "cap": "F19",
    "name": "F19",
    "spoken": "F19",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F19"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 111,
    "cap": "F20",
    "name": "F20",
    "spoken": "F20",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F20"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 112,
    "cap": "F21",
    "name": "F21",
    "spoken": "F21",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F21"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 113,
    "cap": "F22",
    "name": "F22",
    "spoken": "F22",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F22"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 114,
    "cap": "F23",
    "name": "F23",
    "spoken": "F23",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F23"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 115,
    "cap": "F24",
    "name": "F24",
    "spoken": "F24",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "F24"
    ],
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 116,
    "cap": "Execute",
    "name": "Execute",
    "spoken": "Execute",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_EXEC",
      "K_EXECUTE"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 117,
    "cap": "Help",
    "name": "Help",
    "spoken": "Help",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_HELP"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 118,
    "cap": "Menu",
    "name": "Menu",
    "spoken": "Menu",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_MENU"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 119,
    "cap": "Select",
    "name": "Select",
    "spoken": "Select",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_SELECT"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 120,
    "cap": "Stop",
    "name": "Stop",
    "spoken": "Stop",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_STOP"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 121,
    "cap": "Again",
    "name": "Again",
    "spoken": "Again",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_REDO",
      "K_AGAIN"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 122,
    "cap": "Undo",
    "name": "Undo",
    "spoken": "Undo",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_UNDO",
      "UNDO"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 123,
    "cap": "Cut",
    "name": "Cut",
    "spoken": "Cut",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_CUT",
      "CUT"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 124,
    "cap": "Copy",
    "name": "Copy",
    "spoken": "Copy",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_COPY",
      "COPY"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 125,
    "cap": "Paste",
    "name": "Paste",
    "spoken": "Paste",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_PASTE",
      "PSTE"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 126,
    "cap": "Find",
    "name": "Find",
    "spoken": "Find",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_FIND"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 127,
    "cap": "Mute",
    "name": "Mute",
    "spoken": "Mute",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_MUTE"
    ],
    "group": "media"
  },
  {
    "page": "keyboard",
    "id": 128,
    "cap": "Volume Up",
    "name": "Volume Up",
    "spoken": "Volume Up",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_VOL_UP",
      "K_VOLUME_UP",
      "VOLU"
    ],
    "group": "media"
  },
  {
    "page": "keyboard",
    "id": 129,
    "cap": "Volume Down",
    "name": "Volume Down",
    "spoken": "Volume Down",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_VOL_DN",
      "K_VOLUME_DOWN",
      "VOLD"
    ],
    "group": "media"
  },
  {
    "page": "keyboard",
    "id": 130,
    "cap": "Locking Caps Lock",
    "name": "Locking Caps Lock",
    "spoken": "Locking Caps Lock",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "LCAPS",
      "LOCKING_CAPS"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 131,
    "cap": "Locking Num Lock",
    "name": "Locking Num Lock",
    "spoken": "Locking Num Lock",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "LNLCK",
      "LOCKING_NUM"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 132,
    "cap": "Locking Scroll Lock",
    "name": "Locking Scroll Lock",
    "spoken": "Locking Scroll Lock",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "LSLCK",
      "LOCKING_SCROLL"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 133,
    "cap": "Keypad ,",
    "name": "Keypad ,",
    "spoken": "Keypad Comma",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_COMMA"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 134,
    "cap": "Keypad = (Equal) AS/400",
    "name": "Keypad = (Equal) AS/400",
    "spoken": "Keypad Equals Sign (Equal) AS/400",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_EQUAL_AS400"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 135,
    "cap": "_",
    "name": "Ro",
    "spoken": "Ro",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "INT1",
      "INTERNATIONAL_1",
      "INT_RO"
    ],
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 136,
    "cap": "International 2",
    "name": "International 2",
    "spoken": "International 2",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "INT2",
      "INTERNATIONAL_2",
      "INT_KATAKANAHIRAGANA",
      "INT_KANA"
    ],
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 137,
    "cap": "¥",
    "name": "Yen",
    "spoken": "Yen",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "INT3",
      "INTERNATIONAL_3",
      "INT_YEN"
    ],
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 138,
    "cap": "International 4",
    "name": "International 4",
    "spoken": "International 4",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "INT4",
      "INTERNATIONAL_4",
      "INT_HENKAN"
    ],
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 139,
    "cap": "International 5",
    "name": "International 5",
    "spoken": "International 5",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "INT5",
      "INTERNATIONAL_5",
      "INT_MUHENKAN"
    ],
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 140,
    "cap": "International 6",
    "name": "International 6",
    "spoken": "International 6",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "INT6",
      "INTERNATIONAL_6",
      "INT_KPJPCOMMA"
    ],
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 141,
    "cap": "International 7",
    "name": "International 7",
    "spoken": "International 7",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "INT7",
      "INTERNATIONAL_7"
    ],
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 142,
    "cap": "International 8",
    "name": "International 8",
    "spoken": "International 8",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "INT8",
      "INTERNATIONAL_8"
    ],
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 143,
    "cap": "International 9",
    "name": "International 9",
    "spoken": "International 9",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "INT9",
      "INTERNATIONAL_9"
    ],
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 144,
    "cap": "かな",
    "name": "Kana",
    "spoken": "Kana",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "LANG1",
      "LANGUAGE_1",
      "LANG_HANGEUL"
    ],
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 145,
    "cap": "英数",
    "name": "Eisu",
    "spoken": "Eisu",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "LANG2",
      "LANGUAGE_2",
      "LANG_HANJA"
    ],
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 146,
    "cap": "Language 3",
    "name": "Language 3",
    "spoken": "Language 3",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "LANG3",
      "LANGUAGE_3",
      "LANG_KATAKANA"
    ],
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 147,
    "cap": "Language 4",
    "name": "Language 4",
    "spoken": "Language 4",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "LANG4",
      "LANGUAGE_4",
      "LANG_HIRAGANA"
    ],
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 148,
    "cap": "Language 5",
    "name": "Language 5",
    "spoken": "Language 5",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "LANG5",
      "LANGUAGE_5",
      "LANG_ZENKAKUHANKAKU"
    ],
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 149,
    "cap": "Language 6",
    "name": "Language 6",
    "spoken": "Language 6",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "LANG6",
      "LANGUAGE_6"
    ],
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 150,
    "cap": "Language 7",
    "name": "Language 7",
    "spoken": "Language 7",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "LANG7",
      "LANGUAGE_7"
    ],
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 151,
    "cap": "Language 8",
    "name": "Language 8",
    "spoken": "Language 8",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "LANG8",
      "LANGUAGE_8"
    ],
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 152,
    "cap": "Language 9",
    "name": "Language 9",
    "spoken": "Language 9",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "LANG9",
      "LANGUAGE_9"
    ],
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 153,
    "cap": "Alternate Erase",
    "name": "Alternate Erase",
    "spoken": "Alternate Erase",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "ALT_ERASE"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 154,
    "cap": "SysReq/Attention",
    "name": "SysReq/Attention",
    "spoken": "SysReq/Attention",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "SYSREQ",
      "ATTENTION"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 155,
    "cap": "Cancel",
    "name": "Cancel",
    "spoken": "Cancel",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_CANCEL"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 156,
    "cap": "Clear",
    "name": "Clear",
    "spoken": "Clear",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "CLEAR"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 157,
    "cap": "Prior",
    "name": "Prior",
    "spoken": "Prior",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "PRIOR"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 158,
    "cap": "Return",
    "name": "Return",
    "spoken": "Return",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "RET2",
      "RETURN2"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 159,
    "cap": "Separator",
    "name": "Separator",
    "spoken": "Separator",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "SEPARATOR"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 160,
    "cap": "Out",
    "name": "Out",
    "spoken": "Out",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "OUT"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 161,
    "cap": "Oper",
    "name": "Oper",
    "spoken": "Oper",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "OPER"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 162,
    "cap": "Clear/Again",
    "name": "Clear/Again",
    "spoken": "Clear/Again",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "CLEAR_AGAIN"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 163,
    "cap": "CrSel/Props",
    "name": "CrSel/Props",
    "spoken": "CrSel/Props",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "CRSEL"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 164,
    "cap": "ExSel",
    "name": "ExSel",
    "spoken": "ExSel",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "EXSEL"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 180,
    "cap": "Currency Unit",
    "name": "Currency Unit",
    "spoken": "Currency Unit",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "CURU"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 182,
    "cap": "Keypad (",
    "name": "Keypad (",
    "spoken": "Keypad Left Parenthesis",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_LPAR",
      "KP_LEFT_PARENTHESIS"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 183,
    "cap": "Keypad )",
    "name": "Keypad )",
    "spoken": "Keypad Right Parenthesis",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_RPAR",
      "KP_RIGHT_PARENTHESIS"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 205,
    "cap": "Keypad Space",
    "name": "Keypad Space",
    "spoken": "Keypad Space",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KSPC"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 216,
    "cap": "Keypad Clear",
    "name": "Keypad Clear",
    "spoken": "Keypad Clear",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "KP_CLEAR"
    ],
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 224,
    "cap": "⌃",
    "name": "Control",
    "spoken": "Control",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "LCTRL",
      "LEFT_CONTROL",
      "LCTL"
    ],
    "group": "modifier"
  },
  {
    "page": "keyboard",
    "id": 225,
    "cap": "⇧",
    "name": "Shift",
    "spoken": "Shift",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "LSHFT",
      "LEFT_SHIFT",
      "LSHIFT",
      "LSFT"
    ],
    "group": "modifier"
  },
  {
    "page": "keyboard",
    "id": 226,
    "cap": "⌥",
    "name": "Option",
    "spoken": "Option",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "LALT",
      "LEFT_ALT"
    ],
    "group": "modifier"
  },
  {
    "page": "keyboard",
    "id": 227,
    "cap": "⌘",
    "name": "Command",
    "spoken": "Command",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "LGUI",
      "LEFT_GUI",
      "LEFT_WIN",
      "LWIN",
      "LEFT_COMMAND",
      "LCMD",
      "LEFT_META",
      "LMETA"
    ],
    "group": "modifier"
  },
  {
    "page": "keyboard",
    "id": 232,
    "cap": "Play/Pause",
    "name": "Play/Pause",
    "spoken": "Play/Pause",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_PP",
      "K_PLAY_PAUSE"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 233,
    "cap": "Stop",
    "name": "Stop",
    "spoken": "Stop",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_STOP2"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 234,
    "cap": "Previous",
    "name": "Previous",
    "spoken": "Previous",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_PREV",
      "K_PREVIOUS"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 235,
    "cap": "Next",
    "name": "Next",
    "spoken": "Next",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_NEXT"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 236,
    "cap": "Eject",
    "name": "Eject",
    "spoken": "Eject",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_EJECT"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 237,
    "cap": "Volume Up",
    "name": "Volume Up",
    "spoken": "Volume Up",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_VOL_UP2",
      "K_VOLUME_UP2"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 238,
    "cap": "Volume Down",
    "name": "Volume Down",
    "spoken": "Volume Down",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_VOL_DN2",
      "K_VOLUME_DOWN2"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 239,
    "cap": "Mute",
    "name": "Mute",
    "spoken": "Mute",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_MUTE2"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 240,
    "cap": "WWW",
    "name": "WWW",
    "spoken": "WWW",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_WWW"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 241,
    "cap": "Back",
    "name": "Back",
    "spoken": "Back",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_BACK"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 242,
    "cap": "Forward",
    "name": "Forward",
    "spoken": "Forward",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_FORWARD"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 243,
    "cap": "Stop",
    "name": "Stop",
    "spoken": "Stop",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_STOP3"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 244,
    "cap": "Find",
    "name": "Find",
    "spoken": "Find",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_FIND2"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 245,
    "cap": "Scroll Up",
    "name": "Scroll Up",
    "spoken": "Scroll Up",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_SCROLL_UP"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 246,
    "cap": "Scroll Down",
    "name": "Scroll Down",
    "spoken": "Scroll Down",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_SCROLL_DOWN"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 247,
    "cap": "Edit",
    "name": "Edit",
    "spoken": "Edit",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_EDIT"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 248,
    "cap": "Sleep",
    "name": "Sleep",
    "spoken": "Sleep",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_SLEEP"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 249,
    "cap": "Lock",
    "name": "Lock",
    "spoken": "Lock",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_LOCK",
      "K_SCREENSAVER",
      "K_COFFEE"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 250,
    "cap": "Refresh",
    "name": "Refresh",
    "spoken": "Refresh",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_REFRESH"
    ],
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 251,
    "cap": "Calculator",
    "name": "Calculator",
    "spoken": "Calculator",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "K_CALC",
      "K_CALCULATOR"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 48,
    "cap": "Power",
    "name": "Power",
    "spoken": "Power",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_PWR",
      "C_POWER"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 49,
    "cap": "Reset",
    "name": "Reset",
    "spoken": "Reset",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_RESET"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 50,
    "cap": "Sleep",
    "name": "Sleep",
    "spoken": "Sleep",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_SLEEP"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 52,
    "cap": "Sleep Mode",
    "name": "Sleep Mode",
    "spoken": "Sleep Mode",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_SLEEP_MODE"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 64,
    "cap": "Menu",
    "name": "Menu",
    "spoken": "Menu",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MENU"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 65,
    "cap": "Menu Pick",
    "name": "Menu Pick",
    "spoken": "Menu Pick",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MENU_PICK",
      "C_MENU_SELECT"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 66,
    "cap": "Menu Up",
    "name": "Menu Up",
    "spoken": "Menu Up",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MENU_UP"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 67,
    "cap": "Menu Down",
    "name": "Menu Down",
    "spoken": "Menu Down",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MENU_DOWN"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 68,
    "cap": "Menu Left",
    "name": "Menu Left",
    "spoken": "Menu Left",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MENU_LEFT"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 69,
    "cap": "Menu Right",
    "name": "Menu Right",
    "spoken": "Menu Right",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MENU_RIGHT"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 70,
    "cap": "Menu Escape",
    "name": "Menu Escape",
    "spoken": "Menu Escape",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MENU_ESC",
      "C_MENU_ESCAPE"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 71,
    "cap": "Menu Value Increase",
    "name": "Menu Value Increase",
    "spoken": "Menu Value Increase",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MENU_INC",
      "C_MENU_INCREASE"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 72,
    "cap": "Menu Value Decrease",
    "name": "Menu Value Decrease",
    "spoken": "Menu Value Decrease",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MENU_DEC",
      "C_MENU_DECREASE"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 96,
    "cap": "Data On Screen",
    "name": "Data On Screen",
    "spoken": "Data On Screen",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_DATA_ON_SCREEN"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 97,
    "cap": "Closed Caption",
    "name": "Closed Caption",
    "spoken": "Closed Caption",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_CAPTIONS",
      "C_SUBTITLES"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 101,
    "cap": "Snapshot",
    "name": "Snapshot",
    "spoken": "Snapshot",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_SNAPSHOT"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 103,
    "cap": "Picture-in-Picture Toggle",
    "name": "Picture-in-Picture Toggle",
    "spoken": "Picture-in-Picture Toggle",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_PIP"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 105,
    "cap": "Red Menu Button",
    "name": "Red Menu Button",
    "spoken": "Red Menu Button",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_RED",
      "C_RED_BUTTON"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 106,
    "cap": "Green Menu Button",
    "name": "Green Menu Button",
    "spoken": "Green Menu Button",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_GREEN",
      "C_GREEN_BUTTON"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 107,
    "cap": "Blue Menu Button",
    "name": "Blue Menu Button",
    "spoken": "Blue Menu Button",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_BLUE",
      "C_BLUE_BUTTON"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 108,
    "cap": "Yellow Menu Button",
    "name": "Yellow Menu Button",
    "spoken": "Yellow Menu Button",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_YELLOW",
      "C_YELLOW_BUTTON"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 109,
    "cap": "Aspect",
    "name": "Aspect",
    "spoken": "Aspect",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_ASPECT"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 111,
    "cap": "Brightness Up",
    "name": "Brightness Up",
    "spoken": "Brightness Up",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_BRI_UP",
      "C_BRIGHTNESS_INC",
      "C_BRI_INC"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 112,
    "cap": "Brightness Down",
    "name": "Brightness Down",
    "spoken": "Brightness Down",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_BRI_DN",
      "C_BRIGHTNESS_DEC",
      "C_BRI_DEC"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 114,
    "cap": "Display Backlight Toggle",
    "name": "Display Backlight Toggle",
    "spoken": "Display Backlight Toggle",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_BKLT_TOG",
      "C_BACKLIGHT_TOGGLE"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 115,
    "cap": "Display Set Brightness to Minimum",
    "name": "Display Set Brightness to Minimum",
    "spoken": "Display Set Brightness to Minimum",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_BRI_MIN",
      "C_BRIGHTNESS_MINIMUM"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 116,
    "cap": "Display Set Brightness to Maximum",
    "name": "Display Set Brightness to Maximum",
    "spoken": "Display Set Brightness to Maximum",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_BRI_MAX",
      "C_BRIGHTNESS_MAXIMUM"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 117,
    "cap": "Display Set Auto Brightness",
    "name": "Display Set Auto Brightness",
    "spoken": "Display Set Auto Brightness",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_BRI_AUTO",
      "C_BRIGHTNESS_AUTO"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 130,
    "cap": "Mode Step",
    "name": "Mode Step",
    "spoken": "Mode Step",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MODE_STEP",
      "C_MEDIA_STEP"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 131,
    "cap": "Recall Last",
    "name": "Recall Last",
    "spoken": "Recall Last",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_CHAN_LAST",
      "C_RECALL_LAST"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 136,
    "cap": "Media Select Computer",
    "name": "Media Select Computer",
    "spoken": "Media Select Computer",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MEDIA_COMPUTER"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 137,
    "cap": "Media Select TV",
    "name": "Media Select TV",
    "spoken": "Media Select TV",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MEDIA_TV"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 138,
    "cap": "Media Select WWW",
    "name": "Media Select WWW",
    "spoken": "Media Select WWW",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MEDIA_WWW"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 139,
    "cap": "Media Select DVD",
    "name": "Media Select DVD",
    "spoken": "Media Select DVD",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MEDIA_DVD"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 140,
    "cap": "Media Select Telephone",
    "name": "Media Select Telephone",
    "spoken": "Media Select Telephone",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MEDIA_PHONE"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 141,
    "cap": "Media Select Program Guide",
    "name": "Media Select Program Guide",
    "spoken": "Media Select Program Guide",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MEDIA_GUIDE"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 142,
    "cap": "Media Select Video Phone",
    "name": "Media Select Video Phone",
    "spoken": "Media Select Video Phone",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MEDIA_VIDEOPHONE"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 143,
    "cap": "Media Select Games",
    "name": "Media Select Games",
    "spoken": "Media Select Games",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MEDIA_GAMES"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 144,
    "cap": "Media Select Messages",
    "name": "Media Select Messages",
    "spoken": "Media Select Messages",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MEDIA_MESSAGES"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 145,
    "cap": "Media Select CD",
    "name": "Media Select CD",
    "spoken": "Media Select CD",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MEDIA_CD"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 146,
    "cap": "Media Select VCR",
    "name": "Media Select VCR",
    "spoken": "Media Select VCR",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MEDIA_VCR"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 147,
    "cap": "Media Select Tuner",
    "name": "Media Select Tuner",
    "spoken": "Media Select Tuner",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MEDIA_TUNER"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 148,
    "cap": "Quit",
    "name": "Quit",
    "spoken": "Quit",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_QUIT"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 149,
    "cap": "Help",
    "name": "Help",
    "spoken": "Help",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_HELP"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 150,
    "cap": "Media Select Tape",
    "name": "Media Select Tape",
    "spoken": "Media Select Tape",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MEDIA_TAPE"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 151,
    "cap": "Media Select Cable",
    "name": "Media Select Cable",
    "spoken": "Media Select Cable",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MEDIA_CABLE"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 152,
    "cap": "Media Select Satellite",
    "name": "Media Select Satellite",
    "spoken": "Media Select Satellite",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MEDIA_SATELLITE"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 154,
    "cap": "Media Select Home",
    "name": "Media Select Home",
    "spoken": "Media Select Home",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MEDIA_HOME"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 156,
    "cap": "Channel Increment",
    "name": "Channel Increment",
    "spoken": "Channel Increment",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_CHAN_INC",
      "C_CHANNEL_INC"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 157,
    "cap": "Channel Decrement",
    "name": "Channel Decrement",
    "spoken": "Channel Decrement",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_CHAN_DEC",
      "C_CHANNEL_DEC"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 160,
    "cap": "VCR Plus",
    "name": "VCR Plus",
    "spoken": "VCR Plus",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MEDIA_VCR_PLUS"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 176,
    "cap": "Play",
    "name": "Play",
    "spoken": "Play",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_PLAY"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 177,
    "cap": "Pause",
    "name": "Pause",
    "spoken": "Pause",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_PAUSE"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 178,
    "cap": "Record",
    "name": "Record",
    "spoken": "Record",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_REC",
      "C_RECORD"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 179,
    "cap": "Fast Forward",
    "name": "Fast Forward",
    "spoken": "Fast Forward",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_FF",
      "C_FAST_FORWARD"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 180,
    "cap": "Rewind",
    "name": "Rewind",
    "spoken": "Rewind",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_RW",
      "C_REWIND"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 181,
    "cap": "Next Track",
    "name": "Next Track",
    "spoken": "Next Track",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_NEXT",
      "M_NEXT"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 182,
    "cap": "Previous Track",
    "name": "Previous Track",
    "spoken": "Previous Track",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_PREV",
      "C_PREVIOUS",
      "M_PREV"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 183,
    "cap": "Stop",
    "name": "Stop",
    "spoken": "Stop",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_STOP",
      "M_STOP"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 184,
    "cap": "Eject",
    "name": "Eject",
    "spoken": "Eject",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_EJECT",
      "M_EJCT"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 185,
    "cap": "Random Play",
    "name": "Random Play",
    "spoken": "Random Play",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_SHUFFLE",
      "C_RANDOM_PLAY"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 188,
    "cap": "Repeat",
    "name": "Repeat",
    "spoken": "Repeat",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_REPEAT"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 191,
    "cap": "Slow Tracking",
    "name": "Slow Tracking",
    "spoken": "Slow Tracking",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_SLOW2",
      "C_SLOW_TRACKING"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 204,
    "cap": "Stop/Eject",
    "name": "Stop/Eject",
    "spoken": "Stop/Eject",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_STOP_EJECT"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 205,
    "cap": "Play/Pause",
    "name": "Play/Pause",
    "spoken": "Play/Pause",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_PP",
      "C_PLAY_PAUSE",
      "M_PLAY"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 207,
    "cap": "Dictation",
    "name": "Dictation",
    "spoken": "Dictation",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_VOICE_COMMAND"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 226,
    "cap": "Mute",
    "name": "Mute",
    "spoken": "Mute",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_MUTE",
      "M_MUTE"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 229,
    "cap": "Bass Boost",
    "name": "Bass Boost",
    "spoken": "Bass Boost",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_BASS_BOOST"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 233,
    "cap": "Volume Up",
    "name": "Volume Up",
    "spoken": "Volume Up",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_VOL_UP",
      "C_VOLUME_UP",
      "M_VOLU"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 234,
    "cap": "Volume Down",
    "name": "Volume Down",
    "spoken": "Volume Down",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_VOL_DN",
      "C_VOLUME_DOWN",
      "M_VOLD"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 245,
    "cap": "Slow",
    "name": "Slow",
    "spoken": "Slow",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_SLOW"
    ],
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 371,
    "cap": "Alternate Audio Increment",
    "name": "Alternate Audio Increment",
    "spoken": "Alternate Audio Increment",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_ALT_AUDIO_INC",
      "C_ALTERNATE_AUDIO_INCREMENT"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 387,
    "cap": "Consumer Control Configuration",
    "name": "Consumer Control Configuration",
    "spoken": "Consumer Control Configuration",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_CCC"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 388,
    "cap": "Word Processor",
    "name": "Word Processor",
    "spoken": "Word Processor",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_WORD"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 389,
    "cap": "Text Editor",
    "name": "Text Editor",
    "spoken": "Text Editor",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_TEXT_EDITOR"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 390,
    "cap": "Spreadsheet",
    "name": "Spreadsheet",
    "spoken": "Spreadsheet",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_SHEET",
      "C_AL_SPREADSHEET"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 391,
    "cap": "Graphics Editor",
    "name": "Graphics Editor",
    "spoken": "Graphics Editor",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_GRAPHICS_EDITOR"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 392,
    "cap": "Presentation App",
    "name": "Presentation App",
    "spoken": "Presentation App",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_PRESENTATION"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 393,
    "cap": "Database App",
    "name": "Database App",
    "spoken": "Database App",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_DB",
      "C_AL_DATABASE"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 394,
    "cap": "Email Reader",
    "name": "Email Reader",
    "spoken": "Email Reader",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_MAIL",
      "C_AL_EMAIL"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 395,
    "cap": "Newsreader",
    "name": "Newsreader",
    "spoken": "Newsreader",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_NEWS"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 396,
    "cap": "Voicemail",
    "name": "Voicemail",
    "spoken": "Voicemail",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_VOICEMAIL"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 397,
    "cap": "Contacts/Address Book",
    "name": "Contacts/Address Book",
    "spoken": "Contacts/Address Book",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_CONTACTS",
      "C_AL_ADDRESS_BOOK"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 398,
    "cap": "Calendar/Schedule",
    "name": "Calendar/Schedule",
    "spoken": "Calendar/Schedule",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_CAL",
      "C_AL_CALENDAR"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 399,
    "cap": "Task/Project Manager",
    "name": "Task/Project Manager",
    "spoken": "Task/Project Manager",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_TASK_MANAGER"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 400,
    "cap": "Log/Journal/Timecard",
    "name": "Log/Journal/Timecard",
    "spoken": "Log/Journal/Timecard",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_JOURNAL"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 401,
    "cap": "Checkbook/Finance",
    "name": "Checkbook/Finance",
    "spoken": "Checkbook/Finance",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_FINANCE"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 402,
    "cap": "Calculator",
    "name": "Calculator",
    "spoken": "Calculator",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_CALC",
      "C_AL_CALCULATOR"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 403,
    "cap": "A/V Capture/Playback",
    "name": "A/V Capture/Playback",
    "spoken": "A/V Capture/Playback",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_AV_CAPTURE_PLAYBACK"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 404,
    "cap": "Local Machine Browser",
    "name": "Local Machine Browser",
    "spoken": "Local Machine Browser",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_MY_COMPUTER"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 406,
    "cap": "Internet Browser",
    "name": "Internet Browser",
    "spoken": "Internet Browser",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_WWW"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 409,
    "cap": "Network Chat",
    "name": "Network Chat",
    "spoken": "Network Chat",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_CHAT",
      "C_AL_NETWORK_CHAT"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 412,
    "cap": "Logoff",
    "name": "Logoff",
    "spoken": "Logoff",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_LOGOFF"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 414,
    "cap": "Terminal Lock/Screensaver",
    "name": "Terminal Lock/Screensaver",
    "spoken": "Terminal Lock/Screensaver",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_LOCK",
      "C_AL_SCREENSAVER",
      "C_AL_COFFEE"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 415,
    "cap": "Control Panel",
    "name": "Control Panel",
    "spoken": "Control Panel",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_CONTROL_PANEL"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 418,
    "cap": "Select Task/Application",
    "name": "Select Task/Application",
    "spoken": "Select Task/Application",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_SELECT_TASK"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 419,
    "cap": "Next Task/Application",
    "name": "Next Task/Application",
    "spoken": "Next Task/Application",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_NEXT_TASK"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 420,
    "cap": "Previous Task/Application",
    "name": "Previous Task/Application",
    "spoken": "Previous Task/Application",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_PREV_TASK",
      "C_AL_PREVIOUS_TASK"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 422,
    "cap": "Integrated Help Center",
    "name": "Integrated Help Center",
    "spoken": "Integrated Help Center",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_HELP"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 423,
    "cap": "Documents",
    "name": "Documents",
    "spoken": "Documents",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_DOCS",
      "C_AL_DOCUMENTS"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 427,
    "cap": "Spell Check",
    "name": "Spell Check",
    "spoken": "Spell Check",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_SPELL",
      "C_AL_SPELLCHECK"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 430,
    "cap": "Keyboard Layout",
    "name": "Keyboard Layout",
    "spoken": "Keyboard Layout",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_KEYBOARD_LAYOUT"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 433,
    "cap": "Screen Saver",
    "name": "Screen Saver",
    "spoken": "Screen Saver",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_SCREEN_SAVER"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 436,
    "cap": "File Browser",
    "name": "File Browser",
    "spoken": "File Browser",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_FILES",
      "C_AL_FILE_BROWSER"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 438,
    "cap": "Image Browser",
    "name": "Image Browser",
    "spoken": "Image Browser",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_IMAGES",
      "C_AL_IMAGE_BROWSER"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 439,
    "cap": "Audio Browser",
    "name": "Audio Browser",
    "spoken": "Audio Browser",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_AUDIO",
      "C_AL_AUDIO_BROWSER",
      "C_AL_MUSIC"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 440,
    "cap": "Movie Browser",
    "name": "Movie Browser",
    "spoken": "Movie Browser",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_MOVIES",
      "C_AL_MOVIE_BROWSER"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 444,
    "cap": "Instant Messaging",
    "name": "Instant Messaging",
    "spoken": "Instant Messaging",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_IM",
      "C_AL_INSTANT_MESSAGING"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 445,
    "cap": "OEM Features/Tips/Tutorial Browser",
    "name": "OEM Features/Tips/Tutorial Browser",
    "spoken": "OEM Features/Tips/Tutorial Browser",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AL_TIPS",
      "C_AL_OEM_FEATURES",
      "C_AL_TUTORIAL"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 513,
    "cap": "New",
    "name": "New",
    "spoken": "New",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_NEW"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 514,
    "cap": "Open",
    "name": "Open",
    "spoken": "Open",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_OPEN"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 515,
    "cap": "Close",
    "name": "Close",
    "spoken": "Close",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_CLOSE"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 516,
    "cap": "Exit",
    "name": "Exit",
    "spoken": "Exit",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_EXIT"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 519,
    "cap": "Save",
    "name": "Save",
    "spoken": "Save",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_SAVE"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 520,
    "cap": "Print",
    "name": "Print",
    "spoken": "Print",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_PRINT"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 521,
    "cap": "Properties",
    "name": "Properties",
    "spoken": "Properties",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_PROPS",
      "C_AC_PROPERTIES"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 538,
    "cap": "Undo",
    "name": "Undo",
    "spoken": "Undo",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_UNDO"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 539,
    "cap": "Copy",
    "name": "Copy",
    "spoken": "Copy",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_COPY"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 540,
    "cap": "Cut",
    "name": "Cut",
    "spoken": "Cut",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_CUT"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 541,
    "cap": "Paste",
    "name": "Paste",
    "spoken": "Paste",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_PASTE"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 543,
    "cap": "Find",
    "name": "Find",
    "spoken": "Find",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_FIND"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 545,
    "cap": "Spotlight",
    "name": "Spotlight",
    "spoken": "Spotlight",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_SEARCH"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 546,
    "cap": "Go To",
    "name": "Go To",
    "spoken": "Go To",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_GOTO"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 547,
    "cap": "Home",
    "name": "Home",
    "spoken": "Home",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_HOME"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 548,
    "cap": "Back",
    "name": "Back",
    "spoken": "Back",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_BACK"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 549,
    "cap": "Forward",
    "name": "Forward",
    "spoken": "Forward",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_FORWARD"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 550,
    "cap": "Stop",
    "name": "Stop",
    "spoken": "Stop",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_STOP"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 551,
    "cap": "Refresh",
    "name": "Refresh",
    "spoken": "Refresh",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_REFRESH"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 554,
    "cap": "Bookmarks",
    "name": "Bookmarks",
    "spoken": "Bookmarks",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_BOOKMARKS",
      "C_AC_FAVORITES",
      "C_AC_FAVOURITES"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 557,
    "cap": "Zoom In",
    "name": "Zoom In",
    "spoken": "Zoom In",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_ZOOM_IN"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 558,
    "cap": "Zoom Out",
    "name": "Zoom Out",
    "spoken": "Zoom Out",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_ZOOM_OUT"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 559,
    "cap": "Zoom",
    "name": "Zoom",
    "spoken": "Zoom",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_ZOOM"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 562,
    "cap": "View Toggle",
    "name": "View Toggle",
    "spoken": "View Toggle",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_VIEW_TOGGLE"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 563,
    "cap": "Scroll Up",
    "name": "Scroll Up",
    "spoken": "Scroll Up",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_SCROLL_UP"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 564,
    "cap": "Scroll Down",
    "name": "Scroll Down",
    "spoken": "Scroll Down",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_SCROLL_DOWN"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 573,
    "cap": "Edit",
    "name": "Edit",
    "spoken": "Edit",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_EDIT"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 607,
    "cap": "Cancel",
    "name": "Cancel",
    "spoken": "Cancel",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_CANCEL"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 617,
    "cap": "Insert Mode",
    "name": "Insert Mode",
    "spoken": "Insert Mode",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_INS",
      "C_AC_INSERT"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 618,
    "cap": "Delete",
    "name": "Delete",
    "spoken": "Delete",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_DEL"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 633,
    "cap": "Redo/Repeat",
    "name": "Redo/Repeat",
    "spoken": "Redo/Repeat",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_REDO"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 649,
    "cap": "Reply",
    "name": "Reply",
    "spoken": "Reply",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_REPLY"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 651,
    "cap": "Forward Msg",
    "name": "Forward Msg",
    "spoken": "Forward Msg",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_FORWARD_MAIL"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 652,
    "cap": "Send",
    "name": "Send",
    "spoken": "Send",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_SEND"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 669,
    "cap": "🌐",
    "name": "Globe",
    "spoken": "Globe",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "GLOBE",
      "C_AC_NEXT_KEYBOARD_LAYOUT_SELECT"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 671,
    "cap": "Mission Control",
    "name": "Mission Control",
    "spoken": "Mission Control",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_DESKTOP_SHOW_ALL_WINDOWS"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 674,
    "cap": "Apps",
    "name": "Apps",
    "spoken": "Apps",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_AC_DESKTOP_SHOW_ALL_APPLICATIONS"
    ],
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 711,
    "cap": "Keyboard Input Assist Previous",
    "name": "Keyboard Input Assist Previous",
    "spoken": "Keyboard Input Assist Previous",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_KBIA_PREV",
      "C_KEYBOARD_INPUT_ASSIST_PREVIOUS"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 712,
    "cap": "Keyboard Input Assist Next",
    "name": "Keyboard Input Assist Next",
    "spoken": "Keyboard Input Assist Next",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_KBIA_NEXT",
      "C_KEYBOARD_INPUT_ASSIST_NEXT"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 713,
    "cap": "Keyboard Input Assist Previous Group",
    "name": "Keyboard Input Assist Previous Group",
    "spoken": "Keyboard Input Assist Previous Group",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_KBIA_PREV_GRP",
      "C_KEYBOARD_INPUT_ASSIST_PREVIOUS_GROUP"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 714,
    "cap": "Keyboard Input Assist Next Group",
    "name": "Keyboard Input Assist Next Group",
    "spoken": "Keyboard Input Assist Next Group",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_KBIA_NEXT_GRP",
      "C_KEYBOARD_INPUT_ASSIST_NEXT_GROUP"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 715,
    "cap": "Keyboard Input Assist Accept",
    "name": "Keyboard Input Assist Accept",
    "spoken": "Keyboard Input Assist Accept",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_KBIA_ACCEPT",
      "C_KEYBOARD_INPUT_ASSIST_ACCEPT"
    ],
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 716,
    "cap": "Keyboard Input Assist Cancel",
    "name": "Keyboard Input Assist Cancel",
    "spoken": "Keyboard Input Assist Cancel",
    "shifted": null,
    "shiftedSpoken": null,
    "shiftedZmk": null,
    "zmk": [
      "C_KBIA_CANCEL",
      "C_KEYBOARD_INPUT_ASSIST_CANCEL"
    ],
    "group": "other"
  }
];

export const domCodes: DomCode[] = [
  {
    "code": "KeyA",
    "page": "keyboard",
    "id": 4
  },
  {
    "code": "KeyB",
    "page": "keyboard",
    "id": 5
  },
  {
    "code": "KeyC",
    "page": "keyboard",
    "id": 6
  },
  {
    "code": "KeyD",
    "page": "keyboard",
    "id": 7
  },
  {
    "code": "KeyE",
    "page": "keyboard",
    "id": 8
  },
  {
    "code": "KeyF",
    "page": "keyboard",
    "id": 9
  },
  {
    "code": "KeyG",
    "page": "keyboard",
    "id": 10
  },
  {
    "code": "KeyH",
    "page": "keyboard",
    "id": 11
  },
  {
    "code": "KeyI",
    "page": "keyboard",
    "id": 12
  },
  {
    "code": "KeyJ",
    "page": "keyboard",
    "id": 13
  },
  {
    "code": "KeyK",
    "page": "keyboard",
    "id": 14
  },
  {
    "code": "KeyL",
    "page": "keyboard",
    "id": 15
  },
  {
    "code": "KeyM",
    "page": "keyboard",
    "id": 16
  },
  {
    "code": "KeyN",
    "page": "keyboard",
    "id": 17
  },
  {
    "code": "KeyO",
    "page": "keyboard",
    "id": 18
  },
  {
    "code": "KeyP",
    "page": "keyboard",
    "id": 19
  },
  {
    "code": "KeyQ",
    "page": "keyboard",
    "id": 20
  },
  {
    "code": "KeyR",
    "page": "keyboard",
    "id": 21
  },
  {
    "code": "KeyS",
    "page": "keyboard",
    "id": 22
  },
  {
    "code": "KeyT",
    "page": "keyboard",
    "id": 23
  },
  {
    "code": "KeyU",
    "page": "keyboard",
    "id": 24
  },
  {
    "code": "KeyV",
    "page": "keyboard",
    "id": 25
  },
  {
    "code": "KeyW",
    "page": "keyboard",
    "id": 26
  },
  {
    "code": "KeyX",
    "page": "keyboard",
    "id": 27
  },
  {
    "code": "KeyY",
    "page": "keyboard",
    "id": 28
  },
  {
    "code": "KeyZ",
    "page": "keyboard",
    "id": 29
  },
  {
    "code": "Digit1",
    "page": "keyboard",
    "id": 30
  },
  {
    "code": "Digit2",
    "page": "keyboard",
    "id": 31
  },
  {
    "code": "Digit3",
    "page": "keyboard",
    "id": 32
  },
  {
    "code": "Digit4",
    "page": "keyboard",
    "id": 33
  },
  {
    "code": "Digit5",
    "page": "keyboard",
    "id": 34
  },
  {
    "code": "Digit6",
    "page": "keyboard",
    "id": 35
  },
  {
    "code": "Digit7",
    "page": "keyboard",
    "id": 36
  },
  {
    "code": "Digit8",
    "page": "keyboard",
    "id": 37
  },
  {
    "code": "Digit9",
    "page": "keyboard",
    "id": 38
  },
  {
    "code": "Digit0",
    "page": "keyboard",
    "id": 39
  },
  {
    "code": "Enter",
    "page": "keyboard",
    "id": 40
  },
  {
    "code": "Escape",
    "page": "keyboard",
    "id": 41
  },
  {
    "code": "Backspace",
    "page": "keyboard",
    "id": 42
  },
  {
    "code": "Tab",
    "page": "keyboard",
    "id": 43
  },
  {
    "code": "Space",
    "page": "keyboard",
    "id": 44
  },
  {
    "code": "Minus",
    "page": "keyboard",
    "id": 45
  },
  {
    "code": "Equal",
    "page": "keyboard",
    "id": 46
  },
  {
    "code": "BracketLeft",
    "page": "keyboard",
    "id": 47
  },
  {
    "code": "BracketRight",
    "page": "keyboard",
    "id": 48
  },
  {
    "code": "Backslash",
    "page": "keyboard",
    "id": 49
  },
  {
    "code": "Semicolon",
    "page": "keyboard",
    "id": 51
  },
  {
    "code": "Quote",
    "page": "keyboard",
    "id": 52
  },
  {
    "code": "Backquote",
    "page": "keyboard",
    "id": 53
  },
  {
    "code": "Comma",
    "page": "keyboard",
    "id": 54
  },
  {
    "code": "Period",
    "page": "keyboard",
    "id": 55
  },
  {
    "code": "Slash",
    "page": "keyboard",
    "id": 56
  },
  {
    "code": "CapsLock",
    "page": "keyboard",
    "id": 57
  },
  {
    "code": "F1",
    "page": "keyboard",
    "id": 58
  },
  {
    "code": "F2",
    "page": "keyboard",
    "id": 59
  },
  {
    "code": "F3",
    "page": "keyboard",
    "id": 60
  },
  {
    "code": "F4",
    "page": "keyboard",
    "id": 61
  },
  {
    "code": "F5",
    "page": "keyboard",
    "id": 62
  },
  {
    "code": "F6",
    "page": "keyboard",
    "id": 63
  },
  {
    "code": "F7",
    "page": "keyboard",
    "id": 64
  },
  {
    "code": "F8",
    "page": "keyboard",
    "id": 65
  },
  {
    "code": "F9",
    "page": "keyboard",
    "id": 66
  },
  {
    "code": "F10",
    "page": "keyboard",
    "id": 67
  },
  {
    "code": "F11",
    "page": "keyboard",
    "id": 68
  },
  {
    "code": "F12",
    "page": "keyboard",
    "id": 69
  },
  {
    "code": "PrintScreen",
    "page": "keyboard",
    "id": 70
  },
  {
    "code": "ScrollLock",
    "page": "keyboard",
    "id": 71
  },
  {
    "code": "Pause",
    "page": "keyboard",
    "id": 72
  },
  {
    "code": "Insert",
    "page": "keyboard",
    "id": 73
  },
  {
    "code": "Home",
    "page": "keyboard",
    "id": 74
  },
  {
    "code": "PageUp",
    "page": "keyboard",
    "id": 75
  },
  {
    "code": "Delete",
    "page": "keyboard",
    "id": 76
  },
  {
    "code": "End",
    "page": "keyboard",
    "id": 77
  },
  {
    "code": "PageDown",
    "page": "keyboard",
    "id": 78
  },
  {
    "code": "ArrowRight",
    "page": "keyboard",
    "id": 79
  },
  {
    "code": "ArrowLeft",
    "page": "keyboard",
    "id": 80
  },
  {
    "code": "ArrowDown",
    "page": "keyboard",
    "id": 81
  },
  {
    "code": "ArrowUp",
    "page": "keyboard",
    "id": 82
  },
  {
    "code": "NumLock",
    "page": "keyboard",
    "id": 83
  },
  {
    "code": "NumpadDivide",
    "page": "keyboard",
    "id": 84
  },
  {
    "code": "NumpadMultiply",
    "page": "keyboard",
    "id": 85
  },
  {
    "code": "NumpadSubtract",
    "page": "keyboard",
    "id": 86
  },
  {
    "code": "NumpadAdd",
    "page": "keyboard",
    "id": 87
  },
  {
    "code": "NumpadEnter",
    "page": "keyboard",
    "id": 88
  },
  {
    "code": "Numpad1",
    "page": "keyboard",
    "id": 89
  },
  {
    "code": "Numpad2",
    "page": "keyboard",
    "id": 90
  },
  {
    "code": "Numpad3",
    "page": "keyboard",
    "id": 91
  },
  {
    "code": "Numpad4",
    "page": "keyboard",
    "id": 92
  },
  {
    "code": "Numpad5",
    "page": "keyboard",
    "id": 93
  },
  {
    "code": "Numpad6",
    "page": "keyboard",
    "id": 94
  },
  {
    "code": "Numpad7",
    "page": "keyboard",
    "id": 95
  },
  {
    "code": "Numpad8",
    "page": "keyboard",
    "id": 96
  },
  {
    "code": "Numpad9",
    "page": "keyboard",
    "id": 97
  },
  {
    "code": "Numpad0",
    "page": "keyboard",
    "id": 98
  },
  {
    "code": "NumpadDecimal",
    "page": "keyboard",
    "id": 99
  },
  {
    "code": "IntlBackslash",
    "page": "keyboard",
    "id": 100
  },
  {
    "code": "ContextMenu",
    "page": "keyboard",
    "id": 101
  },
  {
    "code": "Power",
    "page": "keyboard",
    "id": 102
  },
  {
    "code": "NumpadEqual",
    "page": "keyboard",
    "id": 103
  },
  {
    "code": "F13",
    "page": "keyboard",
    "id": 104
  },
  {
    "code": "F14",
    "page": "keyboard",
    "id": 105
  },
  {
    "code": "F15",
    "page": "keyboard",
    "id": 106
  },
  {
    "code": "F16",
    "page": "keyboard",
    "id": 107
  },
  {
    "code": "F17",
    "page": "keyboard",
    "id": 108
  },
  {
    "code": "F18",
    "page": "keyboard",
    "id": 109
  },
  {
    "code": "F19",
    "page": "keyboard",
    "id": 110
  },
  {
    "code": "F20",
    "page": "keyboard",
    "id": 111
  },
  {
    "code": "F21",
    "page": "keyboard",
    "id": 112
  },
  {
    "code": "F22",
    "page": "keyboard",
    "id": 113
  },
  {
    "code": "F23",
    "page": "keyboard",
    "id": 114
  },
  {
    "code": "F24",
    "page": "keyboard",
    "id": 115
  },
  {
    "code": "Open",
    "page": "keyboard",
    "id": 116
  },
  {
    "code": "Help",
    "page": "keyboard",
    "id": 117
  },
  {
    "code": "Select",
    "page": "keyboard",
    "id": 119
  },
  {
    "code": "Again",
    "page": "keyboard",
    "id": 121
  },
  {
    "code": "Undo",
    "page": "keyboard",
    "id": 122
  },
  {
    "code": "Cut",
    "page": "keyboard",
    "id": 123
  },
  {
    "code": "Copy",
    "page": "keyboard",
    "id": 124
  },
  {
    "code": "Paste",
    "page": "keyboard",
    "id": 125
  },
  {
    "code": "Find",
    "page": "keyboard",
    "id": 126
  },
  {
    "code": "AudioVolumeMute",
    "page": "keyboard",
    "id": 127
  },
  {
    "code": "AudioVolumeUp",
    "page": "keyboard",
    "id": 128
  },
  {
    "code": "AudioVolumeDown",
    "page": "keyboard",
    "id": 129
  },
  {
    "code": "NumpadComma",
    "page": "keyboard",
    "id": 133
  },
  {
    "code": "IntlRo",
    "page": "keyboard",
    "id": 135
  },
  {
    "code": "KanaMode",
    "page": "keyboard",
    "id": 136
  },
  {
    "code": "IntlYen",
    "page": "keyboard",
    "id": 137
  },
  {
    "code": "Convert",
    "page": "keyboard",
    "id": 138
  },
  {
    "code": "NonConvert",
    "page": "keyboard",
    "id": 139
  },
  {
    "code": "Lang1",
    "page": "keyboard",
    "id": 144
  },
  {
    "code": "Lang2",
    "page": "keyboard",
    "id": 145
  },
  {
    "code": "Lang3",
    "page": "keyboard",
    "id": 146
  },
  {
    "code": "Lang4",
    "page": "keyboard",
    "id": 147
  },
  {
    "code": "Lang5",
    "page": "keyboard",
    "id": 148
  },
  {
    "code": "Abort",
    "page": "keyboard",
    "id": 155
  },
  {
    "code": "Props",
    "page": "keyboard",
    "id": 163
  },
  {
    "code": "NumpadParenLeft",
    "page": "keyboard",
    "id": 182
  },
  {
    "code": "NumpadParenRight",
    "page": "keyboard",
    "id": 183
  },
  {
    "code": "NumpadClear",
    "page": "keyboard",
    "id": 216
  },
  {
    "code": "ControlLeft",
    "page": "keyboard",
    "id": 224
  },
  {
    "code": "ShiftLeft",
    "page": "keyboard",
    "id": 225
  },
  {
    "code": "AltLeft",
    "page": "keyboard",
    "id": 226
  },
  {
    "code": "MetaLeft",
    "page": "keyboard",
    "id": 227
  },
  {
    "code": "ControlRight",
    "page": "keyboard",
    "id": 228
  },
  {
    "code": "ShiftRight",
    "page": "keyboard",
    "id": 229
  },
  {
    "code": "AltRight",
    "page": "keyboard",
    "id": 230
  },
  {
    "code": "MetaRight",
    "page": "keyboard",
    "id": 231
  },
  {
    "code": "BrightnessUp",
    "page": "consumer",
    "id": 111
  },
  {
    "code": "BrightnessDown",
    "page": "consumer",
    "id": 112
  },
  {
    "code": "MediaPlay",
    "page": "consumer",
    "id": 176
  },
  {
    "code": "MediaPause",
    "page": "consumer",
    "id": 177
  },
  {
    "code": "MediaRecord",
    "page": "consumer",
    "id": 178
  },
  {
    "code": "MediaFastForward",
    "page": "consumer",
    "id": 179
  },
  {
    "code": "MediaRewind",
    "page": "consumer",
    "id": 180
  },
  {
    "code": "MediaTrackNext",
    "page": "consumer",
    "id": 181
  },
  {
    "code": "MediaTrackPrevious",
    "page": "consumer",
    "id": 182
  },
  {
    "code": "MediaStop",
    "page": "consumer",
    "id": 183
  },
  {
    "code": "Eject",
    "page": "consumer",
    "id": 184
  },
  {
    "code": "MediaPlayPause",
    "page": "consumer",
    "id": 205
  },
  {
    "code": "MediaSelect",
    "page": "consumer",
    "id": 387
  },
  {
    "code": "LaunchMail",
    "page": "consumer",
    "id": 394
  },
  {
    "code": "LaunchApp2",
    "page": "consumer",
    "id": 402
  },
  {
    "code": "LaunchApp1",
    "page": "consumer",
    "id": 404
  },
  {
    "code": "LaunchControlPanel",
    "page": "consumer",
    "id": 415
  },
  {
    "code": "SelectTask",
    "page": "consumer",
    "id": 418
  },
  {
    "code": "LaunchScreenSaver",
    "page": "consumer",
    "id": 433
  },
  {
    "code": "BrowserSearch",
    "page": "consumer",
    "id": 545
  },
  {
    "code": "BrowserHome",
    "page": "consumer",
    "id": 547
  },
  {
    "code": "BrowserBack",
    "page": "consumer",
    "id": 548
  },
  {
    "code": "BrowserForward",
    "page": "consumer",
    "id": 549
  },
  {
    "code": "BrowserStop",
    "page": "consumer",
    "id": 550
  },
  {
    "code": "BrowserRefresh",
    "page": "consumer",
    "id": 551
  },
  {
    "code": "BrowserFavorites",
    "page": "consumer",
    "id": 554
  },
  {
    "code": "ZoomToggle",
    "page": "consumer",
    "id": 562
  },
  {
    "code": "MailReply",
    "page": "consumer",
    "id": 649
  },
  {
    "code": "MailForward",
    "page": "consumer",
    "id": 651
  },
  {
    "code": "MailSend",
    "page": "consumer",
    "id": 652
  },
  {
    "code": "KeyboardLayoutSelect",
    "page": "consumer",
    "id": 669
  },
  {
    "code": "ShowAllWindows",
    "page": "consumer",
    "id": 671
  },
  {
    "code": "ControlRight",
    "page": "keyboard",
    "id": 228
  },
  {
    "code": "ShiftRight",
    "page": "keyboard",
    "id": 229
  },
  {
    "code": "AltRight",
    "page": "keyboard",
    "id": 230
  },
  {
    "code": "MetaRight",
    "page": "keyboard",
    "id": 231
  }
];

export const macActions: Choice[] = [
  {
    "category": "mac",
    "label": "Mission Control",
    "sentence": "Shows all your open windows.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 0,
        "page": "consumer",
        "id": 671
      }
    },
    "icon": "layout-grid",
    "defaultShortcut": null
  },
  {
    "category": "mac",
    "label": "Apps",
    "sentence": "Shows all your apps, or Launchpad before macOS 26.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 0,
        "page": "consumer",
        "id": 674
      }
    },
    "icon": "grid-3x3",
    "defaultShortcut": null
  },
  {
    "category": "mac",
    "label": "Spotlight",
    "sentence": "Opens Spotlight, to search your Mac.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 0,
        "page": "consumer",
        "id": 545
      }
    },
    "icon": "search",
    "defaultShortcut": null
  },
  {
    "category": "mac",
    "label": "Show Desktop",
    "sentence": "Moves the windows aside to show the desktop.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 0,
        "page": "keyboard",
        "id": 68
      }
    },
    "icon": "monitor",
    "defaultShortcut": "F11"
  },
  {
    "category": "mac",
    "label": "Screenshot",
    "sentence": "Saves a picture of the whole screen.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 10,
        "page": "keyboard",
        "id": 32
      }
    },
    "icon": "camera",
    "defaultShortcut": "⇧⌘3"
  },
  {
    "category": "mac",
    "label": "Screenshot of an area",
    "sentence": "Saves a picture of the part of the screen you choose.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 10,
        "page": "keyboard",
        "id": 33
      }
    },
    "icon": "crop",
    "defaultShortcut": "⇧⌘4"
  },
  {
    "category": "mac",
    "label": "Screenshot options",
    "sentence": "Shows the toolbar for screenshots and screen recordings.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 10,
        "page": "keyboard",
        "id": 34
      }
    },
    "icon": "aperture",
    "defaultShortcut": "⇧⌘5"
  },
  {
    "category": "mac",
    "label": "Lock Screen",
    "sentence": "Locks the Mac.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 9,
        "page": "keyboard",
        "id": 20
      }
    },
    "icon": "lock",
    "defaultShortcut": "⌃⌘Q"
  },
  {
    "category": "mac",
    "label": "Emoji & Symbols",
    "sentence": "Opens the emoji and symbols picker.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 9,
        "page": "keyboard",
        "id": 44
      }
    },
    "icon": "smile",
    "defaultShortcut": "⌃⌘␣"
  },
  {
    "category": "mac",
    "label": "Dictation",
    "sentence": "Starts dictation, or offers to turn it on.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 0,
        "page": "consumer",
        "id": 207
      }
    },
    "icon": "mic",
    "defaultShortcut": null
  },
  {
    "category": "mac",
    "label": "Switch App",
    "sentence": "Switches to the app you used last.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 8,
        "page": "keyboard",
        "id": 43
      }
    },
    "icon": "app-window",
    "defaultShortcut": null
  },
  {
    "category": "mac",
    "label": "Copy",
    "sentence": "Copies the selection.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 8,
        "page": "keyboard",
        "id": 6
      }
    },
    "icon": "copy",
    "defaultShortcut": null
  },
  {
    "category": "mac",
    "label": "Cut",
    "sentence": "Cuts the selection.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 8,
        "page": "keyboard",
        "id": 27
      }
    },
    "icon": "scissors",
    "defaultShortcut": null
  },
  {
    "category": "mac",
    "label": "Paste",
    "sentence": "Pastes what you copied.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 8,
        "page": "keyboard",
        "id": 25
      }
    },
    "icon": "clipboard-paste",
    "defaultShortcut": null
  },
  {
    "category": "mac",
    "label": "Undo",
    "sentence": "Undoes the last change.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 8,
        "page": "keyboard",
        "id": 29
      }
    },
    "icon": "undo-2",
    "defaultShortcut": null
  },
  {
    "category": "mac",
    "label": "Redo",
    "sentence": "Redoes what you undid.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 10,
        "page": "keyboard",
        "id": 29
      }
    },
    "icon": "redo-2",
    "defaultShortcut": null
  },
  {
    "category": "mac",
    "label": "Brighter",
    "sentence": "Makes the screen brighter.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 0,
        "page": "consumer",
        "id": 111
      }
    },
    "icon": "sun",
    "defaultShortcut": null
  },
  {
    "category": "mac",
    "label": "Dimmer",
    "sentence": "Makes the screen dimmer.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 0,
        "page": "consumer",
        "id": 112
      }
    },
    "icon": "sun-dim",
    "defaultShortcut": null
  },
  {
    "category": "mac",
    "label": "Volume up",
    "sentence": "Turns the sound up.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 0,
        "page": "consumer",
        "id": 233
      }
    },
    "icon": "volume-2",
    "defaultShortcut": null
  },
  {
    "category": "mac",
    "label": "Volume down",
    "sentence": "Turns the sound down.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 0,
        "page": "consumer",
        "id": 234
      }
    },
    "icon": "volume-1",
    "defaultShortcut": null
  },
  {
    "category": "mac",
    "label": "Mute",
    "sentence": "Mutes or unmutes the sound.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 0,
        "page": "consumer",
        "id": 226
      }
    },
    "icon": "volume-x",
    "defaultShortcut": null
  },
  {
    "category": "mac",
    "label": "Play/Pause",
    "sentence": "Plays or pauses music or video.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 0,
        "page": "consumer",
        "id": 205
      }
    },
    "icon": "play",
    "defaultShortcut": null
  },
  {
    "category": "mac",
    "label": "Next track",
    "sentence": "Skips to the next track.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 0,
        "page": "consumer",
        "id": 181
      }
    },
    "icon": "skip-forward",
    "defaultShortcut": null
  },
  {
    "category": "mac",
    "label": "Previous track",
    "sentence": "Goes back to the previous track.",
    "action": {
      "kind": "key-press",
      "key": {
        "mods": 0,
        "page": "consumer",
        "id": 182
      }
    },
    "icon": "skip-back",
    "defaultShortcut": null
  }
];

export const sofleGuide: KeyboardGuide = {
  "name": "Keebart Sofle Choc Pro BT",
  "keys": [
    {
      "x": 0,
      "y": 37,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 1, key 1"
    },
    {
      "x": 100,
      "y": 37,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 1, key 2"
    },
    {
      "x": 200,
      "y": 12,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 1, key 3"
    },
    {
      "x": 300,
      "y": 0,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 1, key 4"
    },
    {
      "x": 400,
      "y": 12,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 1, key 5"
    },
    {
      "x": 500,
      "y": 24,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 1, key 6"
    },
    {
      "x": 900,
      "y": 24,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 1, key 1"
    },
    {
      "x": 1000,
      "y": 12,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 1, key 2"
    },
    {
      "x": 1100,
      "y": 0,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 1, key 3"
    },
    {
      "x": 1200,
      "y": 12,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 1, key 4"
    },
    {
      "x": 1300,
      "y": 37,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 1, key 5"
    },
    {
      "x": 1400,
      "y": 37,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 1, key 6"
    },
    {
      "x": 0,
      "y": 137,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 2, key 1"
    },
    {
      "x": 100,
      "y": 137,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 2, key 2"
    },
    {
      "x": 200,
      "y": 112,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 2, key 3"
    },
    {
      "x": 300,
      "y": 100,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 2, key 4"
    },
    {
      "x": 400,
      "y": 112,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 2, key 5"
    },
    {
      "x": 500,
      "y": 124,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 2, key 6"
    },
    {
      "x": 900,
      "y": 124,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 2, key 1"
    },
    {
      "x": 1000,
      "y": 112,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 2, key 2"
    },
    {
      "x": 1100,
      "y": 100,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 2, key 3"
    },
    {
      "x": 1200,
      "y": 112,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 2, key 4"
    },
    {
      "x": 1300,
      "y": 137,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 2, key 5"
    },
    {
      "x": 1400,
      "y": 137,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 2, key 6"
    },
    {
      "x": 0,
      "y": 237,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 3, key 1"
    },
    {
      "x": 100,
      "y": 237,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 3, key 2"
    },
    {
      "x": 200,
      "y": 212,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 3, key 3"
    },
    {
      "x": 300,
      "y": 200,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 3, key 4"
    },
    {
      "x": 400,
      "y": 212,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 3, key 5"
    },
    {
      "x": 500,
      "y": 224,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 3, key 6"
    },
    {
      "x": 900,
      "y": 224,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 3, key 1"
    },
    {
      "x": 1000,
      "y": 212,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 3, key 2"
    },
    {
      "x": 1100,
      "y": 200,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 3, key 3"
    },
    {
      "x": 1200,
      "y": 212,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 3, key 4"
    },
    {
      "x": 1300,
      "y": 237,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 3, key 5"
    },
    {
      "x": 1400,
      "y": 237,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 3, key 6"
    },
    {
      "x": 0,
      "y": 337,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 4, key 1"
    },
    {
      "x": 100,
      "y": 337,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 4, key 2"
    },
    {
      "x": 200,
      "y": 312,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 4, key 3"
    },
    {
      "x": 300,
      "y": 300,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 4, key 4"
    },
    {
      "x": 400,
      "y": 312,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 4, key 5"
    },
    {
      "x": 500,
      "y": 324,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 4, key 6"
    },
    {
      "x": 600,
      "y": 274,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left knob"
    },
    {
      "x": 800,
      "y": 274,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right knob"
    },
    {
      "x": 900,
      "y": 324,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 4, key 1"
    },
    {
      "x": 1000,
      "y": 312,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 4, key 2"
    },
    {
      "x": 1100,
      "y": 300,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 4, key 3"
    },
    {
      "x": 1200,
      "y": 312,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 4, key 4"
    },
    {
      "x": 1300,
      "y": 337,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 4, key 5"
    },
    {
      "x": 1400,
      "y": 337,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 4, key 6"
    },
    {
      "x": 175,
      "y": 437,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 5, key 1"
    },
    {
      "x": 275,
      "y": 412,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 5, key 2"
    },
    {
      "x": 375,
      "y": 412,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Left half, row 5, key 3"
    },
    {
      "x": 490,
      "y": 412,
      "width": 100,
      "height": 100,
      "rotation": 1200,
      "rotationX": 490,
      "rotationY": 412,
      "place": "Left half, row 5, key 4"
    },
    {
      "x": 600,
      "y": 383,
      "width": 100,
      "height": 150,
      "rotation": 2400,
      "rotationX": 600,
      "rotationY": 433,
      "place": "Left half, row 5, key 5"
    },
    {
      "x": 800,
      "y": 383,
      "width": 100,
      "height": 150,
      "rotation": -2400,
      "rotationX": 900,
      "rotationY": 433,
      "place": "Right half, row 5, key 1"
    },
    {
      "x": 910,
      "y": 412,
      "width": 100,
      "height": 100,
      "rotation": -1200,
      "rotationX": 1010,
      "rotationY": 412,
      "place": "Right half, row 5, key 2"
    },
    {
      "x": 1025,
      "y": 412,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 5, key 3"
    },
    {
      "x": 1125,
      "y": 412,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 5, key 4"
    },
    {
      "x": 1225,
      "y": 437,
      "width": 100,
      "height": 100,
      "rotation": 0,
      "rotationX": 0,
      "rotationY": 0,
      "place": "Right half, row 5, key 5"
    }
  ],
  "usbHalf": "left",
  "unlock": [
    {
      "position": 56,
      "hold": true,
      "name": "raise",
      "place": "Right half, row 5, key 2"
    },
    {
      "position": 41,
      "hold": false,
      "name": "B",
      "place": "Left half, row 4, key 6"
    }
  ],
  "bluetoothHold": "raise"
};
