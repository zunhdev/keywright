// The simulated Sofle's stock keymap, as `session_keymap` gives it, the key
// tables, as `key_catalog` and `dom_codes` do, and the Mac actions on macOS 27,
// as `mac_actions` does there, generated from keywright-app (src/ipc.rs) for the
// UI's tests and gallery. Don't edit it: run `mise run generate:bindings`.
import type { Choice, DomCode, KeyEntry, Keymap } from "../ipc/bindings.ts";

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
          "spoken": "Types `.",
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "spoken": "Types `.",
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "spoken": "Types ;.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types '.",
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "spoken": "Types ,.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types ..",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types /.",
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
        }
      ],
      "reach": [
        {
          "text": "The base layer, always on",
          "position": null,
          "place": null
        }
      ],
      "changed": false
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
          "spoken": "Same as the layer below: types `.",
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "spoken": "Types `.",
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "spoken": "Types !.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types @.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types #.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types $.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types %.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types ^.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types &.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types *.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types (.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types ).",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types |.",
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "spoken": "Types =.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types -.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types +.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types {.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types }.",
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "spoken": "Types [.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types ].",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types ;.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types :.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Types \\.",
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
        }
      ],
      "reach": [
        {
          "text": "Hold lower",
          "position": 53,
          "place": "Left half, row 5, key 4"
        }
      ],
      "changed": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "spoken": "Same as the layer below: types `.",
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": true
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
          "changed": false,
          "macWarning": true
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
          "changed": false,
          "macWarning": true
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": true
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
          "changed": false,
          "macWarning": true
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
          "changed": false,
          "macWarning": true
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
          "changed": false,
          "macWarning": true
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "spoken": "Same as the layer below: types ,.",
          "changed": false,
          "macWarning": false
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
          "spoken": "Same as the layer below: types ..",
          "changed": false,
          "macWarning": false
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
          "spoken": "Same as the layer below: types /.",
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
        }
      ],
      "reach": [
        {
          "text": "Hold raise",
          "position": 56,
          "place": "Right half, row 5, key 2"
        }
      ],
      "changed": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
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
          "changed": false,
          "macWarning": false
        }
      ],
      "reach": [
        {
          "text": "lower + raise, built into the firmware",
          "position": null,
          "place": null
        }
      ],
      "changed": false
    }
  ],
  "freeLayerSlots": 0,
  "layerNameMax": 18,
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
  ]
};

