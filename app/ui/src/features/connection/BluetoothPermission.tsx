import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { allowBluetooth, openBluetoothSettings } from "../../ipc/bluetooth.ts";

/** Whether Keywright may not use Bluetooth, so the permission has something to say. */
export const useBluetoothBarred = () =>
  useStore(
    (store) => store.bluetoothPermission !== null && store.bluetoothPermission !== "allowed",
  );

/**
 * What Keywright may do with Bluetooth, and how to change it (CON-6, design §8): asked for
 * in context, after one line saying why; if refused, how to allow it later. Nothing while
 * it's allowed.
 */
export function BluetoothPermission() {
  const messages = useMessages();
  const permission = useStore((store) => store.bluetoothPermission);
  if (permission === "not-asked") {
    return (
      <>
        <p>{messages.format("bluetooth.why")}</p>
        <Button size="compact" onPress={() => void allowBluetooth()}>
          {messages.format("bluetooth.allow")}
        </Button>
      </>
    );
  }
  if (permission === "denied") {
    return (
      <>
        <p>{messages.format("bluetooth.denied")}</p>
        <Button size="compact" onPress={() => void openBluetoothSettings()}>
          {messages.format("bluetooth.open-settings")}
        </Button>
      </>
    );
  }
  if (permission === "restricted") {
    return <p>{messages.format("bluetooth.restricted")}</p>;
  }
  return null;
}
