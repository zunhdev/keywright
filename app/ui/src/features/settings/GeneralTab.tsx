import { useState } from "react";

import { Button } from "../../design-system/Button.tsx";
import { NumberField } from "../../design-system/NumberField.tsx";
import { Segment, SegmentedControl } from "../../design-system/SegmentedControl.tsx";
import { Switch } from "../../design-system/Switch.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { type Appearance, commands, type GeneralSettings } from "../../ipc/bindings.ts";
import { useSettings } from "./use-settings.ts";

import styles from "./Settings.module.css";

/** The appearances, in the order the segmented control shows them. */
const APPEARANCES = ["system", "light", "dark"] as const satisfies readonly Appearance[];

/** The settings on the General tab that can fail to change, by what they're called. */
type Setting = "menu-bar" | "low-battery" | "appearance" | "zmk-names";

/**
 * The General tab (design §9.4): Show in menu bar (BAR-1, BAR-4) and the low-battery level
 * (BAR-3), Open at login (BAR-5), Appearance, and Show ZMK names (EDIT-6). Each takes
 * effect at once. Open at login shows what macOS has recorded, since it can change in
 * System Settings.
 */
export function GeneralTab() {
  const messages = useMessages();
  const [general, setGeneral] = useSettings(commands.generalSettings);
  const [failed, setFailed] = useState<Setting | null>(null);
  if (general === null) return null;
  /**
   * Shows `changed`, and says so if the setting couldn't be changed. A setting that
   * couldn't be saved still holds until Keywright quits; one that the menu bar didn't take
   * stays as it was.
   */
  const changed = (
    setting: Setting,
    result: { readonly status: "ok" | "error" },
    to: Partial<GeneralSettings>,
  ) => {
    setFailed(result.status === "error" ? setting : null);
    if (result.status === "ok" || setting !== "menu-bar") setGeneral({ ...general, ...to });
  };
  const problem = (setting: Setting) =>
    failed === setting ? (
      <p className={styles.problem}>{messages.format("error.setting-not-changed")}</p>
    ) : null;
  const changeMenuBar = async (on: boolean) => {
    changed("menu-bar", await commands.setMenuBar(on), { menuBar: on });
  };
  const changeZmkNames = async (on: boolean) => {
    changed("zmk-names", await commands.setZmkNames(on), { zmkNames: on });
  };
  const login = general.openAtLogin;
  const changeOpenAtLogin = async (on: boolean) => {
    setGeneral({ ...general, openAtLogin: await commands.setOpenAtLogin(on) });
  };
  const changeLowBattery = async (percent: number) => {
    if (Number.isNaN(percent)) return;
    const result = await commands.setLowBattery(percent);
    changed("low-battery", result, {
      lowBattery: result.status === "ok" ? result.data : percent,
    });
  };
  const chooseAppearance = async (keys: Set<unknown>) => {
    const appearance = APPEARANCES.find((choice) => keys.has(choice));
    if (appearance === undefined) return;
    changed("appearance", await commands.setAppearance(appearance), { appearance });
  };
  return (
    <>
      <section className={styles.setting}>
        <Switch isSelected={general.menuBar} onChange={(on) => void changeMenuBar(on)}>
          {messages.format("settings.menu-bar")}
        </Switch>
        <p className={styles.note}>{messages.format("settings.menu-bar-note")}</p>
        {problem("menu-bar")}
      </section>
      <section className={styles.setting}>
        <div className={styles.number}>
          <NumberField
            label={messages.format("settings.low-battery")}
            value={general.lowBattery}
            minValue={general.lowBatteryRange.min}
            maxValue={general.lowBatteryRange.max}
            formatOptions={{ style: "unit", unit: "percent" }}
            onChange={(percent) => void changeLowBattery(percent)}
          />
        </div>
        <p className={styles.note}>{messages.format("settings.low-battery-note")}</p>
        {problem("low-battery")}
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
      <section className={styles.setting} aria-label={messages.format("settings.appearance")}>
        <h2 className={styles.title}>{messages.format("settings.appearance")}</h2>
        <SegmentedControl
          aria-label={messages.format("settings.appearance")}
          selectedKeys={[general.appearance]}
          onSelectionChange={(keys) => void chooseAppearance(keys)}
        >
          {APPEARANCES.map((appearance) => (
            <Segment key={appearance} id={appearance}>
              {messages.format(`settings.appearance.${appearance}`)}
            </Segment>
          ))}
        </SegmentedControl>
        {problem("appearance")}
      </section>
      <section className={styles.setting}>
        <Switch isSelected={general.zmkNames} onChange={(on) => void changeZmkNames(on)}>
          {messages.format("settings.zmk-names")}
        </Switch>
        <p className={styles.note}>{messages.format("settings.zmk-names-note")}</p>
        {problem("zmk-names")}
      </section>
    </>
  );
}