export const keyCatalog: KeyEntry[] = [
  {
    "page": "keyboard",
    "id": 4,
    "cap": "A",
    "name": "A",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 5,
    "cap": "B",
    "name": "B",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 6,
    "cap": "C",
    "name": "C",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 7,
    "cap": "D",
    "name": "D",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 8,
    "cap": "E",
    "name": "E",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 9,
    "cap": "F",
    "name": "F",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 10,
    "cap": "G",
    "name": "G",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 11,
    "cap": "H",
    "name": "H",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 12,
    "cap": "I",
    "name": "I",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 13,
    "cap": "J",
    "name": "J",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 14,
    "cap": "K",
    "name": "K",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 15,
    "cap": "L",
    "name": "L",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 16,
    "cap": "M",
    "name": "M",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 17,
    "cap": "N",
    "name": "N",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 18,
    "cap": "O",
    "name": "O",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 19,
    "cap": "P",
    "name": "P",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 20,
    "cap": "Q",
    "name": "Q",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 21,
    "cap": "R",
    "name": "R",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 22,
    "cap": "S",
    "name": "S",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 23,
    "cap": "T",
    "name": "T",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 24,
    "cap": "U",
    "name": "U",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 25,
    "cap": "V",
    "name": "V",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 26,
    "cap": "W",
    "name": "W",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 27,
    "cap": "X",
    "name": "X",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 28,
    "cap": "Y",
    "name": "Y",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 29,
    "cap": "Z",
    "name": "Z",
    "group": "letter"
  },
  {
    "page": "keyboard",
    "id": 30,
    "cap": "1",
    "name": "1",
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 31,
    "cap": "2",
    "name": "2",
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 32,
    "cap": "3",
    "name": "3",
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 33,
    "cap": "4",
    "name": "4",
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 34,
    "cap": "5",
    "name": "5",
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 35,
    "cap": "6",
    "name": "6",
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 36,
    "cap": "7",
    "name": "7",
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 37,
    "cap": "8",
    "name": "8",
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 38,
    "cap": "9",
    "name": "9",
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 39,
    "cap": "0",
    "name": "0",
    "group": "number"
  },
  {
    "page": "keyboard",
    "id": 40,
    "cap": "↩",
    "name": "Return",
    "group": "editing"
  },
  {
    "page": "keyboard",
    "id": 41,
    "cap": "⎋ Esc",
    "name": "Escape",
    "group": "editing"
  },
  {
    "page": "keyboard",
    "id": 42,
    "cap": "⌫",
    "name": "Delete",
    "group": "editing"
  },
  {
    "page": "keyboard",
    "id": 43,
    "cap": "⇥",
    "name": "Tab",
    "group": "editing"
  },
  {
    "page": "keyboard",
    "id": 44,
    "cap": "␣",
    "name": "Space",
    "group": "editing"
  },
  {
    "page": "keyboard",
    "id": 45,
    "cap": "-",
    "name": "-",
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 46,
    "cap": "=",
    "name": "=",
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 47,
    "cap": "[",
    "name": "[",
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 48,
    "cap": "]",
    "name": "]",
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 49,
    "cap": "\\",
    "name": "\\",
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 50,
    "cap": "#",
    "name": "ISO #",
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 51,
    "cap": ";",
    "name": ";",
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 52,
    "cap": "'",
    "name": "'",
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 53,
    "cap": "`",
    "name": "`",
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 54,
    "cap": ",",
    "name": ",",
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 55,
    "cap": ".",
    "name": ".",
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 56,
    "cap": "/",
    "name": "/",
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 57,
    "cap": "⇪",
    "name": "Caps Lock",
    "group": "modifier"
  },
  {
    "page": "keyboard",
    "id": 58,
    "cap": "F1",
    "name": "F1",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 59,
    "cap": "F2",
    "name": "F2",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 60,
    "cap": "F3",
    "name": "F3",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 61,
    "cap": "F4",
    "name": "F4",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 62,
    "cap": "F5",
    "name": "F5",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 63,
    "cap": "F6",
    "name": "F6",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 64,
    "cap": "F7",
    "name": "F7",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 65,
    "cap": "F8",
    "name": "F8",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 66,
    "cap": "F9",
    "name": "F9",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 67,
    "cap": "F10",
    "name": "F10",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 68,
    "cap": "F11",
    "name": "F11",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 69,
    "cap": "F12",
    "name": "F12",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 70,
    "cap": "Print Screen",
    "name": "Print Screen",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 71,
    "cap": "Scroll Lock",
    "name": "Scroll Lock",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 72,
    "cap": "Pause",
    "name": "Pause",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 73,
    "cap": "Insert",
    "name": "Insert",
    "group": "editing"
  },
  {
    "page": "keyboard",
    "id": 74,
    "cap": "↖",
    "name": "Home",
    "group": "navigation"
  },
  {
    "page": "keyboard",
    "id": 75,
    "cap": "⇞",
    "name": "Page Up",
    "group": "navigation"
  },
  {
    "page": "keyboard",
    "id": 76,
    "cap": "⌦",
    "name": "Forward Delete",
    "group": "editing"
  },
  {
    "page": "keyboard",
    "id": 77,
    "cap": "↘",
    "name": "End",
    "group": "navigation"
  },
  {
    "page": "keyboard",
    "id": 78,
    "cap": "⇟",
    "name": "Page Down",
    "group": "navigation"
  },
  {
    "page": "keyboard",
    "id": 79,
    "cap": "→",
    "name": "Right Arrow",
    "group": "navigation"
  },
  {
    "page": "keyboard",
    "id": 80,
    "cap": "←",
    "name": "Left Arrow",
    "group": "navigation"
  },
  {
    "page": "keyboard",
    "id": 81,
    "cap": "↓",
    "name": "Down Arrow",
    "group": "navigation"
  },
  {
    "page": "keyboard",
    "id": 82,
    "cap": "↑",
    "name": "Up Arrow",
    "group": "navigation"
  },
  {
    "page": "keyboard",
    "id": 83,
    "cap": "⌧",
    "name": "Clear",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 84,
    "cap": "/",
    "name": "Keypad /",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 85,
    "cap": "*",
    "name": "Keypad *",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 86,
    "cap": "-",
    "name": "Keypad -",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 87,
    "cap": "+",
    "name": "Keypad +",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 88,
    "cap": "⌤",
    "name": "Keypad Enter",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 89,
    "cap": "1",
    "name": "Keypad 1",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 90,
    "cap": "2",
    "name": "Keypad 2",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 91,
    "cap": "3",
    "name": "Keypad 3",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 92,
    "cap": "4",
    "name": "Keypad 4",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 93,
    "cap": "5",
    "name": "Keypad 5",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 94,
    "cap": "6",
    "name": "Keypad 6",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 95,
    "cap": "7",
    "name": "Keypad 7",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 96,
    "cap": "8",
    "name": "Keypad 8",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 97,
    "cap": "9",
    "name": "Keypad 9",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 98,
    "cap": "0",
    "name": "Keypad 0",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 99,
    "cap": ".",
    "name": "Keypad .",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 100,
    "cap": "\\",
    "name": "ISO \\",
    "group": "symbol"
  },
  {
    "page": "keyboard",
    "id": 101,
    "cap": "Menu",
    "name": "Menu",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 102,
    "cap": "Power",
    "name": "Power",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 103,
    "cap": "=",
    "name": "Keypad =",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 104,
    "cap": "F13",
    "name": "F13",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 105,
    "cap": "F14",
    "name": "F14",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 106,
    "cap": "F15",
    "name": "F15",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 107,
    "cap": "F16",
    "name": "F16",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 108,
    "cap": "F17",
    "name": "F17",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 109,
    "cap": "F18",
    "name": "F18",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 110,
    "cap": "F19",
    "name": "F19",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 111,
    "cap": "F20",
    "name": "F20",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 112,
    "cap": "F21",
    "name": "F21",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 113,
    "cap": "F22",
    "name": "F22",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 114,
    "cap": "F23",
    "name": "F23",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 115,
    "cap": "F24",
    "name": "F24",
    "group": "function"
  },
  {
    "page": "keyboard",
    "id": 116,
    "cap": "Execute",
    "name": "Execute",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 117,
    "cap": "Help",
    "name": "Help",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 118,
    "cap": "Menu",
    "name": "Menu",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 119,
    "cap": "Select",
    "name": "Select",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 120,
    "cap": "Stop",
    "name": "Stop",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 121,
    "cap": "Again",
    "name": "Again",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 122,
    "cap": "Undo",
    "name": "Undo",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 123,
    "cap": "Cut",
    "name": "Cut",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 124,
    "cap": "Copy",
    "name": "Copy",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 125,
    "cap": "Paste",
    "name": "Paste",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 126,
    "cap": "Find",
    "name": "Find",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 127,
    "cap": "Mute",
    "name": "Mute",
    "group": "media"
  },
  {
    "page": "keyboard",
    "id": 128,
    "cap": "Volume Up",
    "name": "Volume Up",
    "group": "media"
  },
  {
    "page": "keyboard",
    "id": 129,
    "cap": "Volume Down",
    "name": "Volume Down",
    "group": "media"
  },
  {
    "page": "keyboard",
    "id": 130,
    "cap": "Locking Caps Lock",
    "name": "Locking Caps Lock",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 131,
    "cap": "Locking Num Lock",
    "name": "Locking Num Lock",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 132,
    "cap": "Locking Scroll Lock",
    "name": "Locking Scroll Lock",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 133,
    "cap": "Keypad ,",
    "name": "Keypad ,",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 134,
    "cap": "Keypad = (Equal) AS/400",
    "name": "Keypad = (Equal) AS/400",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 135,
    "cap": "International 1",
    "name": "International 1",
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 136,
    "cap": "International 2",
    "name": "International 2",
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 137,
    "cap": "International 3",
    "name": "International 3",
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 138,
    "cap": "International 4",
    "name": "International 4",
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 139,
    "cap": "International 5",
    "name": "International 5",
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 140,
    "cap": "International 6",
    "name": "International 6",
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 141,
    "cap": "International 7",
    "name": "International 7",
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 142,
    "cap": "International 8",
    "name": "International 8",
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 143,
    "cap": "International 9",
    "name": "International 9",
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 144,
    "cap": "Language 1",
    "name": "Language 1",
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 145,
    "cap": "Language 2",
    "name": "Language 2",
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 146,
    "cap": "Language 3",
    "name": "Language 3",
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 147,
    "cap": "Language 4",
    "name": "Language 4",
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 148,
    "cap": "Language 5",
    "name": "Language 5",
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 149,
    "cap": "Language 6",
    "name": "Language 6",
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 150,
    "cap": "Language 7",
    "name": "Language 7",
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 151,
    "cap": "Language 8",
    "name": "Language 8",
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 152,
    "cap": "Language 9",
    "name": "Language 9",
    "group": "international"
  },
  {
    "page": "keyboard",
    "id": 153,
    "cap": "Alternate Erase",
    "name": "Alternate Erase",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 154,
    "cap": "SysReq/Attention",
    "name": "SysReq/Attention",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 155,
    "cap": "Cancel",
    "name": "Cancel",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 156,
    "cap": "Clear",
    "name": "Clear",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 157,
    "cap": "Prior",
    "name": "Prior",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 158,
    "cap": "Return",
    "name": "Return",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 159,
    "cap": "Separator",
    "name": "Separator",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 160,
    "cap": "Out",
    "name": "Out",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 161,
    "cap": "Oper",
    "name": "Oper",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 162,
    "cap": "Clear/Again",
    "name": "Clear/Again",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 163,
    "cap": "CrSel/Props",
    "name": "CrSel/Props",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 164,
    "cap": "ExSel",
    "name": "ExSel",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 180,
    "cap": "Currency Unit",
    "name": "Currency Unit",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 182,
    "cap": "Keypad (",
    "name": "Keypad (",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 183,
    "cap": "Keypad )",
    "name": "Keypad )",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 205,
    "cap": "Keypad Space",
    "name": "Keypad Space",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 216,
    "cap": "Keypad Clear",
    "name": "Keypad Clear",
    "group": "numpad"
  },
  {
    "page": "keyboard",
    "id": 224,
    "cap": "⌃",
    "name": "Control",
    "group": "modifier"
  },
  {
    "page": "keyboard",
    "id": 225,
    "cap": "⇧",
    "name": "Shift",
    "group": "modifier"
  },
  {
    "page": "keyboard",
    "id": 226,
    "cap": "⌥",
    "name": "Option",
    "group": "modifier"
  },
  {
    "page": "keyboard",
    "id": 227,
    "cap": "⌘",
    "name": "Command",
    "group": "modifier"
  },
  {
    "page": "keyboard",
    "id": 232,
    "cap": "Play/Pause",
    "name": "Play/Pause",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 233,
    "cap": "Stop",
    "name": "Stop",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 234,
    "cap": "Previous",
    "name": "Previous",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 235,
    "cap": "Next",
    "name": "Next",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 236,
    "cap": "Eject",
    "name": "Eject",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 237,
    "cap": "Volume Up",
    "name": "Volume Up",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 238,
    "cap": "Volume Down",
    "name": "Volume Down",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 239,
    "cap": "Mute",
    "name": "Mute",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 240,
    "cap": "WWW",
    "name": "WWW",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 241,
    "cap": "Back",
    "name": "Back",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 242,
    "cap": "Forward",
    "name": "Forward",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 243,
    "cap": "Stop",
    "name": "Stop",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 244,
    "cap": "Find",
    "name": "Find",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 245,
    "cap": "Scroll Up",
    "name": "Scroll Up",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 246,
    "cap": "Scroll Down",
    "name": "Scroll Down",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 247,
    "cap": "Edit",
    "name": "Edit",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 248,
    "cap": "Sleep",
    "name": "Sleep",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 249,
    "cap": "Lock",
    "name": "Lock",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 250,
    "cap": "Refresh",
    "name": "Refresh",
    "group": "other"
  },
  {
    "page": "keyboard",
    "id": 251,
    "cap": "Calculator",
    "name": "Calculator",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 48,
    "cap": "Power",
    "name": "Power",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 49,
    "cap": "Reset",
    "name": "Reset",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 50,
    "cap": "Sleep",
    "name": "Sleep",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 52,
    "cap": "Sleep Mode",
    "name": "Sleep Mode",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 64,
    "cap": "Menu",
    "name": "Menu",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 65,
    "cap": "Menu Pick",
    "name": "Menu Pick",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 66,
    "cap": "Menu Up",
    "name": "Menu Up",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 67,
    "cap": "Menu Down",
    "name": "Menu Down",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 68,
    "cap": "Menu Left",
    "name": "Menu Left",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 69,
    "cap": "Menu Right",
    "name": "Menu Right",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 70,
    "cap": "Menu Escape",
    "name": "Menu Escape",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 71,
    "cap": "Menu Value Increase",
    "name": "Menu Value Increase",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 72,
    "cap": "Menu Value Decrease",
    "name": "Menu Value Decrease",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 96,
    "cap": "Data On Screen",
    "name": "Data On Screen",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 97,
    "cap": "Closed Caption",
    "name": "Closed Caption",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 101,
    "cap": "Snapshot",
    "name": "Snapshot",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 103,
    "cap": "Picture-in-Picture Toggle",
    "name": "Picture-in-Picture Toggle",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 105,
    "cap": "Red Menu Button",
    "name": "Red Menu Button",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 106,
    "cap": "Green Menu Button",
    "name": "Green Menu Button",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 107,
    "cap": "Blue Menu Button",
    "name": "Blue Menu Button",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 108,
    "cap": "Yellow Menu Button",
    "name": "Yellow Menu Button",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 109,
    "cap": "Aspect",
    "name": "Aspect",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 111,
    "cap": "Brightness Up",
    "name": "Brightness Up",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 112,
    "cap": "Brightness Down",
    "name": "Brightness Down",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 114,
    "cap": "Display Backlight Toggle",
    "name": "Display Backlight Toggle",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 115,
    "cap": "Display Set Brightness to Minimum",
    "name": "Display Set Brightness to Minimum",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 116,
    "cap": "Display Set Brightness to Maximum",
    "name": "Display Set Brightness to Maximum",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 117,
    "cap": "Display Set Auto Brightness",
    "name": "Display Set Auto Brightness",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 130,
    "cap": "Mode Step",
    "name": "Mode Step",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 131,
    "cap": "Recall Last",
    "name": "Recall Last",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 136,
    "cap": "Media Select Computer",
    "name": "Media Select Computer",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 137,
    "cap": "Media Select TV",
    "name": "Media Select TV",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 138,
    "cap": "Media Select WWW",
    "name": "Media Select WWW",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 139,
    "cap": "Media Select DVD",
    "name": "Media Select DVD",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 140,
    "cap": "Media Select Telephone",
    "name": "Media Select Telephone",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 141,
    "cap": "Media Select Program Guide",
    "name": "Media Select Program Guide",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 142,
    "cap": "Media Select Video Phone",
    "name": "Media Select Video Phone",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 143,
    "cap": "Media Select Games",
    "name": "Media Select Games",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 144,
    "cap": "Media Select Messages",
    "name": "Media Select Messages",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 145,
    "cap": "Media Select CD",
    "name": "Media Select CD",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 146,
    "cap": "Media Select VCR",
    "name": "Media Select VCR",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 147,
    "cap": "Media Select Tuner",
    "name": "Media Select Tuner",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 148,
    "cap": "Quit",
    "name": "Quit",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 149,
    "cap": "Help",
    "name": "Help",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 150,
    "cap": "Media Select Tape",
    "name": "Media Select Tape",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 151,
    "cap": "Media Select Cable",
    "name": "Media Select Cable",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 152,
    "cap": "Media Select Satellite",
    "name": "Media Select Satellite",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 154,
    "cap": "Media Select Home",
    "name": "Media Select Home",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 156,
    "cap": "Channel Increment",
    "name": "Channel Increment",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 157,
    "cap": "Channel Decrement",
    "name": "Channel Decrement",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 160,
    "cap": "VCR Plus",
    "name": "VCR Plus",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 176,
    "cap": "Play",
    "name": "Play",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 177,
    "cap": "Pause",
    "name": "Pause",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 178,
    "cap": "Record",
    "name": "Record",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 179,
    "cap": "Fast Forward",
    "name": "Fast Forward",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 180,
    "cap": "Rewind",
    "name": "Rewind",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 181,
    "cap": "Next Track",
    "name": "Next Track",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 182,
    "cap": "Previous Track",
    "name": "Previous Track",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 183,
    "cap": "Stop",
    "name": "Stop",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 184,
    "cap": "Eject",
    "name": "Eject",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 185,
    "cap": "Random Play",
    "name": "Random Play",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 188,
    "cap": "Repeat",
    "name": "Repeat",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 191,
    "cap": "Slow Tracking",
    "name": "Slow Tracking",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 204,
    "cap": "Stop/Eject",
    "name": "Stop/Eject",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 205,
    "cap": "Play/Pause",
    "name": "Play/Pause",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 207,
    "cap": "Voice Command",
    "name": "Voice Command",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 226,
    "cap": "Mute",
    "name": "Mute",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 229,
    "cap": "Bass Boost",
    "name": "Bass Boost",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 233,
    "cap": "Volume Up",
    "name": "Volume Up",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 234,
    "cap": "Volume Down",
    "name": "Volume Down",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 245,
    "cap": "Slow",
    "name": "Slow",
    "group": "media"
  },
  {
    "page": "consumer",
    "id": 371,
    "cap": "Alternate Audio Increment",
    "name": "Alternate Audio Increment",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 387,
    "cap": "Consumer Control Configuration",
    "name": "Consumer Control Configuration",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 388,
    "cap": "Word Processor",
    "name": "Word Processor",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 389,
    "cap": "Text Editor",
    "name": "Text Editor",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 390,
    "cap": "Spreadsheet",
    "name": "Spreadsheet",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 391,
    "cap": "Graphics Editor",
    "name": "Graphics Editor",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 392,
    "cap": "Presentation App",
    "name": "Presentation App",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 393,
    "cap": "Database App",
    "name": "Database App",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 394,
    "cap": "Email Reader",
    "name": "Email Reader",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 395,
    "cap": "Newsreader",
    "name": "Newsreader",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 396,
    "cap": "Voicemail",
    "name": "Voicemail",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 397,
    "cap": "Contacts/Address Book",
    "name": "Contacts/Address Book",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 398,
    "cap": "Calendar/Schedule",
    "name": "Calendar/Schedule",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 399,
    "cap": "Task/Project Manager",
    "name": "Task/Project Manager",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 400,
    "cap": "Log/Journal/Timecard",
    "name": "Log/Journal/Timecard",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 401,
    "cap": "Checkbook/Finance",
    "name": "Checkbook/Finance",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 402,
    "cap": "Calculator",
    "name": "Calculator",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 403,
    "cap": "A/V Capture/Playback",
    "name": "A/V Capture/Playback",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 404,
    "cap": "Local Machine Browser",
    "name": "Local Machine Browser",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 406,
    "cap": "Internet Browser",
    "name": "Internet Browser",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 409,
    "cap": "Network Chat",
    "name": "Network Chat",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 412,
    "cap": "Logoff",
    "name": "Logoff",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 414,
    "cap": "Terminal Lock/Screensaver",
    "name": "Terminal Lock/Screensaver",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 415,
    "cap": "Control Panel",
    "name": "Control Panel",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 418,
    "cap": "Select Task/Application",
    "name": "Select Task/Application",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 419,
    "cap": "Next Task/Application",
    "name": "Next Task/Application",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 420,
    "cap": "Previous Task/Application",
    "name": "Previous Task/Application",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 422,
    "cap": "Integrated Help Center",
    "name": "Integrated Help Center",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 423,
    "cap": "Documents",
    "name": "Documents",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 427,
    "cap": "Spell Check",
    "name": "Spell Check",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 430,
    "cap": "Keyboard Layout",
    "name": "Keyboard Layout",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 433,
    "cap": "Screen Saver",
    "name": "Screen Saver",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 436,
    "cap": "File Browser",
    "name": "File Browser",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 438,
    "cap": "Image Browser",
    "name": "Image Browser",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 439,
    "cap": "Audio Browser",
    "name": "Audio Browser",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 440,
    "cap": "Movie Browser",
    "name": "Movie Browser",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 444,
    "cap": "Instant Messaging",
    "name": "Instant Messaging",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 445,
    "cap": "OEM Features/Tips/Tutorial Browser",
    "name": "OEM Features/Tips/Tutorial Browser",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 513,
    "cap": "New",
    "name": "New",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 514,
    "cap": "Open",
    "name": "Open",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 515,
    "cap": "Close",
    "name": "Close",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 516,
    "cap": "Exit",
    "name": "Exit",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 519,
    "cap": "Save",
    "name": "Save",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 520,
    "cap": "Print",
    "name": "Print",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 521,
    "cap": "Properties",
    "name": "Properties",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 538,
    "cap": "Undo",
    "name": "Undo",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 539,
    "cap": "Copy",
    "name": "Copy",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 540,
    "cap": "Cut",
    "name": "Cut",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 541,
    "cap": "Paste",
    "name": "Paste",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 543,
    "cap": "Find",
    "name": "Find",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 545,
    "cap": "Spotlight",
    "name": "Spotlight",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 546,
    "cap": "Go To",
    "name": "Go To",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 547,
    "cap": "Home",
    "name": "Home",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 548,
    "cap": "Back",
    "name": "Back",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 549,
    "cap": "Forward",
    "name": "Forward",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 550,
    "cap": "Stop",
    "name": "Stop",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 551,
    "cap": "Refresh",
    "name": "Refresh",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 554,
    "cap": "Bookmarks",
    "name": "Bookmarks",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 557,
    "cap": "Zoom In",
    "name": "Zoom In",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 558,
    "cap": "Zoom Out",
    "name": "Zoom Out",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 559,
    "cap": "Zoom",
    "name": "Zoom",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 562,
    "cap": "View Toggle",
    "name": "View Toggle",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 563,
    "cap": "Scroll Up",
    "name": "Scroll Up",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 564,
    "cap": "Scroll Down",
    "name": "Scroll Down",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 573,
    "cap": "Edit",
    "name": "Edit",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 607,
    "cap": "Cancel",
    "name": "Cancel",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 617,
    "cap": "Insert Mode",
    "name": "Insert Mode",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 618,
    "cap": "Delete",
    "name": "Delete",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 633,
    "cap": "Redo/Repeat",
    "name": "Redo/Repeat",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 649,
    "cap": "Reply",
    "name": "Reply",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 651,
    "cap": "Forward Msg",
    "name": "Forward Msg",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 652,
    "cap": "Send",
    "name": "Send",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 669,
    "cap": "🌐",
    "name": "Globe",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 671,
    "cap": "Mission Control",
    "name": "Mission Control",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 674,
    "cap": "Apps",
    "name": "Apps",
    "group": "application"
  },
  {
    "page": "consumer",
    "id": 711,
    "cap": "Keyboard Input Assist Previous",
    "name": "Keyboard Input Assist Previous",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 712,
    "cap": "Keyboard Input Assist Next",
    "name": "Keyboard Input Assist Next",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 713,
    "cap": "Keyboard Input Assist Previous Group",
    "name": "Keyboard Input Assist Previous Group",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 714,
    "cap": "Keyboard Input Assist Next Group",
    "name": "Keyboard Input Assist Next Group",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 715,
    "cap": "Keyboard Input Assist Accept",
    "name": "Keyboard Input Assist Accept",
    "group": "other"
  },
  {
    "page": "consumer",
    "id": 716,
    "cap": "Keyboard Input Assist Cancel",
    "name": "Keyboard Input Assist Cancel",
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
