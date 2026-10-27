import { useEffect, useState } from "react";
import type { Key } from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Select, SelectItem } from "../../design-system/Select.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { commands, type SecondaryLabels } from "../../ipc/bindings.ts";
import { followBluetoothPermission, openBluetoothSettings } from "../../ipc/bluetooth.ts";
import { BluetoothPermission } from "../connection/BluetoothPermission.tsx";
import { useSettings } from "./use-settings.ts";

import styles from "./Settings.module.css";

/** What the secondary labels' choice calls "no input source". */
const NONE = "none";

/** Settings › Keyboard's secondary labels (MAC-6). */
function SecondaryLabelsSetting() {
  const messages = useMessages();
  const [secondary, setSecondary] = useSettings(commands.secondaryLabels);
  const [failed, setFailed] = useState(false);
  if (secondary === null) return null;
  const choose = async (key: Key | null) => {
    const id = key === null || key === NONE ? null : String(key);
    const result = await commands.setSecondaryLabels(id);
    setFailed(result.status === "error");
    setSecondary({ ...secondary, chosen: id } satisfies SecondaryLabels);
  };
  const options = [
    { id: NONE, name: messages.format("settings.secondary-none") },
    ...secondary.layouts,
  ];
  return (
    <section className={styles.setting}>
      <Select
        label={messages.format("settings.secondary")}
        items={options}
        value={secondary.chosen ?? NONE}
        onChange={(key) => void choose(key)}
      >
        {(option) => <SelectItem id={option.id}>{option.name}</SelectItem>}
      </Select>
      <p className={styles.note}>{messages.format("settings.secondary-note")}</p>
      {failed ? (
        <p className={styles.problem}>{messages.format("error.setting-not-changed")}</p>
      ) : null}
    </section>
  );
}

/**
 * Settings › Keyboard's Bluetooth (CON-6, design §9.4): whether Keywright may use it, read
 * again each time the window shows, with Allow Bluetooth… before macOS has asked, and Open
 * System Settings once it has.
 */
function BluetoothSetting() {
  const messages = useMessages();
  const permission = useStore((store) => store.bluetoothPermission);
  useEffect(() => {
    const following = followBluetoothPermission();
    return () => {
      void following.then((stop) => stop());
    };
  }, []);
  if (permission === null) return null;
  return (
    <section className={styles.setting} aria-label={messages.format("settings.bluetooth")}>
      <h2 className={styles.title}>{messages.format("settings.bluetooth")}</h2>
      {permission === "allowed" ? (
        <>
          <p className={styles.note}>{messages.format("bluetooth.allowed")}</p>
          <Button size="compact" onPress={() => void openBluetoothSettings()}>
            {messages.format("bluetooth.open-settings")}
          </Button>
        </>
      ) : (
        <div className={styles.note}>
          <BluetoothPermission />
        </div>
      )}
    </section>
  );
}

/**
 * Settings › Keyboard's Globe key (MAC-8): whether a key acts as Globe on this Mac, with
 * Turn Off, which gives it back its own meaning. It's set up in the inspector's Globe
 * helper.
 */
function GlobeSetting() {
  const messages = useMessages();
  const [on, setOn] = useSettings(commands.globeKey);
  const [failed, setFailed] = useState(false);
  if (on === null) return null;
  const turnOff = async () => {
    const result = await commands.turnOffGlobe();
    setFailed(result.status === "error");
    setOn(false);
  };
  return (
    <section className={styles.setting} aria-label={messages.format("globe.setting")}>
      <h2 className={styles.title}>{messages.format("globe.setting")}</h2>
      <p className={styles.note}>{messages.format(on ? "globe.on" : "globe.off")}</p>
      {on ? (
        <Button size="compact" onPress={() => void turnOff()}>
          {messages.format("globe.turn-off")}
        </Button>
      ) : null}
      {failed ? (
        <p className={styles.problem}>{messages.format("error.globe-not-mapped")}</p>
      ) : null}
    </section>
  );
}

/**
 * The Keyboard tab (design §9.4): the input source whose labels keycaps show too, as
 * dual-printed keycaps do (MAC-6), from the layouts turned on in System Settings, or none;
 * whether Keywright may use Bluetooth (CON-6); and the Globe key (MAC-8), with Turn Off.
 */
export function KeyboardTab() {
  return (
    <>
      <SecondaryLabelsSetting />
      <BluetoothSetting />
      <GlobeSetting />
    </>
  );
}
