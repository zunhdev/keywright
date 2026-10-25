//! Actions as the inspector edits them (design §5.4, EDIT-1, EDIT-2): what a key does, in a
//! form the UI can show in its editors and send back with `set_action`, and what the
//! keyboard's firmware offers.
//!
//! An [`Action`] says the same as the core's, field for field: the app converts it both
//! ways, and the core turns it into the keyboard's binding. The words for each choice come
//! from the core too, as on the canvas.

use keywright_core::action::{self as core, BtCommand, ExtPowerCommand, OutCommand, RgbCommand};
use keywright_core::behavior::{BehaviorId, BehaviorKind, BehaviorTable, ValueKind};
use keywright_core::hid::{self, HidUsage, Mods, UsagePage};
use keywright_core::keycap;
use keywright_core::keymap::{Keymap, LayerId};
use keywright_core::keys::source::Labels;
use keywright_core::keys::{self, KeyCategory};
use keywright_core::mac::{self, presets};
use serde::{Deserialize, Serialize};
use specta::Type;

use super::AppState;

/// A key or shortcut: a key, with modifiers held.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct Shortcut {
    /// The modifiers held, as ZMK's bits: 1 left Control, 2 left Shift, 4 left Option, 8
    /// left Command, then 16 to 128 for the right ones.
    pub mods: u8,
    /// The key's usage page.
    pub page: Page,
    /// The key's usage ID on its page.
    pub id: u16,
}

/// A usage page ZMK can send.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum Page {
    /// Keyboard keys.
    Keyboard,
    /// Consumer controls, such as media keys.
    Consumer,
}

/// What a key does (tech spec §8.1). In TypeScript, `{ kind: "key-press", key }` and so on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Action {
    /// Types a key or shortcut.
    KeyPress {
        /// The key or shortcut.
        key: Shortcut,
    },
    /// A key on tap, another on hold.
    ModTap {
        /// What holding it holds.
        hold: Shortcut,
        /// What tapping it types.
        tap: Shortcut,
    },
    /// A key on tap, a layer on hold.
    LayerTap {
        /// The layer, by ID.
        layer: u32,
        /// What tapping it types.
        tap: Shortcut,
    },
    /// A layer while held.
    Momentary {
        /// The layer, by ID.
        layer: u32,
    },
    /// Switches to a layer.
    ToLayer {
        /// The layer, by ID.
        layer: u32,
    },
    /// Turns a layer on or off.
    Toggle {
        /// The layer, by ID.
        layer: u32,
    },
    /// A layer for the next key only.
    StickyLayer {
        /// The layer, by ID.
        layer: u32,
    },
    /// A key, usually a modifier, held for the next key only.
    StickyKey {
        /// The key.
        key: Shortcut,
    },
    /// Same as the layer below.
    Transparent,
    /// Nothing.
    Nothing,
    /// A Bluetooth command.
    Bluetooth {
        /// The command.
        command: Bluetooth,
        /// The device it's about, from 0, for `select` and `disconnect`; otherwise 0.
        profile: u32,
    },
    /// Where the keyboard types.
    Output {
        /// Where.
        command: Output,
    },
    /// A lighting command.
    Lighting {
        /// The command.
        command: Lighting,
    },
    /// Power to the lights and display.
    ExternalPower {
        /// The command.
        command: Power,
    },
    /// Capitals until the end of the word.
    CapsWord,
    /// Repeats the last key.
    KeyRepeat,
    /// Unlocks the keyboard for editing.
    Unlock,
    /// Puts the half into update mode.
    Bootloader,
    /// Restarts the half.
    Reset,
    /// A behavior Keywright doesn't know, with its parameters.
    Other {
        /// The behavior's ID on this keyboard.
        behavior: u32,
        /// The first parameter.
        param1: Param,
        /// The second parameter.
        param2: Param,
    },
}

/// A parameter of a behavior Keywright doesn't know: a number, or a key where the
/// behavior's metadata says it takes one, so the UI never encodes keys itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Param {
    /// A number: a named constant, a number in a range, or a layer's ID.
    Number {
        /// The number.
        value: u32,
    },
    /// A key or shortcut.
    Key {
        /// The key or shortcut.
        key: Shortcut,
    },
}

impl Param {
    /// The parameter as the firmware takes it.
    fn encoded(self) -> u32 {
        match self {
            Self::Number { value } => value,
            Self::Key { key } => hid::Shortcut::from(key).to_param(),
        }
    }
}

impl Action {
    /// What a key does, as the UI shows it: a behavior Keywright doesn't know has its
    /// parameters as keys where its metadata says it takes a key there.
    pub fn of(action: core::Action, behaviors: &BehaviorTable) -> Self {
        let Self::Other {
            behavior,
            param1,
            param2,
        } = Self::from(action)
        else {
            return Self::from(action);
        };
        let sets = behaviors
            .get(BehaviorId(behavior))
            .map_or(&[][..], |known| known.parameters.as_slice());
        let takes_key = |values: fn(&keywright_core::behavior::ParameterSet) -> &[_]| {
            sets.iter()
                .flat_map(values)
                .any(|value: &keywright_core::behavior::ParameterValue| {
                    matches!(value.kind, ValueKind::HidUsage { .. })
                })
        };
        let as_key = |param: Param, key: bool| match param {
            Param::Number { value } if key => {
                hid::Shortcut::from_param(value).map_or(param, |shortcut| Param::Key {
                    key: shortcut.into(),
                })
            }
            _ => param,
        };
        Self::Other {
            behavior,
            param1: as_key(param1, takes_key(|set| &set.param1)),
            param2: as_key(param2, takes_key(|set| &set.param2)),
        }
    }
}

