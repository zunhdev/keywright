//! What each key shows on the keyboard canvas, and what it does in one sentence, in Mac
//! terms (design §4.2, §11, VIEW-2, EDIT-4).
//!
//! [`describe`] reads a whole keymap. Each [`Key`] has its [`Keycap`]: a label such as `A`,
//! `⇧⌘4` or `!`, a tap and a hold, a layer key, "does nothing", or an action Keywright
//! doesn't know. Its sentence says the same in words, such as "Tap for Escape, hold for ⌃
//! Control."
//!
//! A "same as layer below" key shows what it inherits, which is what the key does on the
//! next layer down that is on whenever its own layer is: the base layer, and for a layer
//! the firmware turns on by itself, such as Adjust, the layers that turn it on. So on the
//! Sofle, Raise's see-through keys show the base layer's, not Lower's, and Adjust's show
//! Raise's, then Lower's.
//!
//! Keys are labelled as a US keyboard labels them ([`us`]).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use crate::action::{Action, BtCommand, ExtPowerCommand, OutCommand, RgbCommand};
use crate::behavior::BehaviorTable;
use crate::guard::{BASE, ConditionalLayer};
use crate::hid::{HidUsage, Mods, Shortcut, UsagePage};
use crate::keymap::{Keymap, LayerId};
use crate::keys::us;

/// What a key shows and does on one layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
    /// What its keycap shows. For a "same as layer below" key, what it inherits.
    pub keycap: Keycap,
    /// Whether the key is "same as layer below", so [`Key::keycap`] is what it inherits and
    /// is drawn faded, with a ▽ badge.
    pub inherited: bool,
    /// What it does, in one sentence, such as "Tap for Escape, hold for ⌃ Control."
    pub sentence: String,
}

/// What a keycap shows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keycap {
    /// A key or shortcut, such as `A`, `⇧⌘4` or `!`, or an action in a word or two, such
    /// as `Caps Word`.
    Label(String),
    /// A key when tapped, and something else when held.
    TapHold {
        /// The key or shortcut when tapped.
        tap: String,
        /// What holding it does.
        hold: Hold,
    },
    /// A key that turns a layer on.
    Layer {
        /// The layer.
        layer: LayerId,
        /// Its name, as [`layer_name`] gives it.
        name: String,
        /// How the key turns it on.
        mode: LayerMode,
    },
    /// A modifier, such as `⇧`, held for the next key only.
    Sticky(String),
    /// Nothing.
    Nothing,
    /// An action Keywright doesn't know, by the name the firmware gives its behavior.
    Other(String),
}

/// What holding a tap-and-hold key does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hold {
    /// Holds a modifier or a key, such as `⌃`.
    Key(String),
    /// Turns a layer on.
    Layer {
        /// The layer.
        layer: LayerId,
        /// Its name, as [`layer_name`] gives it.
        name: String,
    },
}

/// How a layer key turns its layer on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerMode {
    /// While the key is held.
    Hold,
    /// Switches to it, turning the others off.
    Switch,
    /// Turns it on, or off again.
    Toggle,
    /// For the next key only.
    NextKey,
}

/// What every key shows and does: layer by layer, in the keymap's order, then by key
/// position. `conditional_layers` are the layers the firmware turns on by itself, from the
/// keyboard's support pack.
#[must_use]
pub fn describe(
    keymap: &Keymap,
    behaviors: &BehaviorTable,
    conditional_layers: &[ConditionalLayer],
) -> Vec<Vec<Key>> {
    let words = Words { keymap, behaviors };
    keymap
        .layers
        .iter()
        .enumerate()
        .map(|(index, layer)| {
            let below = layers_below(keymap, conditional_layers, index);
            layer
                .bindings
                .iter()
                .enumerate()
                .map(|(position, binding)| {
                    let action = Action::from_binding(*binding, behaviors);
                    if action != Action::Transparent {
                        return Key {
                            keycap: words.keycap(action),
                            inherited: false,
                            sentence: words.sentence(action),
                        };
                    }
                    let inherited = inherited(keymap, behaviors, &below, position);
                    let sentence = words.sentence(inherited);
                    Key {
                        keycap: words.keycap(inherited),
                        inherited: true,
                        sentence: format!(
                            "Same as the layer below: {}",
                            lowercase_first(&sentence)
                        ),
                    }
                })
                .collect()
        })
        .collect()
}

