import { useEffect, useState } from "react";

import { Button } from "../../design-system/Button.tsx";
import { Switch } from "../../design-system/Switch.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { commands, type GeneralSettings } from "../../ipc/bindings.ts";

import styles from "./Settings.module.css";

/**
 * The Settings window (design §9.4), with its General tab: Show in menu bar (BAR-1, BAR-4)
 * and Open at login (BAR-5). Open at login shows what macOS has recorded, read each time
 * the window shows, since it can change in System Settings. The other tabs come with
 * KEY-67.
 */
export function SettingsWindow() {
  const messages = useMessages();
  const [general, setGeneral] = useState<GeneralSettings | null>(null);
  const [failed, setFailed] = useState(false);

  useEffect(() => {
    const read = async () => {
      setGeneral(await commands.generalSettings());
    };
    void read();
    const onShow = () => {
      if (document.visibilityState === "visible") void read();
    };
    document.addEventListener("visibilitychange", onShow);
    return () => {
      document.removeEventListener("visibilitychange", onShow);
    };
  }, []);

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
    <main className={styles.window}>
      <h1 className={styles.tab}>{messages.format("settings.general")}</h1>
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
    </main>
  );
}