/// A Bluetooth command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum Bluetooth {
    /// Forgets the pairing of the device in use.
    Clear,
    /// The next device.
    Next,
    /// The previous device.
    Previous,
    /// A device, by `profile`.
    Select,
    /// Forgets every pairing.
    ClearAll,
    /// Disconnects a device, by `profile`.
    Disconnect,
}

/// Where the keyboard types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum Output {
    /// Switches between USB and Bluetooth.
    Toggle,
    /// USB.
    Usb,
    /// Bluetooth.
    Bluetooth,
}

/// A lighting command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum Lighting {
    /// Turns the lights on or off.
    Toggle,
    /// Turns them on.
    On,
    /// Turns them off.
    Off,
    /// Shifts their colour one way.
    HueUp,
    /// Shifts their colour the other way.
    HueDown,
    /// Makes their colour stronger.
    SaturationUp,
    /// Makes their colour paler.
    SaturationDown,
    /// Makes them brighter.
    BrightnessUp,
    /// Makes them dimmer.
    BrightnessDown,
    /// Speeds up their effect.
    SpeedUp,
    /// Slows down their effect.
    SpeedDown,
    /// The next effect.
    NextEffect,
    /// The previous effect.
    PreviousEffect,
}

/// A command for the power to the lights and display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum Power {
    /// Turns it on or off.
    Toggle,
    /// Turns it on.
    On,
    /// Turns it off.
    Off,
}

impl From<UsagePage> for Page {
    fn from(page: UsagePage) -> Self {
        match page {
            UsagePage::Keyboard => Self::Keyboard,
            UsagePage::Consumer => Self::Consumer,
        }
    }
}

impl From<Page> for UsagePage {
    fn from(page: Page) -> Self {
        match page {
            Page::Keyboard => Self::Keyboard,
            Page::Consumer => Self::Consumer,
        }
    }
}

impl From<hid::Shortcut> for Shortcut {
    fn from(shortcut: hid::Shortcut) -> Self {
        Self {
            mods: shortcut.mods.bits(),
            page: shortcut.usage.page.into(),
            id: shortcut.usage.id,
        }
    }
}

impl From<Shortcut> for hid::Shortcut {
    fn from(shortcut: Shortcut) -> Self {
        Self {
            mods: Mods::from_bits(shortcut.mods),
            usage: HidUsage {
                page: shortcut.page.into(),
                id: shortcut.id,
            },
        }
    }
}

impl From<core::Action> for Action {
    fn from(action: core::Action) -> Self {
        match action {
            core::Action::KeyPress(key) => Self::KeyPress { key: key.into() },
            core::Action::ModTap { hold, tap } => Self::ModTap {
                hold: hold.into(),
                tap: tap.into(),
            },
            core::Action::LayerTap { layer, tap } => Self::LayerTap {
                layer: layer.0,
                tap: tap.into(),
            },
            core::Action::Momentary(layer) => Self::Momentary { layer: layer.0 },
            core::Action::ToLayer(layer) => Self::ToLayer { layer: layer.0 },
            core::Action::Toggle(layer) => Self::Toggle { layer: layer.0 },
            core::Action::StickyLayer(layer) => Self::StickyLayer { layer: layer.0 },
            core::Action::StickyKey(key) => Self::StickyKey { key: key.into() },
            core::Action::Transparent => Self::Transparent,
            core::Action::None => Self::Nothing,
            core::Action::Bluetooth(command) => {
                let (command, profile) = match command {
                    BtCommand::Clear => (Bluetooth::Clear, 0),
                    BtCommand::Next => (Bluetooth::Next, 0),
                    BtCommand::Previous => (Bluetooth::Previous, 0),
                    BtCommand::Select(profile) => (Bluetooth::Select, profile),
                    BtCommand::ClearAll => (Bluetooth::ClearAll, 0),
                    BtCommand::Disconnect(profile) => (Bluetooth::Disconnect, profile),
                };
                Self::Bluetooth { command, profile }
            }
            core::Action::Output(command) => Self::Output {
                command: match command {
                    OutCommand::Toggle => Output::Toggle,
                    OutCommand::Usb => Output::Usb,
                    OutCommand::Bluetooth => Output::Bluetooth,
                },
            },
            core::Action::Lighting(command) => Self::Lighting {
                command: lighting(command),
            },
            core::Action::ExternalPower(command) => Self::ExternalPower {
                command: match command {
                    ExtPowerCommand::Toggle => Power::Toggle,
                    ExtPowerCommand::On => Power::On,
                    ExtPowerCommand::Off => Power::Off,
                },
            },
            core::Action::CapsWord => Self::CapsWord,
            core::Action::KeyRepeat => Self::KeyRepeat,
            core::Action::Unlock => Self::Unlock,
            core::Action::Bootloader => Self::Bootloader,
            core::Action::Reset => Self::Reset,
            core::Action::Other {
                behavior,
                param1,
                param2,
            } => Self::Other {
                behavior: behavior.0,
                param1: Param::Number { value: param1 },
                param2: Param::Number { value: param2 },
            },
        }
    }
}