/// The layers a key on the layer at `index` falls through to, from the nearest: those
/// below it, down to the base layer, that are on whenever it is.
fn layers_below(
    keymap: &Keymap,
    conditional_layers: &[ConditionalLayer],
    index: usize,
) -> Vec<LayerId> {
    let Some(layer) = keymap.layers.get(index) else {
        return Vec::new();
    };
    // The base layer is always on. A layer the firmware turns on by itself is on only when
    // its conditions are, which may be conditional layers too.
    let mut on = BTreeSet::from([BASE, layer.id]);
    loop {
        let before = on.len();
        for conditional in conditional_layers {
            if on.contains(&conditional.then_layer) {
                on.extend(conditional.if_layers.iter().copied());
            }
        }
        if on.len() == before {
            break;
        }
    }
    // Layers below the base layer in the order are never consulted.
    let Some(base) = keymap.layers.iter().position(|layer| layer.id == BASE) else {
        return Vec::new();
    };
    keymap
        .layers
        .get(base..index)
        .unwrap_or_default()
        .iter()
        .rev()
        .map(|layer| layer.id)
        .filter(|id| on.contains(id))
        .collect()
}

/// What a "same as layer below" key does: the first action other than "same as layer
/// below" on the layers it falls through to, or nothing.
fn inherited(
    keymap: &Keymap,
    behaviors: &BehaviorTable,
    below: &[LayerId],
    position: usize,
) -> Action {
    below
        .iter()
        .filter_map(|id| keymap.layer(*id)?.bindings.get(position))
        .map(|binding| Action::from_binding(*binding, behaviors))
        .find(|action| *action != Action::Transparent)
        .unwrap_or(Action::None)
}

/// "Types Q." as "types Q.", to follow a colon. Every sentence starts with a verb.
fn lowercase_first(sentence: &str) -> String {
    let mut chars = sentence.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_lowercase().chain(chars).collect()
    })
}

/// A keymap's words for its actions, which name its layers.
struct Words<'a> {
    keymap: &'a Keymap,
    behaviors: &'a BehaviorTable,
}

