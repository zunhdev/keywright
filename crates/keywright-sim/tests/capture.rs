//! The simulated Sofle against a session captured from the real keyboard (spike S1,
//! KEY-8): the behaviors it reports, its physical layout and its stock keymap.

#![cfg(test)]

use std::fs;
use std::path::Path;

use keywright_sim::firmware::zmk_v0_3_behaviors;
use keywright_sim::{Firmware, Keyboard};
use keywright_studio::frame::FrameDecoder;
use keywright_studio::message::{
    BehaviorDetails, BehaviorsResponse, DeviceMessage, Keymap, KeymapResponse, PhysicalLayouts,
    Response,
};

const CAPTURE: &str = "sofle-usb-unlock-reset-and-edit.txt";

/// Every response in the capture, in order.
fn responses() -> Vec<Response> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../keywright-studio/fixtures")
        .join(CAPTURE);
    let text = fs::read_to_string(&path).unwrap();
    let mut decoder = FrameDecoder::new();
    let mut responses = Vec::new();
    for line in text.lines() {
        let Some(hex) = line.split('#').next().unwrap().trim().strip_prefix('<') else {
            continue;
        };
        let bytes: Vec<u8> = hex
            .split_whitespace()
            .map(|byte| u8::from_str_radix(byte, 16).unwrap())
            .collect();
        for payload in decoder.decode(&bytes) {
            if let DeviceMessage::Response { response, .. } =
                DeviceMessage::decode(&payload.unwrap()).unwrap()
            {
                responses.push(response);
            }
        }
    }
    responses
}

#[test]
fn the_behaviors_are_the_keyboards() {
    let captured: Vec<BehaviorDetails> = responses()
        .into_iter()
        .filter_map(|response| match response {
            Response::Behaviors(BehaviorsResponse::GetBehaviorDetails(details)) => Some(details),
            _ => None,
        })
        .collect();
    let simulated: Vec<BehaviorDetails> = zmk_v0_3_behaviors()
        .into_iter()
        .map(|behavior| behavior.details)
        .collect();
    assert_eq!(simulated, captured);
}

#[test]
fn the_physical_layout_is_the_keyboards() {
    let captured = responses()
        .into_iter()
        .find_map(|response| match response {
            Response::Keymap(KeymapResponse::GetPhysicalLayouts(layouts)) => Some(layouts),
            _ => None,
        })
        .unwrap();
    assert_eq!(
        captured,
        PhysicalLayouts {
            active_layout_index: 0,
            layouts: vec![Firmware::sofle().unwrap().layout],
        },
    );
}

#[test]
fn the_keymap_is_the_keyboards() {
    let captured: Keymap = responses()
        .into_iter()
        .find_map(|response| match response {
            Response::Keymap(KeymapResponse::GetKeymap(keymap)) => Some(keymap),
            _ => None,
        })
        .unwrap();
    let firmware = Firmware::sofle().unwrap();
    let name = |id| {
        firmware
            .behaviors
            .iter()
            .find(|behavior| behavior.details.id == id)
            .map_or("?", |behavior| behavior.details.display_name.as_str())
    };
    let mut keyboard = Keyboard::new(firmware.clone());
    keyboard.unlock();
    let simulated = keyboard.keymap();

    let mut differences = Vec::new();
    for (layer, expected) in simulated.layers.iter().zip(&captured.layers) {
        if (layer.id, &layer.name) != (expected.id, &expected.name) {
            differences.push(format!(
                "layer {} {:?}, keyboard {} {:?}",
                layer.id.0, layer.name, expected.id.0, expected.name
            ));
        }
        for (position, (binding, expected)) in
            layer.bindings.iter().zip(&expected.bindings).enumerate()
        {
            if binding != expected {
                differences.push(format!(
                    "layer {}, key {position}: {} {:#010x} {:#x}, keyboard {} {:#010x} {:#x}",
                    layer.id.0,
                    name(binding.behavior_id),
                    binding.param1,
                    binding.param2,
                    name(expected.behavior_id),
                    expected.param1,
                    expected.param2,
                ));
            }
        }
    }
    assert!(differences.is_empty(), "{}", differences.join("\n"));
    assert_eq!(simulated, captured);
}