impl From<Action> for core::Action {
    fn from(action: Action) -> Self {
        match action {
            Action::KeyPress { key } => Self::KeyPress(key.into()),
            Action::ModTap { hold, tap } => Self::ModTap {
                hold: hold.into(),
                tap: tap.into(),
            },
            Action::LayerTap { layer, tap } => Self::LayerTap {
                layer: LayerId(layer),
                tap: tap.into(),
            },
            Action::Momentary { layer } => Self::Momentary(LayerId(layer)),
            Action::ToLayer { layer } => Self::ToLayer(LayerId(layer)),
            Action::Toggle { layer } => Self::Toggle(LayerId(layer)),
            Action::StickyLayer { layer } => Self::StickyLayer(LayerId(layer)),
            Action::StickyKey { key } => Self::StickyKey(key.into()),
            Action::Transparent => Self::Transparent,
            Action::Nothing => Self::None,
            Action::Bluetooth { command, profile } => Self::Bluetooth(match command {
                Bluetooth::Clear => BtCommand::Clear,
                Bluetooth::Next => BtCommand::Next,
                Bluetooth::Previous => BtCommand::Previous,
                Bluetooth::Select => BtCommand::Select(profile),
                Bluetooth::ClearAll => BtCommand::ClearAll,
                Bluetooth::Disconnect => BtCommand::Disconnect(profile),
            }),
            Action::Output { command } => Self::Output(match command {
                Output::Toggle => OutCommand::Toggle,
                Output::Usb => OutCommand::Usb,
                Output::Bluetooth => OutCommand::Bluetooth,
            }),
            Action::Lighting { command } => Self::Lighting(rgb(command)),
            Action::ExternalPower { command } => Self::ExternalPower(match command {
                Power::Toggle => ExtPowerCommand::Toggle,
                Power::On => ExtPowerCommand::On,
                Power::Off => ExtPowerCommand::Off,
            }),
            Action::CapsWord => Self::CapsWord,
            Action::KeyRepeat => Self::KeyRepeat,
            Action::Unlock => Self::Unlock,
            Action::Bootloader => Self::Bootloader,
            Action::Reset => Self::Reset,
            Action::Other {
                behavior,
                param1,
                param2,
            } => Self::Other {
                behavior: BehaviorId(behavior),
                param1: param1.encoded(),
                param2: param2.encoded(),
            },
        }
    }
}

/// The lighting commands, in both forms, in the order the inspector lists them.
const LIGHTING: [(Lighting, RgbCommand); 13] = [
    (Lighting::Toggle, RgbCommand::Toggle),
    (Lighting::On, RgbCommand::On),
    (Lighting::Off, RgbCommand::Off),
    (Lighting::BrightnessUp, RgbCommand::BrightnessUp),
    (Lighting::BrightnessDown, RgbCommand::BrightnessDown),
    (Lighting::HueUp, RgbCommand::HueUp),
    (Lighting::HueDown, RgbCommand::HueDown),
    (Lighting::SaturationUp, RgbCommand::SaturationUp),
    (Lighting::SaturationDown, RgbCommand::SaturationDown),
    (Lighting::NextEffect, RgbCommand::NextEffect),
    (Lighting::PreviousEffect, RgbCommand::PreviousEffect),
    (Lighting::SpeedUp, RgbCommand::SpeedUp),
    (Lighting::SpeedDown, RgbCommand::SpeedDown),
];

fn lighting(command: RgbCommand) -> Lighting {
    LIGHTING
        .iter()
        .find(|(_, rgb)| *rgb == command)
        .map_or(Lighting::Toggle, |(lighting, _)| *lighting)
}

fn rgb(command: Lighting) -> RgbCommand {
    LIGHTING
        .iter()
        .find(|(lighting, _)| *lighting == command)
        .map_or(RgbCommand::Toggle, |(_, rgb)| *rgb)
}

/// A kind of action the keyboard's firmware has a behavior for (EDIT-2): the inspector
/// offers only those.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    /// Types a key or shortcut.
    KeyPress,
    /// A key on tap, a modifier on hold.
    ModTap,
    /// A key on tap, a layer on hold.
    LayerTap,
    /// A layer while held.
    Momentary,
    /// Switches to a layer.
    ToLayer,
    /// Turns a layer on or off.
    Toggle,
    /// A layer for the next key.
    StickyLayer,
    /// A modifier for the next key.
    StickyKey,
    /// Same as the layer below.
    Transparent,
    /// Nothing.
    Nothing,
}

/// The kinds of action, of those the inspector edits beyond its choices, that the
/// keyboard's firmware has a behavior for.
pub fn kinds(behaviors: &BehaviorTable) -> Vec<Kind> {
    [
        (BehaviorKind::KeyPress, Kind::KeyPress),
        (BehaviorKind::ModTap, Kind::ModTap),
        (BehaviorKind::LayerTap, Kind::LayerTap),
        (BehaviorKind::MomentaryLayer, Kind::Momentary),
        (BehaviorKind::ToLayer, Kind::ToLayer),
        (BehaviorKind::ToggleLayer, Kind::Toggle),
        (BehaviorKind::StickyLayer, Kind::StickyLayer),
        (BehaviorKind::StickyKey, Kind::StickyKey),
        (BehaviorKind::Transparent, Kind::Transparent),
        (BehaviorKind::None, Kind::Nothing),
    ]
    .into_iter()
    .filter(|(behavior, _)| behaviors.find(*behavior).is_some())
    .map(|(_, kind)| kind)
    .collect()
}

/// A group of actions in the action picker (EDIT-1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    /// Mac actions: Mission Control, Spotlight, Lock Screen and the like (MAC-4).
    Mac,
    /// Media & system: playback, volume and brightness.
    Media,
    /// Bluetooth & connection.
    Bluetooth,
    /// Lighting.
    Lighting,
    /// Special: Caps Word, Repeat, Unlock editing, Update mode, Restart.
    Special,
}

/// An action the inspector offers as it is, such as Play/Pause or Bluetooth device 1, in
/// the words the keycap and the inspector use.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    /// The group it's listed in.
    pub category: Category,
    /// What a keycap shows for it, such as `Play/Pause` or `ᛒ 1`.
    pub label: String,
    /// What it does, in one sentence.
    pub sentence: String,
    /// The action.
    pub action: Action,
    /// The name of its icon in the UI's icon set, for a Mac action.
    pub icon: Option<String>,
    /// For a Mac action that relies on a default macOS shortcut, which people can change,
    /// that shortcut, such as `⌃⌘Q` for Lock Screen (design §5.4).
    pub default_shortcut: Option<String>,
}