impl Words<'_> {
    fn keycap(&self, action: Action) -> Keycap {
        let layer = |layer: LayerId, mode: LayerMode| Keycap::Layer {
            layer,
            name: layer_name(self.keymap, layer),
            mode,
        };
        match action {
            Action::KeyPress(shortcut) => Keycap::Label(shortcut_label(shortcut)),
            Action::ModTap { hold, tap } => Keycap::TapHold {
                tap: shortcut_label(tap),
                hold: Hold::Key(shortcut_label(hold)),
            },
            Action::LayerTap { layer, tap } => Keycap::TapHold {
                tap: shortcut_label(tap),
                hold: Hold::Layer {
                    layer,
                    name: layer_name(self.keymap, layer),
                },
            },
            Action::Momentary(id) => layer(id, LayerMode::Hold),
            Action::ToLayer(id) => layer(id, LayerMode::Switch),
            Action::Toggle(id) => layer(id, LayerMode::Toggle),
            Action::StickyLayer(id) => layer(id, LayerMode::NextKey),
            Action::StickyKey(shortcut) => Keycap::Sticky(shortcut_label(shortcut)),
            // `describe` shows what the key inherits instead.
            Action::Transparent | Action::None => Keycap::Nothing,
            Action::Bluetooth(command) => Keycap::Label(bluetooth_cap(command)),
            Action::Output(command) => Keycap::Label(output_words(command).0.to_owned()),
            Action::Lighting(command) => Keycap::Label(lighting_words(command).0.to_owned()),
            Action::ExternalPower(command) => Keycap::Label(power_words(command).0.to_owned()),
            Action::CapsWord => Keycap::Label("Caps Word".to_owned()),
            Action::KeyRepeat => Keycap::Label("Repeat".to_owned()),
            Action::Unlock => Keycap::Label("Unlock".to_owned()),
            Action::Bootloader => Keycap::Label("Update mode".to_owned()),
            Action::Reset => Keycap::Label("Restart".to_owned()),
            Action::Other { behavior, .. } => Keycap::Other(
                self.behaviors
                    .get(behavior)
                    .map_or_else(|| "Unknown".to_owned(), |known| known.name.clone()),
            ),
        }
    }

    fn sentence(&self, action: Action) -> String {
        let layer = |id| layer_phrase(self.keymap, id);
        match action {
            Action::KeyPress(shortcut) if modifier_bits(shortcut.usage).is_some() => {
                format!("Acts as {}.", shortcut_name(shortcut))
            }
            Action::KeyPress(shortcut) if shortcut.usage.page == UsagePage::Consumer => {
                format!("Presses {}.", shortcut_name(shortcut))
            }
            Action::KeyPress(shortcut) => format!("Types {}.", shortcut_name(shortcut)),
            Action::ModTap { hold, tap } => format!(
                "Tap for {}, hold for {}.",
                shortcut_name(tap),
                shortcut_name(hold)
            ),
            Action::LayerTap { layer: id, tap } => {
                format!("Tap for {}, hold to use {}.", shortcut_name(tap), layer(id))
            }
            Action::Momentary(id) => format!("Hold to use {}.", layer(id)),
            Action::ToLayer(id) => format!("Switches to {}.", layer(id)),
            Action::Toggle(id) => format!("Turns {} on or off.", layer(id)),
            Action::StickyLayer(id) => format!("Uses {} for the next key.", layer(id)),
            Action::StickyKey(shortcut) => {
                format!("Holds {} for the next key.", shortcut_name(shortcut))
            }
            Action::Transparent => "Same as the layer below.".to_owned(),
            Action::None => "Does nothing.".to_owned(),
            Action::Bluetooth(command) => bluetooth_sentence(command),
            Action::Output(command) => output_words(command).1.to_owned(),
            Action::Lighting(command) => lighting_words(command).1.to_owned(),
            Action::ExternalPower(command) => power_words(command).1.to_owned(),
            Action::CapsWord => "Types capitals until the end of the word.".to_owned(),
            Action::KeyRepeat => "Repeats the last key.".to_owned(),
            Action::Unlock => "Unlocks the keyboard, so Keywright can change it.".to_owned(),
            Action::Bootloader => {
                "Puts the keyboard into update mode, for new firmware.".to_owned()
            }
            Action::Reset => "Restarts the keyboard.".to_owned(),
            Action::Other { behavior, .. } => match self.behaviors.get(behavior) {
                Some(known) => format!("Runs the firmware's {} action.", known.name),
                None => "Runs an action the firmware doesn't describe.".to_owned(),
            },
        }
    }
}

/// A layer's name: its own, or `Layer 3` for the third, unnamed. A layer key can point at
/// a layer the keymap doesn't have, such as one that was removed: that's `Missing layer`.
#[must_use]
pub fn layer_name(keymap: &Keymap, id: LayerId) -> String {
    match keymap
        .layers
        .iter()
        .enumerate()
        .find(|(_, layer)| layer.id == id)
    {
        Some((_, layer)) if !layer.name.is_empty() => layer.name.clone(),
        Some((index, _)) => format!("Layer {}", index.saturating_add(1)),
        None => "Missing layer".to_owned(),
    }
}

