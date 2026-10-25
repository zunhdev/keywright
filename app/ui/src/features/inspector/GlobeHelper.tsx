import { useEffect, useState } from "react";

import { Button } from "../../design-system/Button.tsx";
import { Switch } from "../../design-system/Switch.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { commands, type OpenAtLogin } from "../../ipc/bindings.ts";
import { makeKeyGlobe } from "../keyboard-canvas/edits.ts";

import styles from "./Inspector.module.css";

/**
 * The Globe helper, under the Mac actions (design §5.4, MAC-8): macOS takes the Globe key
 * only from Apple keyboards, so Use This Key as Globe makes the selected key act as Globe
 * on this Mac. It works while Keywright runs, so Open at login is here too, as macOS has
 * it recorded.
 */
export function GlobeHelper() {
  const messages = useMessages();
  const [login, setLogin] = useState<OpenAtLogin | null>(null);
  useEffect(() => {
    let current = true;
    void commands.generalSettings().then((general) => {
      if (current) setLogin(general.openAtLogin);
      return general;
    });
    return () => {
      current = false;
    };
  }, []);
  const changeOpenAtLogin = async (on: boolean) => {
    setLogin(await commands.setOpenAtLogin(on));
  };
  return (
    <section className={styles.fields} aria-label={messages.format("globe.title")}>
      <h3 className={styles.label}>{messages.format("globe.title")}</h3>
      <p className={styles.note}>{messages.format("globe.explanation")}</p>
      <Button size="compact" onPress={() => void makeKeyGlobe()}>
        {messages.format("globe.use")}
      </Button>
      <p className={styles.note}>{messages.format("globe.while-running")}</p>
      {login === null ? null : (
        <Switch isSelected={login.on} onChange={(on) => void changeOpenAtLogin(on)}>
          {messages.format("settings.open-at-login")}
        </Switch>
      )}
    </section>
  );
}