/// The media keys the inspector lists (design §5.4): play/pause, next, previous, volume
/// up, down and mute, and brightness up and down.
const MEDIA: [u16; 8] = [0xCD, 0xB5, 0xB6, 0xE9, 0xEA, 0xE2, 0x6F, 0x70];

/// The actions the keyboard's firmware can do that the inspector lists as they are, by
/// group: only those it has a behavior for (EDIT-2). Power to the lights and display isn't
/// offered: ZMK v0.3 doesn't describe its parameter, so it can't be assigned (tech spec
/// §8.1). Keys are named as `labels` names them.
pub fn choices(keymap: &Keymap, behaviors: &BehaviorTable, labels: &Labels) -> Vec<Choice> {
    let media = MEDIA.map(|id| {
        core::Action::KeyPress(hid::Shortcut {
            mods: Mods::NONE,
            usage: HidUsage {
                page: UsagePage::Consumer,
                id,
            },
        })
    });
    let bluetooth = (0..5)
        .map(|profile| core::Action::Bluetooth(BtCommand::Select(profile)))
        .chain(
            [
                BtCommand::Next,
                BtCommand::Previous,
                BtCommand::Clear,
                BtCommand::ClearAll,
            ]
            .map(core::Action::Bluetooth),
        )
        .chain(
            [OutCommand::Usb, OutCommand::Bluetooth, OutCommand::Toggle].map(core::Action::Output),
        );
    let special = [
        core::Action::CapsWord,
        core::Action::KeyRepeat,
        core::Action::Unlock,
        core::Action::Bootloader,
        core::Action::Reset,
    ];
    let groups = media
        .into_iter()
        .map(|action| (Category::Media, action))
        .chain(bluetooth.map(|action| (Category::Bluetooth, action)))
        .chain(LIGHTING.map(|(_, command)| (Category::Lighting, core::Action::Lighting(command))))
        .chain(special.map(|action| (Category::Special, action)));
    groups
        .filter(|(_, action)| action.to_binding(behaviors).is_ok())
        .map(|(category, action)| {
            let key = keycap::describe_action(keymap, behaviors, action, labels);
            Choice {
                category,
                label: match key.keycap {
                    keycap::Keycap::Label(text) => text,
                    _ => key.sentence.clone(),
                },
                sentence: key.sentence,
                action: action.into(),
                icon: None,
                default_shortcut: None,
            }
        })
        .collect()
}

/// The Mac actions to offer on a macOS version: those verified on it (MAC-4, tech spec
/// §8.2), with their shortcuts as `labels` names their keys. On another system, or a
/// version they weren't verified on, there are none.
#[must_use]
pub fn mac_choices(version: Option<presets::MacOsVersion>, labels: &Labels) -> Vec<Choice> {
    version
        .into_iter()
        .flat_map(presets::offered)
        .map(|preset| Choice {
            category: Category::Mac,
            label: preset.name.to_owned(),
            sentence: preset.effect.to_owned(),
            action: Action::KeyPress {
                key: preset.shortcut.into(),
            },
            icon: Some(preset.icon.to_owned()),
            default_shortcut: preset
                .uses_default_shortcut
                .then(|| keycap::shortcut_label(preset.shortcut, labels)),
        })
        .collect()
}

/// The Mac actions this Mac's macOS version offers, for the action picker. They don't
/// depend on the keyboard, so the UI asks once, and again in another input source.
#[tauri::command]
#[specta::specta]
#[must_use]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn mac_actions(app: tauri::State<'_, AppState>) -> Vec<Choice> {
    mac_choices(
        keywright_platform::system::macos_version().map(presets::MacOsVersion),
        &app.context().labels,
    )
}

/// A behavior Keywright doesn't know, which the inspector edits by its parameters
/// (EDIT-2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
pub struct OtherBehavior {
    /// Its ID on this keyboard.
    pub id: u32,
    /// The firmware's name for it.
    pub name: String,
    /// The combinations of parameters it accepts: a binding must match one.
    pub parameters: Vec<ParameterSet>,
}

/// Values a behavior's two parameters accept together.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
pub struct ParameterSet {
    /// What the first accepts: any one of them. None means it must be 0.
    pub param1: Vec<ParameterValue>,
    /// What the second accepts, as for the first.
    pub param2: Vec<ParameterValue>,
}

/// A value, or a kind of value, a parameter accepts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
pub struct ParameterValue {
    /// What the firmware calls it. It can be empty.
    pub name: String,
    /// The values it covers.
    pub value: Value,
}

/// The values a parameter value covers. In TypeScript, `{ kind: "range", min, max }` and so
/// on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Value {
    /// Only 0.
    Nil,
    /// Exactly this number.
    Constant {
        /// The number.
        value: u32,
    },
    /// Any number from `min` to `max`.
    Range {
        /// The smallest.
        min: i32,
        /// The largest.
        max: i32,
    },
    /// A key or shortcut.
    Key,
    /// A layer, by ID.
    Layer,
}

