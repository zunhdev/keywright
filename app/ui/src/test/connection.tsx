/** The connection screens for their tests: a fake Rust side and the Sofle in each state. */
import { mockIPC } from "@tauri-apps/api/mocks";
import { act, render } from "@testing-library/react";
import { I18nProvider } from "react-aria-components";

import { useStore } from "../app/store.ts";
import { followConnection } from "../features/connection/follow.ts";
import { UnlockSheet } from "../features/connection/UnlockSheet.tsx";
import { KeyboardCanvas } from "../features/keyboard-canvas/KeyboardCanvas.tsx";
import { LOCALE } from "../i18n/messages.ts";
import type { BluetoothPermission, SessionState, SessionStatus } from "../ipc/bindings.ts";
import { sofleGuide } from "./sofle.ts";

/**
 * Fakes the Rust side: answers `keyboard_guide` with the Sofle's, macOS allows Bluetooth
 * when asked, and records commands.
 */
function rustSide(): string[] {
  const calls: string[] = [];
  mockIPC((command, args) => {
    if (command === "keyboard_guide") {
      const pack = args !== undefined && "pack" in args ? args["pack"] : null;
      return pack === null || pack === "keebart-sofle-choc-pro-bt" ? sofleGuide : null;
    }
    calls.push(command);
    return command === "allow_bluetooth" ? "allowed" : null;
  });
  return calls;
}

/** Sets what macOS says about Keywright using Bluetooth. */
export function permit(permission: BluetoothPermission) {
  act(() => {
    useStore.setState({ bluetoothPermission: permission });
  });
}

/** The Sofle over USB, in `state`. */
export function sofle(
  state: SessionState,
  changes = 0,
  pack: string | null = "keebart-sofle-choc-pro-bt",
): SessionStatus {
  return {
    state,
    keyboard: "Sofle Choc BT",
    pack,
    transport: "usb",
    changes,
    canUndo: false,
    canRedo: false,
    undo: null,
    redo: null,
  };
}

/** Moves the session to `status`, as the Rust side reports it. */
export function report(status: SessionStatus | null) {
  act(() => {
    useStore.setState({ session: status });
  });
}

/** Draws the canvas and the unlock sheet, following the session as the window does. */
export function drawWindow(status: SessionStatus | null) {
  const calls = rustSide();
  const stop = followConnection();
  useStore.setState({ session: status });
  const view = render(
    <I18nProvider locale={LOCALE}>
      <KeyboardCanvas />
      <UnlockSheet />
    </I18nProvider>,
  );
  return { calls, stop, ...view };
}
