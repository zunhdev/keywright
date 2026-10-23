# Spike S4: Telling the right half apart on USB

Spike S4 (tech spec §15, [KEY-11](https://linear.app/zunhdev/issue/KEY-11)) asked:

- whether the Sofle's right half, plugged in alone, shows up as a serial port with ZMK's IDs;
- whether it answers Studio;
- how Keywright can tell, reliably, that the cable is in the wrong half.

It does show up, never answers, and can be told apart without asking it anything: it has **no keyboard interface**, because the right half doesn't type over USB. The product string that the session relied on, `Generic CDC`, is real, but the serial port library never sees it.

## Setup

- **Keyboard:** the Keebart Sofle Choc Pro BT from the earlier spikes, paired with the Mac over Bluetooth.
- **Mac:** MacBook Pro (M4 Max), macOS 27.2.
- **Date:** 2026-10-24.
- **Probes:** a throwaway Rust program outside the repository listed every USB device with ZMK's IDs through `nusb` 0.2.8, with its product string, serial number and interfaces, and every serial port through `serialport` 4.10.1. The S5 harness then asked each port, and Bluetooth, for Studio's device info.

## What each half shows

With the right half plugged in alone, the left half on its battery and on Bluetooth, and then with both plugged in:

|                                     | Left half                                     | Right half                     |
| ----------------------------------- | --------------------------------------------- | ------------------------------ |
| USB IDs                             | `1D50:615E`                                   | `1D50:615E`                    |
| USB product                         | `Sofle Choc Pro`                              | `Generic CDC`                  |
| Manufacturer                        | `ZMK Project`                                 | `ZMK Project`                  |
| USB serial number                   | `9CAD9FB5AF677D89`                            | `8BC278624254AB1C`             |
| Interfaces                          | serial (CDC ACM and data), **keyboard (HID)** | serial (CDC ACM and data) only |
| Port                                | `/dev/cu.usbmodem101`                         | `/dev/cu.usbmodem2101`         |
| Studio                              | answers                                       | **never answers**              |
| Product, as `serialport` reports it | `Sofle Choc Pro`                              | **none**                       |

- **Alone, the right half answered nothing** within the 1 s probe, while the left half answered over Bluetooth. With both plugged in, the left half answered over USB, and Bluetooth went quiet, since the keyboard types over USB then (spike S5).
- **Why `serialport` misses the product.** It reads the product string from the USB _interface_ that holds the serial port (`IOUSBHostInterface`), not from the USB _device_. The left half's interface carries one; the right half's doesn't. So the session's rule, "silent with product `Generic CDC`", has never matched: the CLI shows the right half as a keyboard without live editing (KEY-35).
- **The serial number** doesn't link the halves: the right half's is its own, and only the left half's matches what Studio reports.

## The rule

A ZMK serial port whose USB device has **no keyboard (HID) interface** belongs to a half that doesn't talk to the computer. The half that does types over the same cable, so it always has one. This holds whatever the board calls itself, and needs no request, so selection can skip such a port instead of waiting a second for it.

With a keyboard interface, a silent port means the keyboard is typing over the other connection (spike S5): Bluetooth answers, or else there's no live editing.

| USB port                    | Bluetooth            | Keywright                                                                                 |
| --------------------------- | -------------------- | ----------------------------------------------------------------------------------------- |
| no keyboard interface       | the keyboard answers | edits over Bluetooth, and says the cable is in the half that doesn't talk to the computer |
| no keyboard interface       | nothing answers      | **WrongHalf**: plug the cable into the other half                                         |
| keyboard interface, answers | —                    | edits over USB                                                                            |
| keyboard interface, silent  | the keyboard answers | edits over Bluetooth: "typing over Bluetooth" (design §7)                                 |
| keyboard interface, silent  | nothing answers      | **NoLiveEditing**                                                                         |

The support pack's `usb-half` hint names the half to plug in, so the Sofle's message can say "left" and "right"; a keyboard without a pack gets "the other half".

## Decisions

- **The interfaces, not the product string**, tell the halves apart. Reading them needs the USB device, which `serialport` doesn't expose; `nusb` (Apache-2.0 OR MIT) does, and pairs with the serial port by USB serial number. [KEY-99](https://linear.app/zunhdev/issue/KEY-99) makes `keywright-transport` and the session use the rule.
- **Design §7's card stands** for the case where nothing answers: "This looks like the right half. Keywright talks to the left half; plug the cable in there." When the left half answers over Bluetooth, Keywright edits there, and says so instead of blocking.