/// The behaviors Keywright doesn't know, with what their parameters accept.
pub fn other_behaviors(behaviors: &BehaviorTable) -> Vec<OtherBehavior> {
    behaviors
        .iter()
        .filter(|behavior| behaviors.kind(behavior.id).is_none())
        .map(|behavior| OtherBehavior {
            id: behavior.id.0,
            name: behavior.name.clone(),
            parameters: behavior
                .parameters
                .iter()
                .map(|set| {
                    let values = |values: &[keywright_core::behavior::ParameterValue]| {
                        values
                            .iter()
                            .map(|value| ParameterValue {
                                name: value.name.clone(),
                                value: match value.kind {
                                    ValueKind::Nil => Value::Nil,
                                    ValueKind::Constant(value) => Value::Constant { value },
                                    ValueKind::Range { min, max } => Value::Range { min, max },
                                    ValueKind::HidUsage { .. } => Value::Key,
                                    ValueKind::LayerId => Value::Layer,
                                },
                            })
                            .collect()
                    };
                    ParameterSet {
                        param1: values(&set.param1),
                        param2: values(&set.param2),
                    }
                })
                .collect(),
        })
        .collect()
}

/// A key the key picker offers (design §5.6), in its group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
pub struct KeyEntry {
    /// The key's usage page.
    pub page: Page,
    /// The key's usage ID.
    pub id: u16,
    /// What its keycap shows, such as `⌫`.
    pub cap: String,
    /// What text calls it, such as `Delete`.
    pub name: String,
    /// What it types with Shift, if that's another character, such as `!` for 1.
    pub shifted: Option<String>,
    /// What ZMK calls it alone, without modifiers: the name Keywright writes first, such as
    /// `BSPC`, then the others, such as `BACKSPACE`. The key picker's search finds keys by
    /// them.
    pub zmk: Vec<String>,
    /// Its group.
    pub group: Group,
}

/// A group of keys in the key picker's More tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum Group {
    /// A to Z.
    Letter,
    /// 1 to 0.
    Number,
    /// Punctuation.
    Symbol,
    /// Return, Escape, Delete, Tab, Space and the like.
    Editing,
    /// Arrows, Home, End, Page Up and Page Down.
    Navigation,
    /// F1 to F24.
    Function,
    /// The numeric keypad.
    Numpad,
    /// The keys of Japanese, Korean and other keyboards.
    International,
    /// Control, Shift, Option, Command and Caps Lock.
    Modifier,
    /// Playback, volume and brightness.
    Media,
    /// Keys that open an app or act in one.
    Application,
    /// Everything else.
    Other,
}

impl From<KeyCategory> for Group {
    fn from(category: KeyCategory) -> Self {
        match category {
            KeyCategory::Letter => Self::Letter,
            KeyCategory::Number => Self::Number,
            KeyCategory::Symbol => Self::Symbol,
            KeyCategory::Editing => Self::Editing,
            KeyCategory::Navigation => Self::Navigation,
            KeyCategory::Function => Self::Function,
            KeyCategory::Numpad => Self::Numpad,
            KeyCategory::International => Self::International,
            KeyCategory::Modifier => Self::Modifier,
            KeyCategory::Media => Self::Media,
            KeyCategory::Application => Self::Application,
            KeyCategory::Other => Self::Other,
        }
    }
}

/// Every key ZMK names, with its labels in the input source and its group, for the key
/// picker. The UI asks once, and again in another input source.
#[tauri::command]
#[specta::specta]
#[must_use]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn key_catalog(app: tauri::State<'_, AppState>) -> Vec<KeyEntry> {
    catalog(&app.context().labels)
}

/// Every key ZMK names, with its labels, its ZMK names and its group. Right-hand modifiers
/// are left out: the modifier toggles choose a side.
pub fn catalog(labels: &Labels) -> Vec<KeyEntry> {
    keys::named_usages()
        .filter(|usage| !(usage.page == UsagePage::Keyboard && (0xE4..=0xE7).contains(&usage.id)))
        .filter_map(|usage| {
            let (cap, name) = labels.cap(usage)?;
            Some(KeyEntry {
                page: usage.page.into(),
                id: usage.id,
                cap,
                name,
                shifted: labels.shifted(usage),
                zmk: zmk_names(usage),
                group: KeyCategory::of(usage).into(),
            })
        })
        .collect()
}

/// What ZMK calls a key alone: the name Keywright writes first, then the others in the
/// order `keys.h` defines them.
fn zmk_names(usage: HidUsage) -> Vec<String> {
    let alone = hid::Shortcut {
        mods: Mods::NONE,
        usage,
    };
    let written = keys::zmk::name(alone);
    written
        .into_iter()
        .chain(
            keys::zmk::names()
                .filter(|candidate| candidate.shortcut == alone && Some(candidate.name) != written)
                .map(|candidate| candidate.name),
        )
        .map(str::to_owned)
        .collect()
}

/// The kind of keyboard macOS takes the keyboard to be (tech spec §11.1), which the key
/// picker draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum KeyboardType {
    /// The US layout, with a wide Return key.
    Ansi,
    /// The European layout: a tall Return key, and an extra key right of left Shift.
    Iso,
    /// The Japanese layout, with extra keys of its own.
    Jis,
}

impl From<mac::KeyboardType> for KeyboardType {
    fn from(keyboard: mac::KeyboardType) -> Self {
        match keyboard {
            mac::KeyboardType::Ansi => Self::Ansi,
            mac::KeyboardType::Iso => Self::Iso,
            mac::KeyboardType::Jis => Self::Jis,
        }
    }
}

/// The kind of keyboard the key labels are for, for the key picker's Keyboard tab. Like
/// the catalog, it changes with the input source.
#[tauri::command]
#[specta::specta]
#[must_use]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn keyboard_type(app: tauri::State<'_, AppState>) -> KeyboardType {
    app.context().labels.keyboard().into()
}

/// A physical key, by its `KeyboardEvent.code`, for the shortcut recorder (MAC-2, tech
/// spec §11.2): what the key is, whatever the input source makes it type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
pub struct DomCode {
    /// The `KeyboardEvent.code` value, such as `KeyA` or `ShiftRight`.
    pub code: String,
    /// The key's usage page.
    pub page: Page,
    /// The key's usage ID.
    pub id: u16,
}

