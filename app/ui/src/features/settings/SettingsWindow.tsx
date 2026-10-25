import { useEffect, useState } from "react";
import type { Key } from "react-aria-components";

import { Button } from "../../design-system/Button.tsx";
import { Select, SelectItem } from "../../design-system/Select.tsx";
import { Switch } from "../../design-system/Switch.tsx";
import { Tab, TabList, TabPanel, Tabs } from "../../design-system/Tabs.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { commands, type SecondaryLabels } from "../../ipc/bindings.ts";

import styles from "./Settings.module.css";

/** What the secondary labels' choice calls "no input source". */
const NONE = "none";

/**
 * Reads a tab's settings with `read`, and again each time the window shows, since some can
 * change outside Keywright, such as in System Settings. `null` until they're read.
 */
function useSettings<T>(read: () => Promise<T>): [T | null, (settings: T) => void] {
  const [settings, setSettings] = useState<T | null>(null);
  useEffect(() => {
    const load = async () => {
      setSettings(await read());
    };
    void load();
    const onShow = () => {
      if (document.visibilityState === "visible") void load();
    };
    document.addEventListener("visibilitychange", onShow);
    return () => {
      document.removeEventListener("visibilitychange", onShow);
    };
  }, [read]);
  return [settings, setSettings];
}

/**
 * The General tab: Show in menu bar (BAR-1, BAR-4) and Open at login (BAR-5). Open at
 * login shows what macOS has recorded, since it can change in System Settings.
 */
function General() {
  const messages = useMessages();
  const [general, setGeneral] = useSettings(commands.generalSettings);
  const [failed, setFailed] = useState(false);
  if (general === null) return null;
  const login = general.openAtLogin;
  const changeMenuBar = async (on: boolean) => {
    const result = await commands.setMenuBar(on);
    setFailed(result.status === "error");
    if (result.status === "ok") setGeneral({ ...general, menuBar: on });
  };
  const changeOpenAtLogin = async (on: boolean) => {
    setGeneral({ ...general, openAtLogin: await commands.setOpenAtLogin(on) });
  };
  return (
    <>
      <section className={styles.setting}>
        <Switch isSelected={general.menuBar} onChange={(on) => void changeMenuBar(on)}>
          {messages.format("settings.menu-bar")}
        </Switch>
        <p className={styles.note}>{messages.format("settings.menu-bar-note")}</p>
        {failed ? (
          <p className={styles.problem}>{messages.format("error.setting-not-changed")}</p>
        ) : null}
      </section>
      <section className={styles.setting}>
        <Switch isSelected={login.on} onChange={(on) => void changeOpenAtLogin(on)}>
          {messages.format("settings.open-at-login")}
        </Switch>
        {login.problem === null ? null : (
          <>
            <p className={styles.problem}>{messages.format(`settings.login-${login.problem}`)}</p>
            <Button size="compact" onPress={() => void commands.openLoginItemsSettings()}>
              {messages.format("settings.open-login-items")}
            </Button>
          </>
        )}
      </section>
    </>
  );
}

/**
 * The Keyboard tab: the input source whose labels keycaps show too, as dual-printed
 * keycaps do (MAC-6), from the layouts turned on in System Settings, or none; and the
 * Globe key (MAC-8), with Turn Off.
 */
function Keyboard() {
  return (
    <>
      <SecondaryLabelsSetting />
      <GlobeSetting />
    </>
  );
}

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
 * The Settings window (design §9.4): the General tab, and the Keyboard tab with the
 * secondary labels (MAC-6) and the Globe key (MAC-8). The other settings come with KEY-67.
 */
export function SettingsWindow() {
  const messages = useMessages();
  return (
    <main className={styles.window}>
      <Tabs>
        <TabList aria-label={messages.format("settings.tabs")}>
          <Tab id="general">{messages.format("settings.general")}</Tab>
          <Tab id="keyboard">{messages.format("settings.keyboard")}</Tab>
        </TabList>
        <TabPanel id="general">
          <General />
        </TabPanel>
        <TabPanel id="keyboard">
          <Keyboard />
        </TabPanel>
      </Tabs>
    </main>
  );
}
