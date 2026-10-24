import { I18nProvider } from "react-aria-components";

import { LOCALE, useMessages } from "../i18n/messages.ts";

import styles from "./App.module.css";

/**
 * The whole UI. It sets the locale for everything inside, React Aria's own strings
 * included, and shows the main window's content.
 */
export function App() {
  return (
    <I18nProvider locale={LOCALE}>
      <MainWindow />
    </I18nProvider>
  );
}

/** A placeholder until the three-pane main window (design §3). */
function MainWindow() {
  const messages = useMessages();
  return (
    <main className={styles.main}>
      <h1>{messages.format("app.name")}</h1>
    </main>
  );
}
