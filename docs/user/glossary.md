# Glossary

Keywright uses plain words for what keyboard firmware has names of its own for. If you've used ZMK, or read about it, this table matches them up. **Keywright › Settings… › General › Show ZMK names** shows ZMK's names in the app too.

| Keywright says              | ZMK says                   | What it means                                                                                                                 |
| --------------------------- | -------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Action                      | Behavior binding           | What a key does.                                                                                                              |
| Key or shortcut             | `&kp`, Key Press           | Types a key, with or without modifiers, such as ⌘C.                                                                           |
| Tap for X, hold for ⌃       | `&mt`, Mod-Tap             | A key when tapped, a modifier while held.                                                                                     |
| Tap for X, hold for a layer | `&lt`, Layer-Tap           | A key when tapped, a layer while held.                                                                                        |
| Hold for a layer            | `&mo`, Momentary Layer     | The layer is on while you hold the key.                                                                                       |
| Switch to a layer           | `&to`, To Layer            | Makes a layer the one that's on.                                                                                              |
| Toggle a layer              | `&tog`, Toggle Layer       | Turns a layer on, or off again.                                                                                               |
| Layer for the next key      | `&sl`, Sticky Layer        | The layer is on for the next key only.                                                                                        |
| Sticky modifier             | `&sk`, Sticky Key          | Holds a modifier for the next key only.                                                                                       |
| Same as layer below         | `&trans`, Transparent      | The key does what the next layer down does.                                                                                   |
| Does nothing                | `&none`, None              | The key does nothing on this layer, and the layers below don't get it.                                                        |
| Unlock editing              | `&studio_unlock`           | Unlocks the keyboard, so Keywright can change it.                                                                             |
| Update mode                 | `&bootloader`              | Puts a half into update mode, for new firmware.                                                                               |
| Left half / right half      | Central / peripheral       | On a split keyboard, the left half talks to the computer, and the right half to the left.                                     |
| Trying / saved              | Unsaved changes / saved    | A change works at once, but lasts through a restart only once saved.                                                          |
| Base layer                  | Layer 0, the default layer | What the keys do with no layer keys held.                                                                                     |
| Backup                      | —                          | A `.keywright` file with a keyboard's whole keymap.                                                                           |
| Support pack                | —                          | What Keywright knows about a keyboard model that the keyboard can't say itself: its name, how to unlock it, its stock keymap. |
| Stock                       | —                          | As your keyboard came from its maker.                                                                                         |