/// A layer in a sentence: "the Lower layer", "layer 3", or "a missing layer".
fn layer_phrase(keymap: &Keymap, id: LayerId) -> String {
    match keymap
        .layers
        .iter()
        .enumerate()
        .find(|(_, layer)| layer.id == id)
    {
        Some((_, layer)) if !layer.name.is_empty() => format!("the {} layer", layer.name),
        Some((index, _)) => format!("layer {}", index.saturating_add(1)),
        None => "a missing layer".to_owned(),
    }
}

/// The modifiers in the order Apple shows them, ⌃⌥⇧⌘, each left or right.
const MODIFIERS: [(u8, &str, &str); 4] = [
    (
        Mods::LEFT_CONTROL.bits() | Mods::RIGHT_CONTROL.bits(),
        "⌃",
        "Control",
    ),
    (
        Mods::LEFT_OPTION.bits() | Mods::RIGHT_OPTION.bits(),
        "⌥",
        "Option",
    ),
    (
        Mods::LEFT_SHIFT.bits() | Mods::RIGHT_SHIFT.bits(),
        "⇧",
        "Shift",
    ),
    (
        Mods::LEFT_COMMAND.bits() | Mods::RIGHT_COMMAND.bits(),
        "⌘",
        "Command",
    ),
];

/// The modifiers in `bits`, in Apple's order, each with its symbol and name.
fn held(bits: u8) -> impl Iterator<Item = (&'static str, &'static str)> {
    MODIFIERS
        .iter()
        .filter(move |(mask, ..)| bits & mask != 0)
        .map(|(_, symbol, name)| (*symbol, *name))
}

/// The modifiers' symbols, such as `⇧⌘`.
fn symbols(bits: u8) -> String {
    held(bits).map(|(symbol, _)| symbol).collect()
}

/// The modifier bit of a modifier key, such as left Shift's.
fn modifier_bits(usage: HidUsage) -> Option<u8> {
    match (usage.page, usage.id) {
        (UsagePage::Keyboard, id @ 0xE0..=0xE7) => Some(1 << (id - 0xE0)),
        _ => None,
    }
}

/// Whether Shift is the only modifier, so a key can show what it types with it.
const fn only_shift(mods: Mods) -> bool {
    let shift = Mods::LEFT_SHIFT.bits() | Mods::RIGHT_SHIFT.bits();
    !mods.is_empty() && mods.bits() & !shift == 0
}

/// What a key is called, or its usage for a key without a name, such as `Key 07:00A5`.
fn names(usage: HidUsage) -> (String, String) {
    us::label(usage).map_or_else(
        || {
            let unknown = format!("Key {:02X}:{:04X}", usage.page.number(), usage.id);
            (unknown.clone(), unknown)
        },
        |label| (label.cap.to_owned(), label.name.to_owned()),
    )
}

/// A key or shortcut as a keycap shows it: the modifiers in Apple's order, then the key,
/// such as `⇧⌘4`. With Shift alone, a key shows what it types, such as `!`, and a modifier
/// key shows only modifiers, such as `⌃⇧`.
#[must_use]
pub fn shortcut_label(shortcut: Shortcut) -> String {
    let mods = shortcut.mods;
    if let Some(bits) = modifier_bits(shortcut.usage) {
        return symbols(mods.bits() | bits);
    }
    if only_shift(mods)
        && let Some(shifted) = us::label(shortcut.usage).and_then(|label| label.shifted)
    {
        return shifted.to_owned();
    }
    let (cap, _) = names(shortcut.usage);
    format!("{}{cap}", symbols(mods.bits()))
}

/// A key or shortcut as text calls it: a key by its name, such as `Escape`, a modifier by
/// its symbol and name, such as `⌃ Control` or `⌃⇧ Control-Shift`, and a shortcut as a
/// keycap shows it, such as `⌘C`.
#[must_use]
pub fn shortcut_name(shortcut: Shortcut) -> String {
    let mods = shortcut.mods;
    if let Some(bits) = modifier_bits(shortcut.usage) {
        let bits = mods.bits() | bits;
        let names: Vec<_> = held(bits).map(|(_, name)| name).collect();
        return format!("{} {}", symbols(bits), names.join("-"));
    }
    if mods.is_empty() {
        let (_, name) = names(shortcut.usage);
        return name;
    }
    shortcut_label(shortcut)
}

