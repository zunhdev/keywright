import { describe, expect, test } from "bun:test";

import { emit } from "@tauri-apps/api/event";
import { mockIPC } from "@tauri-apps/api/mocks";
import { act, render, screen, waitFor, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { I18nProvider } from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { toasts } from "../../design-system/Toast.tsx";
import { LOCALE } from "../../i18n/messages.ts";
import type { BackupEntry, ImportPreview } from "../../ipc/bindings.ts";
import { expectNoAxeViolations } from "../../test/axe.ts";
import { status } from "../../test/canvas.tsx";
import { exportKeymap, importBackup } from "./actions.ts";
import { BackupsWindow } from "./BackupsWindow.tsx";
import { followImport, ImportSheet } from "./ImportSheet.tsx";

/** Two automatic backups, newest first. */
const BACKUPS: BackupEntry[] = [
  {
    id: "20261025-123645-before-firmware.keywright",
    created: "2026-10-25T12:36:45Z",
    reason: "before-firmware",
    keyboard: "Sofle Choc BT",
  },
  {
    id: "20261024-090000-first-connect.keywright",
    created: "2026-10-24T09:00:00Z",
    reason: "first-connect",
    keyboard: "Sofle Choc BT",
  },
];

/** What the newest backup would change: a key and a layer's name, and two keys it can't. */
const PREVIEW: ImportPreview = {
  file: BACKUPS[0]!.id,
  created: BACKUPS[0]!.created,
  reason: "before-firmware",
  keyboard: "Sofle Choc BT",
  rows: [
    {
      key: { kind: "key", layer: 0, position: 13 },
      layer: 0,
      layerName: "default",
      place: "Left half, row 2, key 2",
      before: "⎋ Esc",
      after: "Q",
      sentence: "Types Q.",
      spoken: { before: "Types Escape.", after: "Types Q." },
    },
    {
      key: { kind: "renamed", layer: 1 },
      layer: 1,
      layerName: "Numbers",
      place: null,
      before: "Numbers",
      after: "lower",
      sentence: null,
      spoken: null,
    },
  ],
  keys: 1,
  cannotApply: [
    {
      kind: "missing-action",
      layer: "default",
      place: "Left half, row 1, key 2",
      action: "Mouse Move",
    },
    { kind: "unreadable", layer: "default", place: "Left half, row 1, key 3" },
  ],
};

/**
 * Fakes the Rust side, with `backups` and their previews, or the keymap not loaded if
 * `ready` is false, and records each command with its arguments.
 */
function rustSide(backups: BackupEntry[] = BACKUPS, ready = true): [string, unknown][] {
  const calls: [string, unknown][] = [];
  mockIPC(
    (command, args) => {
      calls.push([command, args]);
      switch (command) {
        case "backups":
          return backups;
        case "backup_preview":
        case "import_preview":
          if (!ready) {
            // Tauri rejects a command with its error's payload, which isn't an Error.
            // oxlint-disable-next-line no-throw-literal, typescript/only-throw-error
            throw { code: "not-ready" };
          }
          return PREVIEW;
        default:
          return null;
      }
    },
    { shouldMockEvents: true },
  );
  return calls;
}

function drawWindow() {
  return render(
    <I18nProvider locale={LOCALE}>
      <BackupsWindow />
    </I18nProvider>,
  );
}

describe("the Backups window", () => {
  test("lists the backups and previews the newest against the keymap", async () => {
    const calls = rustSide();
    const { container } = drawWindow();
    const list = await screen.findByRole("grid", { name: "Backups" });
    const rows = within(list).getAllByRole("row");
    expect(rows).toHaveLength(2);
    expect(rows[0]!.textContent).toContain("Automatic: before firmware updateSofle Choc BT");
    expect(rows[1]!.textContent).toContain("Automatic: first connectSofle Choc BT");
    expect(rows[0]!.getAttribute("aria-selected")).toBe("true");

    expect(await screen.findByText("1 key will change; 2 things can't be applied.")).toBeDefined();
    expect(screen.getByText("Left half, row 2, key 2")).toBeDefined();
    expect(screen.getByText("Can't apply")).toBeDefined();
    expect(screen.getByText("“Mouse Move” isn't in this keyboard's firmware.")).toBeDefined();
    expect(
      screen.getByText("Keywright can't read what this key does in the backup."),
    ).toBeDefined();
    expect(calls).toContainEqual(["backup_preview", { id: BACKUPS[0]!.id }]);
    await expectNoAxeViolations(container);

    await userEvent.click(rows[1]!);
    await waitFor(() => {
      expect(calls).toContainEqual(["backup_preview", { id: BACKUPS[1]!.id }]);
    });
    await userEvent.click(screen.getByRole("button", { name: "Show in Finder" }));
    expect(calls).toContainEqual(["show_backup_in_finder", { id: BACKUPS[1]!.id }]);
    await userEvent.click(screen.getByRole("button", { name: "Restore…" }));
    expect(calls).toContainEqual(["restore_backup", { id: BACKUPS[1]!.id }]);
  });

  test("says how backups come about while there are none", async () => {
    rustSide([]);
    drawWindow();
    expect(await screen.findByText(/No backups yet/u)).toBeDefined();
    expect(screen.getByRole("button", { name: "Restore…" }).hasAttribute("disabled")).toBe(true);
  });

  test("asks for the keyboard before comparing", async () => {
    rustSide(BACKUPS, false);
    drawWindow();
    expect(await screen.findByText(/Connect and unlock your keyboard/u)).toBeDefined();
    expect(screen.getByRole("button", { name: "Restore…" }).hasAttribute("disabled")).toBe(true);
    expect(screen.queryByRole("alert")).toBeNull();
  });
});

/** Draws the sheet, with the Sofle ready, following the shell's requests. */
async function drawSheet() {
  useStore.setState({ session: { ...status("ready"), changes: 0 }, importing: false });
  const stop = await followImport();
  render(
    <I18nProvider locale={LOCALE}>
      <ImportSheet />
    </I18nProvider>,
  );
  return stop;
}

describe("the Import preview", () => {
  test("opens when the shell asks, and applies the backup as changes", async () => {
    const calls = rustSide();
    const stop = await drawSheet();
    await act(async () => {
      await emit("import://requested", null);
    });
    const sheet = await screen.findByRole("dialog", { name: "Import a backup" });
    expect(await within(sheet).findByText(/^Sofle Choc BT, /u)).toBeDefined();
    expect(within(sheet).getByText("1 key will change; 2 things can't be applied.")).toBeDefined();
    expect(within(sheet).getByText("lower")).toBeDefined();
    await expectNoAxeViolations(sheet);

    await userEvent.click(within(sheet).getByRole("button", { name: "Apply as Changes" }));
    expect(calls).toContainEqual(["apply_import", { force: false }]);
    await waitFor(() => {
      expect(screen.queryByRole("dialog")).toBeNull();
    });
    stop();
  });

  test("a backup asked for while the sheet is open replaces the one shown", async () => {
    const calls = rustSide();
    const stop = await drawSheet();
    await act(async () => {
      await emit("import://requested", null);
    });
    await screen.findByRole("dialog", { name: "Import a backup" });
    const asked = () => calls.filter(([command]) => command === "import_preview").length;
    await waitFor(() => {
      expect(asked()).toBe(1);
    });
    await act(async () => {
      await emit("import://requested", null);
    });
    await waitFor(() => {
      expect(asked()).toBe(2);
    });
    stop();
  });

  test("Cancel forgets the backup", async () => {
    const calls = rustSide();
    const stop = await drawSheet();
    act(() => {
      useStore.setState({ importing: true });
    });
    const sheet = await screen.findByRole("dialog", { name: "Import a backup" });
    await userEvent.click(within(sheet).getByRole("button", { name: "Cancel" }));
    expect(calls.map(([command]) => command)).toContain("cancel_import");
    expect(calls.map(([command]) => command)).not.toContain("apply_import");
    expect(useStore.getState().importing).toBe(false);
    stop();
  });
});

describe("File › Import… and Export…", () => {
  test("export says where the file went, and nothing if cancelled", async () => {
    let saved: string | null = "sofle-choc-bt-20261025-140300.keywright";
    mockIPC((command) => (command === "export_keymap" ? saved : null));
    await exportKeymap();
    expect(toasts.visibleToasts.map((toast) => toast.content.title)).toEqual([
      "Exported your keymap to sofle-choc-bt-20261025-140300.keywright.",
    ]);
    toasts.clear();
    saved = null;
    await exportKeymap();
    expect(toasts.visibleToasts).toEqual([]);
  });

  test("import asks the shell, which opens the preview", async () => {
    const calls = rustSide();
    await importBackup();
    expect(calls.map(([command]) => command)).toContain("import_file");
  });

  test("a backup opened before the window listened shows once it does", async () => {
    mockIPC((command) => command === "import_pending", { shouldMockEvents: true });
    const stop = await followImport();
    expect(useStore.getState().importing).toBe(true);
    stop();
  });
});
