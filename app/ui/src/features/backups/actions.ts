/**
 * File › Import… and Export… (BAK-1, BAK-3): the shell shows the open and save panels.
 * Importing opens the Import preview; exporting says where the file went. What went wrong
 * shows as a toast.
 */
import { toasts } from "../../design-system/Toast.tsx";
import { messages } from "../../i18n/messages.ts";
import { commands } from "../../ipc/bindings.ts";
import { run } from "../keyboard-canvas/edits.ts";

/** How long the "exported" toast stays, in ms. */
const TOAST = 5000;

/** Shows the open panel, then the Import preview for the backup chosen. */
export async function importBackup(): Promise<void> {
  await run(commands.importFile());
}

/** Shows the save panel, writes the keymap where it says, and says so. */
export async function exportKeymap(): Promise<void> {
  const answer = await commands.exportKeymap();
  const strings = messages();
  if (answer.status === "error") {
    toasts.add({ title: strings.format(`error.${answer.error.code}`) }, { timeout: TOAST });
  } else if (answer.data !== null) {
    toasts.add({ title: strings.format("export.done", { file: answer.data }) }, { timeout: TOAST });
  }
}