/// Every `KeyboardEvent.code` value the recorder knows, with the key it stands for.
#[tauri::command]
#[specta::specta]
#[must_use]
pub fn dom_codes() -> Vec<DomCode> {
    keys::named_usages()
        .chain((0xE4..=0xE7).map(|id| HidUsage {
            page: UsagePage::Keyboard,
            id,
        }))
        .filter_map(|usage| {
            Some(DomCode {
                code: keys::dom::code(usage)?.to_owned(),
                page: usage.page.into(),
                id: usage.id,
            })
        })
        .collect()
}

/// What a key or shortcut is called (design §5.3): on a keycap, in text, and the Mac
/// action it is, if it's one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
pub struct ShortcutWords {
    /// As a keycap shows it, such as `⇧⌘4`.
    pub label: String,
    /// As text calls it, such as `Escape`.
    pub name: String,
    /// The Mac action preset it is, such as `Screenshot of an area`, if it's one.
    pub preset: Option<String>,
}

/// What a key or shortcut is called, and the Mac action it is, for the recorder.
#[tauri::command]
#[specta::specta]
#[must_use]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn shortcut_words(app: tauri::State<'_, AppState>, shortcut: Shortcut) -> ShortcutWords {
    words(shortcut, &app.context().labels)
}

/// What a key or shortcut is called, as `labels` names keys, and the Mac action it is.
fn words(shortcut: Shortcut, labels: &Labels) -> ShortcutWords {
    let shortcut = hid::Shortcut::from(shortcut);
    ShortcutWords {
        label: keycap::shortcut_label(shortcut, labels),
        name: keycap::shortcut_name(shortcut, labels),
        preset: presets::PRESETS
            .iter()
            .find(|preset| preset.shortcut == shortcut)
            .map(|preset| preset.name.to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use keywright_core::action::{self as core, BtCommand, ExtPowerCommand, OutCommand};
    use keywright_core::behavior::BehaviorId;
    use keywright_core::hid::{self, HidUsage, Mods, UsagePage};
    use keywright_core::keymap::LayerId;
    use keywright_core::keys::source::{Labels, Typed};
    use keywright_core::mac;
    use keywright_device::{Connection, Peer, Session, State};
    use keywright_sim::Simulator;
    use serde_json::json;

    use super::{
        Action, Category, KeyboardType, LIGHTING, Page, Param, Shortcut, Value, catalog, dom_codes,
        mac_choices, words,
    };
    use crate::ipc::keymap::Keymap;
    use crate::ipc::{AppState, Context};
    use keywright_core::mac::presets::MacOsVersion;

    fn shortcut(mods: Mods, page: UsagePage, id: u16) -> hid::Shortcut {
        hid::Shortcut {
            mods,
            usage: HidUsage { page, id },
        }
    }

    /// The simulated Sofle, unlocked and ready, and the keymap the UI draws.
    async fn ready_sofle() -> (AppState, Keymap) {
        let session = Session::start();
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        session
            .connect(Connection::new(keyboard.connect(), Peer::Simulated))
            .await
            .unwrap();
        session
            .wait_for(|state| *state == State::Ready)
            .await
            .unwrap();
        let keymap = Keymap::of(&session.snapshot().await.unwrap(), &Context::default()).unwrap();
        (AppState::new(session), keymap)
    }

    #[test]
    fn every_kind_of_action_crosses_to_the_ui_and_back_unchanged() {
        let escape = shortcut(Mods::NONE, UsagePage::Keyboard, 0x29);
        let control = shortcut(Mods::RIGHT_SHIFT, UsagePage::Keyboard, 0xE0);
        let mute = shortcut(Mods::NONE, UsagePage::Consumer, 0xE2);
        let mut actions = vec![
            core::Action::KeyPress(escape),
            core::Action::KeyPress(mute),
            core::Action::ModTap {
                hold: control,
                tap: escape,
            },
            core::Action::LayerTap {
                layer: LayerId(2),
                tap: escape,
            },
            core::Action::Momentary(LayerId(1)),
            core::Action::ToLayer(LayerId(1)),
            core::Action::Toggle(LayerId(1)),
            core::Action::StickyLayer(LayerId(1)),
            core::Action::StickyKey(control),
            core::Action::Transparent,
            core::Action::None,
            core::Action::CapsWord,
            core::Action::KeyRepeat,
            core::Action::Unlock,
            core::Action::Bootloader,
            core::Action::Reset,
            core::Action::Other {
                behavior: BehaviorId(9),
                param1: 3,
                param2: 4,
            },
        ];
        actions.extend(
            [
                BtCommand::Clear,
                BtCommand::Next,
                BtCommand::Previous,
                BtCommand::Select(3),
                BtCommand::ClearAll,
                BtCommand::Disconnect(2),
            ]
            .map(core::Action::Bluetooth),
        );
        actions.extend(
            [OutCommand::Toggle, OutCommand::Usb, OutCommand::Bluetooth].map(core::Action::Output),
        );
        actions.extend(LIGHTING.map(|(_, command)| core::Action::Lighting(command)));
        actions.extend(
            [
                ExtPowerCommand::Toggle,
                ExtPowerCommand::On,
                ExtPowerCommand::Off,
            ]
            .map(core::Action::ExternalPower),
        );
        for action in actions {
            let ui = Action::from(action);
            let json = serde_json::to_value(ui).unwrap();
            let back: Action = serde_json::from_value(json).unwrap();
            assert_eq!(core::Action::from(back), action, "{action:?}");
        }
    }

    /// Key Toggle takes a key: the UI gets it as one, and gives it back as one.
    #[tokio::test]
    async fn an_unknown_behaviors_key_parameter_is_a_key() {
        let (app, keymap) = ready_sofle().await;
        let toggle = keymap
            .others
            .iter()
            .find(|other| other.name == "Key Toggle")
            .unwrap()
            .id;
        let a = Shortcut {
            mods: 0,
            page: Page::Keyboard,
            id: 0x04,
        };
        let action = Action::Other {
            behavior: toggle,
            param1: Param::Key { key: a },
            param2: Param::Number { value: 0 },
        };
        app.set_action(0, vec![13], action, false).await.unwrap();
        let keymap =
            Keymap::of(&app.session.snapshot().await.unwrap(), &Context::default()).unwrap();
        let key = &keymap.layers[0].keys[13];
        assert_eq!(key.action, action);
        assert_eq!(key.sentence, "Runs the firmware's Key Toggle action.");
    }

    #[test]
    fn the_recorder_knows_physical_keys_both_sides_modifiers_included() {
        let codes = dom_codes();
        let find = |code: &str| {
            codes
                .iter()
                .find(|each| each.code == code)
                .map(|each| (each.page, each.id))
        };
        assert_eq!(find("KeyA"), Some((Page::Keyboard, 0x04)));
        assert_eq!(find("Digit4"), Some((Page::Keyboard, 0x21)));
        assert_eq!(find("ShiftLeft"), Some((Page::Keyboard, 0xE1)));
        assert_eq!(find("AltRight"), Some((Page::Keyboard, 0xE6)));
        assert_eq!(find("Escape"), Some((Page::Keyboard, 0x29)));
        assert_eq!(find("Fn"), None);
    }

    #[test]
    fn a_recorded_shortcut_is_named_and_matched_to_a_mac_action() {
        let screenshot_area = words(
            Shortcut {
                mods: 0x0A,
                page: Page::Keyboard,
                id: 0x21,
            },
            &Labels::us(),
        );
        assert_eq!(screenshot_area.label, "⇧⌘4");
        assert_eq!(
            screenshot_area.preset.as_deref(),
            Some("Screenshot of an area")
        );
        let option = words(
            Shortcut {
                mods: 0,
                page: Page::Keyboard,
                id: 0xE2,
            },
            &Labels::us(),
        );
        assert_eq!(
            (option.label.as_str(), option.name.as_str()),
            ("⌥", "⌥ Option")
        );
        assert_eq!(option.preset, None);
    }

    #[tokio::test]
    async fn the_kinds_are_those_the_firmware_has() {
        let (_, keymap) = ready_sofle().await;
        assert_eq!(keymap.kinds.len(), 10);
    }

    #[test]
    fn actions_reach_the_ui_tagged_by_kind() {
        let escape = shortcut(Mods::LEFT_COMMAND, UsagePage::Keyboard, 0x29);
        assert_eq!(
            serde_json::to_value(Action::from(core::Action::KeyPress(escape))).unwrap(),
            json!({ "kind": "key-press", "key": { "mods": 8, "page": "keyboard", "id": 0x29 } })
        );
        assert_eq!(
            serde_json::to_value(Action::from(core::Action::Bluetooth(BtCommand::Select(1))))
                .unwrap(),
            json!({ "kind": "bluetooth", "command": "select", "profile": 1 })
        );
        assert_eq!(
            serde_json::to_value(Action::from(core::Action::None)).unwrap(),
            json!({ "kind": "nothing" })
        );
    }

    /// The Sofle's firmware has every standard behavior, so the inspector lists every
    /// choice but power to the lights and display, which can't be assigned.
    #[tokio::test]
    async fn the_choices_are_what_the_firmware_can_do_in_keycap_words() {
        let (_, keymap) = ready_sofle().await;
        let count = |category| {
            keymap
                .choices
                .iter()
                .filter(|choice| choice.category == category)
                .count()
        };
        assert_eq!(count(Category::Media), 8);
        assert_eq!(count(Category::Bluetooth), 12);
        assert_eq!(count(Category::Lighting), 13);
        assert_eq!(count(Category::Special), 5);
        let play = &keymap.choices[0];
        assert_eq!(
            (play.label.as_str(), play.sentence.as_str()),
            ("Play/Pause", "Presses Play/Pause.")
        );
        assert!(
            keymap
                .choices
                .iter()
                .all(|choice| !matches!(choice.action, Action::ExternalPower { .. }))
        );
    }

    /// On macOS 27, every preset S7 verified is offered, each with its icon, and those that
    /// rely on a default shortcut name it. On a version they weren't verified on, or
    /// another system, none is.
    #[test]
    fn mac_actions_are_offered_on_the_versions_they_were_verified_on() {
        let offered = mac_choices(Some(MacOsVersion(27)), &Labels::us());
        assert_eq!(offered.len(), 24);
        assert!(
            offered
                .iter()
                .all(|choice| choice.category == Category::Mac)
        );
        let find = |name: &str| offered.iter().find(|choice| choice.label == name).unwrap();
        let lock = find("Lock Screen");
        assert_eq!(
            (lock.icon.as_deref(), lock.default_shortcut.as_deref()),
            (Some("lock"), Some("⌃⌘Q"))
        );
        assert_eq!(
            lock.action,
            Action::KeyPress {
                key: Shortcut {
                    mods: 0x09,
                    page: Page::Keyboard,
                    id: 0x14
                }
            }
        );
        let mission_control = find("Mission Control");
        assert_eq!(mission_control.default_shortcut, None);
        assert_eq!(
            mission_control.action,
            Action::KeyPress {
                key: Shortcut {
                    mods: 0,
                    page: Page::Consumer,
                    id: 0x29F
                }
            }
        );
        assert_eq!(find("Copy").default_shortcut, None);
        assert_eq!(mac_choices(Some(MacOsVersion(14)), &Labels::us()), vec![]);
        assert_eq!(mac_choices(None, &Labels::us()), vec![]);
    }

    /// MAC-1: the words the UI shows for keys and actions say Command, Option, Control and
    /// Shift, never GUI, Win, Meta, Super or Alt, and modifiers go in the order ⌃⌥⇧⌘.
    #[tokio::test]
    async fn the_words_for_keys_and_actions_are_the_macs() {
        let (_, keymap) = ready_sofle().await;
        // The catalog's ZMK names are for the key picker's search, and show only with Show
        // ZMK names on (EDIT-6).
        let catalog: Vec<_> = catalog(&Labels::us())
            .into_iter()
            .map(|key| (key.cap, key.name, key.shifted))
            .collect();
        let all_mods = words(
            Shortcut {
                mods: 0x0F,
                page: Page::Keyboard,
                id: 0x04,
            },
            &Labels::us(),
        );
        assert_eq!(all_mods.label, "⌃⌥⇧⌘A");
        let words = serde_json::to_string(&(
            &keymap,
            &catalog,
            mac_choices(Some(MacOsVersion(27)), &Labels::us()),
            all_mods,
        ))
        .unwrap();
        for word in words.split(|c: char| !c.is_alphanumeric()) {
            assert!(
                !["GUI", "Win", "Meta", "Super", "Alt"].contains(&word),
                "{word} is shown"
            );
        }
    }

    /// The Sofle's firmware has three behaviors Keywright doesn't know: Grave/Escape, Key
    /// Toggle and the knobs' key press.
    #[tokio::test]
    async fn behaviors_keywright_doesnt_know_come_with_their_parameters() {
        let (_, keymap) = ready_sofle().await;
        let names: Vec<_> = keymap
            .others
            .iter()
            .map(|other| other.name.as_str())
            .collect();
        assert!(names.contains(&"Grave/Escape"), "{names:?}");
        let toggle = keymap
            .others
            .iter()
            .find(|other| other.name == "Key Toggle")
            .unwrap();
        assert_eq!(toggle.parameters.len(), 1);
        assert_eq!(toggle.parameters[0].param1[0].value, Value::Key);
        assert_eq!(toggle.parameters[0].param2, []);
    }

    #[test]
    fn the_key_catalog_has_every_key_zmk_names_by_group_but_right_modifiers() {
        let catalog = catalog(&Labels::us());
        let find = |page, id| catalog.iter().find(|key| key.page == page && key.id == id);
        let delete = find(Page::Keyboard, 0x2A).unwrap();
        assert_eq!((delete.cap.as_str(), delete.name.as_str()), ("⌫", "Delete"));
        assert!(find(Page::Keyboard, 0xE0).is_some(), "left Control");
        assert!(find(Page::Keyboard, 0xE4).is_none(), "right Control");
        assert_eq!(find(Page::Consumer, 0xE9).unwrap().name, "Volume Up");
        assert!(catalog.len() > 300);
    }

    /// The key picker shows Shift's character on a keycap, and its search finds keys by
    /// their ZMK names, the one Keywright writes first.
    #[test]
    fn the_key_catalog_has_shift_characters_and_zmk_names() {
        let catalog = catalog(&Labels::us());
        let find = |page, id| catalog.iter().find(|key| key.page == page && key.id == id);
        let one = find(Page::Keyboard, 0x1E).unwrap();
        assert_eq!(one.shifted.as_deref(), Some("!"));
        assert_eq!(one.zmk, ["N1", "NUMBER_1", "NUM_1"]);
        let delete = find(Page::Keyboard, 0x2A).unwrap();
        assert_eq!(delete.shifted, None);
        assert_eq!(delete.zmk.first().map(String::as_str), Some("BSPC"));
        assert!(delete.zmk.iter().any(|name| name == "BACKSPACE"));
        assert_eq!(
            find(Page::Consumer, 0xE9)
                .unwrap()
                .zmk
                .first()
                .map(String::as_str),
            Some("C_VOL_UP")
        );
    }

    #[test]
    fn the_keyboard_type_is_the_labels_one() {
        assert_eq!(
            KeyboardType::from(Labels::us().keyboard()),
            KeyboardType::Ansi
        );
        let iso = Labels::from_typed(mac::KeyboardType::Iso, |_| Typed::default());
        assert_eq!(KeyboardType::from(iso.keyboard()), KeyboardType::Iso);
    }

    /// The inspector sets any action, on several keys at once, as one change.
    #[tokio::test]
    async fn an_action_from_the_inspector_is_set_and_undone_in_one_step() {
        let (app, _) = ready_sofle().await;
        let escape_control = Action::from(core::Action::ModTap {
            hold: shortcut(Mods::NONE, UsagePage::Keyboard, 0xE0),
            tap: shortcut(Mods::NONE, UsagePage::Keyboard, 0x29),
        });
        app.set_action(0, vec![13, 14], escape_control, false)
            .await
            .unwrap();
        let keymap =
            Keymap::of(&app.session.snapshot().await.unwrap(), &Context::default()).unwrap();
        for position in [13, 14] {
            let key = &keymap.layers[0].keys[position];
            assert_eq!(key.action, escape_control);
            assert_eq!(key.sentence, "Tap for Escape, hold for ⌃ Control.");
        }
        app.session.undo().await.unwrap();
        let keymap =
            Keymap::of(&app.session.snapshot().await.unwrap(), &Context::default()).unwrap();
        assert_eq!(keymap.layers[0].keys[13].sentence, "Types Q.");
        assert_eq!(keymap.layers[0].keys[14].sentence, "Types W.");
    }
}
