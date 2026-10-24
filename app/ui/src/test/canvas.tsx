/**
 * Drawing the keyboard canvas in tests: the simulated Sofle's stock keymap, with the
 * session in a state, and its keys.
 */
import { render, screen } from "@testing-library/react";
import { I18nProvider } from "react-aria-components";

import { useStore } from "../app/store.ts";
import { KeyboardCanvas } from "../features/keyboard-canvas/KeyboardCanvas.tsx";
import { LOCALE } from "../i18n/messages.ts";
import type { SessionStatus } from "../ipc/bindings.ts";
import { sofleKeymap } from "./sofle-keymap.ts";

/** The Sofle's status over USB, in `kind`, with nothing to undo. */
export function status(kind: "ready" | "locked" | "no-device"): SessionStatus {
  return {
    state: { kind },
    keyboard: "Sofle Choc BT",
    pack: "keebart-sofle-choc-pro-bt",
    transport: "usb",
    changes: 0,
    canUndo: false,
    canRedo: false,
  };
}

/** Draws the canvas, with the session in `kind` and the Sofle's stock keymap read. */
export function drawSofle(kind: "ready" | "locked" | "no-device" = "ready") {
  useStore.setState({ session: status(kind), keymap: sofleKeymap });
  return render(
    <I18nProvider locale={LOCALE}>
      <KeyboardCanvas />
    </I18nProvider>,
  );
}

/** The keys drawn, by key position. */
export function keys(): HTMLElement[] {
  return screen.getAllByRole("button", { hidden: true });
}

/** The key at `position`, which must be there. */
export function keyAt(position: number): HTMLElement {
  const found = keys()[position];
  if (found === undefined) throw new Error(`no key at ${position}`);
  return found;
}
