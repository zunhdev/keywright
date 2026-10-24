import { I18nProvider } from "react-aria-components";

import { ToastRegion } from "../design-system/Toast.tsx";
import { LOCALE, useMessages } from "../i18n/messages.ts";
import { useStore } from "./store.ts";

import styles from "./App.module.css";

/**
 * The whole UI. It sets the locale for everything inside, React Aria's own strings
 * included, and shows the main window's content and its toasts.
 */
export function App() {
  return (
    <I18nProvider locale={LOCALE}>
      <MainWindow />
      <ToastRegion />
    </I18nProvider>
  );
}

/**
 * A placeholder until the three-pane main window (design §3): the keyboard's name, and
 * what the session is doing, as the toolbar will show them.
 */
function MainWindow() {
  const messages = useMessages();
  const session = useStore((store) => store.session);
  return (
    <main className={styles.main}>
      <h1>{session?.keyboard ?? messages.format("app.name")}</h1>
      {/* A live region (`output` has the status role), so VoiceOver says each new state. */}
      <output>{session ? messages.format(`session.${session.state.kind}`) : ""}</output>
    </main>
  );
}
