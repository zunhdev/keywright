/**
 * The UI's entry point, which index.html loads, for each of Keywright's windows: the main
 * window follows the session, its keymap, the keyboard's battery, the menu bar and its own
 * traffic lights, and the Settings window (`#settings`) shows the settings (tech spec
 * §12.2).
 */
// First, before React loads.
import "./no-performance-tracks.ts";
import "@fontsource-variable/inter/wght.css";
import "@fontsource-variable/jetbrains-mono/wght.css";
import "./design-system/tokens.css";
import "./app/global.css";
import { StrictMode } from "react";
import { I18nProvider } from "react-aria-components";
import { createRoot } from "react-dom/client";

import { App } from "./app/App.tsx";
import { followTextFocus } from "./app/text-focus.ts";
import { followQuit } from "./features/changes/QuitDialog.tsx";
import { followConnection } from "./features/connection/follow.ts";
import { SettingsWindow } from "./features/settings/SettingsWindow.tsx";
import { LOCALE } from "./i18n/messages.ts";
import { followBattery } from "./ipc/battery.ts";
import { followBluetoothPermission } from "./ipc/bluetooth.ts";
import { followKeymap, loadKeyTables } from "./ipc/keymap.ts";
import { followMenu } from "./ipc/menu.ts";
import { followSession } from "./ipc/session.ts";
import { followTrafficLights } from "./ipc/window.ts";

const root = document.querySelector("#root");
if (!root) {
  throw new Error("index.html should have an element with the id root");
}
if (location.hash === "#settings") {
  createRoot(root).render(
    <StrictMode>
      <I18nProvider locale={LOCALE}>
        <SettingsWindow />
      </I18nProvider>
    </StrictMode>,
  );
} else {
  // For as long as the page is open.
  void followSession();
  void followKeymap();
  void followBattery();
  void followBluetoothPermission();
  void loadKeyTables();
  void followMenu();
  void followTrafficLights();
  followTextFocus();
  followConnection();
  void followQuit();
  createRoot(root).render(
    <StrictMode>
      <App />
    </StrictMode>,
  );
}
