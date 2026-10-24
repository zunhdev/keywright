import { useStore } from "../app/store.ts";
import { toasts } from "../design-system/Toast.tsx";
import { messages } from "../i18n/messages.ts";
import { type BluetoothPermission, commands } from "./bindings.ts";

/** How often to look again while macOS asks the person, in ms. */
const LOOK_EVERY = 1000;

/** How long to keep looking while macOS asks, in ms: the person may take a while. */
const ASKING_FOR = 120_000;

/** Reads what macOS says about Keywright using Bluetooth, into the store. It asks nobody. */
async function readPermission(): Promise<void> {
  useStore.setState({ bluetoothPermission: await commands.bluetoothPermission() });
}

/**
 * Keeps the store's `bluetoothPermission` up to date (CON-6): reads it now, and each time
 * the window shows, since the person may have changed it in System Settings meanwhile.
 * Returns a function that stops following.
 */
export async function followBluetoothPermission(): Promise<() => void> {
  await readPermission();
  const onShow = () => {
    if (document.visibilityState === "visible") void readPermission();
  };
  document.addEventListener("visibilitychange", onShow);
  return () => {
    document.removeEventListener("visibilitychange", onShow);
  };
}

/**
 * Uses Bluetooth, so macOS asks whether Keywright may, then follows the answer until it
 * comes. Only for when the person chooses Bluetooth, after the UI says why (CON-6).
 */
export async function allowBluetooth(): Promise<void> {
  const permission = await answer(await commands.allowBluetooth(), Date.now() + ASKING_FOR);
  useStore.setState({ bluetoothPermission: permission });
}

/** The permission once the person has answered, looking again each second until `until`. */
async function answer(
  permission: BluetoothPermission,
  until: number,
): Promise<BluetoothPermission> {
  if (permission !== "not-asked" || Date.now() >= until) return permission;
  await new Promise((resolve) => {
    setTimeout(resolve, LOOK_EVERY);
  });
  return answer(await commands.bluetoothPermission(), until);
}

/** Opens System Settings where the person can let Keywright use Bluetooth. */
export async function openBluetoothSettings(): Promise<void> {
  const result = await commands.openBluetoothSettings();
  if (result.status === "error") {
    toasts.add({ title: messages().format(`error.${result.error.code}`) });
  }
}