/// A Bluetooth command on a keycap. ZMK numbers devices from 0, people from 1.
fn bluetooth_cap(command: BtCommand) -> String {
    match command {
        BtCommand::Select(profile) => format!("ᛒ {}", profile.saturating_add(1)),
        BtCommand::Next => "ᛒ Next".to_owned(),
        BtCommand::Previous => "ᛒ Previous".to_owned(),
        BtCommand::Clear => "ᛒ Clear".to_owned(),
        BtCommand::ClearAll => "ᛒ Clear all".to_owned(),
        BtCommand::Disconnect(profile) => format!("ᛒ {} off", profile.saturating_add(1)),
    }
}

fn bluetooth_sentence(command: BtCommand) -> String {
    match command {
        BtCommand::Select(profile) => {
            format!("Uses Bluetooth device {}.", profile.saturating_add(1))
        }
        BtCommand::Next => "Uses the next Bluetooth device.".to_owned(),
        BtCommand::Previous => "Uses the previous Bluetooth device.".to_owned(),
        BtCommand::Clear => "Forgets the pairing of the Bluetooth device in use.".to_owned(),
        BtCommand::ClearAll => "Forgets every Bluetooth pairing.".to_owned(),
        BtCommand::Disconnect(profile) => format!(
            "Disconnects Bluetooth device {}, keeping its pairing.",
            profile.saturating_add(1)
        ),
    }
}

/// Where the keyboard types: on a keycap, and in a sentence.
const fn output_words(command: OutCommand) -> (&'static str, &'static str) {
    match command {
        OutCommand::Toggle => (
            "USB or ᛒ",
            "Switches between typing over USB and over Bluetooth.",
        ),
        OutCommand::Usb => ("Use USB", "Types over USB."),
        OutCommand::Bluetooth => ("Use ᛒ", "Types over Bluetooth."),
    }
}

/// A lighting command: on a keycap, and in a sentence.
const fn lighting_words(command: RgbCommand) -> (&'static str, &'static str) {
    match command {
        RgbCommand::Toggle => ("Lights", "Turns the lights on or off."),
        RgbCommand::On => ("Lights on", "Turns the lights on."),
        RgbCommand::Off => ("Lights off", "Turns the lights off."),
        RgbCommand::HueUp => ("Hue +", "Shifts the lights' colour one way."),
        RgbCommand::HueDown => ("Hue −", "Shifts the lights' colour the other way."),
        RgbCommand::SaturationUp => ("Saturation +", "Makes the lights' colour stronger."),
        RgbCommand::SaturationDown => ("Saturation −", "Makes the lights' colour paler."),
        RgbCommand::BrightnessUp => ("Brightness +", "Makes the lights brighter."),
        RgbCommand::BrightnessDown => ("Brightness −", "Makes the lights dimmer."),
        RgbCommand::SpeedUp => ("Speed +", "Speeds up the lights' effect."),
        RgbCommand::SpeedDown => ("Speed −", "Slows down the lights' effect."),
        RgbCommand::NextEffect => ("Next effect", "Switches the lights to the next effect."),
        RgbCommand::PreviousEffect => (
            "Previous effect",
            "Switches the lights to the previous effect.",
        ),
    }
}

/// A command for the power to the lights and display: on a keycap, and in a sentence.
const fn power_words(command: ExtPowerCommand) -> (&'static str, &'static str) {
    match command {
        ExtPowerCommand::Toggle => (
            "Power",
            "Turns the power to the lights and display on or off.",
        ),
        ExtPowerCommand::On => ("Power on", "Turns the power to the lights and display on."),
        ExtPowerCommand::Off => (
            "Power off",
            "Turns the power to the lights and display off.",
        ),
    }
}
