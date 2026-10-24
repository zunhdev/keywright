import { I18nProvider } from "react-aria-components";

import { ToastRegion } from "../design-system/Toast.tsx";
import { LOCALE } from "../i18n/messages.ts";
import { MainWindow } from "./MainWindow.tsx";

/**
 * The whole UI. It sets the locale for everything inside, React Aria's own strings
 * included, and shows the main window and its toasts.
 */
export function App() {
  return (
    <I18nProvider locale={LOCALE}>
      <MainWindow />
      <ToastRegion />
    </I18nProvider>
  );
}
