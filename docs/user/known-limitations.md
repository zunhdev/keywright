# Known limitations

Keywright changes your keyboard live, over ZMK Studio, the way your keyboard's stock firmware lets apps edit it. Some things aren't open to that, so this version of Keywright can't change them. Most are planned for a future version, which will build new firmware for your keyboard.

| Keywright can't yet…                                          | Because                                                                                                     |
| ------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| Make combos (keys pressed together), macros or tap-dance keys | The firmware doesn't let apps edit them.                                                                    |
| Change how long counts as a hold                              | It's set in the firmware: 200 milliseconds on the Sofle's stock firmware.                                   |
| Change what turning the knobs does                            | The firmware doesn't let apps edit it. Pressing a knob is a key, which you can change.                      |
| Change the lights' defaults, sleep timeouts or the displays   | They're set in the firmware.                                                                                |
| Add a fifth layer on the Sofle                                | The stock firmware has room for 4, all in use.                                                              |
| Change when Adjust turns on (lower and raise together)        | The firmware doesn't let apps edit it. Keywright shows it.                                                  |
| Show which layer is on, or which keys you press               | The keyboard doesn't report them.                                                                           |
| Edit over the right half's cable                              | Only the left half talks to the computer. Keywright says so if you try.                                     |
| Edit over the cable while the keyboard types over Bluetooth   | The keyboard can be edited only over the connection it types over, and locks when that changes.             |
| Show the right half's battery                                 | The stock firmware doesn't report it.                                                                       |
| Show the battery without Bluetooth                            | Keyboards report their battery over Bluetooth only.                                                         |
| Send the Globe key itself                                     | macOS only takes Globe from Apple's keyboards. Keywright makes a key you choose act as Globe while it runs. |
