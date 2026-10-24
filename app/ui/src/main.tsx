/**
 * The UI's entry point, which index.html loads: it follows the session, its keymap and the
 * menu bar, and renders the app into the page (tech spec §12.2).
 */
// First, before React loads.
import "./no-performance-tracks.ts";
import "@fontsource-variable/inter/wght.css";
import "@fontsource-variable/jetbrains-mono/wght.css";
import "./design-system/tokens.css";
import "./app/global.css";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { App } from "./app/App.tsx";
import { followTextFocus } from "./app/text-focus.ts";
import { followConnection } from "./features/connection/follow.ts";
import { followKeymap, loadKeyTables } from "./ipc/keymap.ts";
import { followMenu } from "./ipc/menu.ts";
import { followSession } from "./ipc/session.ts";

const root = document.querySelector("#root");
if (!root) {
  throw new Error("index.html should have an element with the id root");
}
// For as long as the page is open.
void followSession();
void followKeymap();
void loadKeyTables();
void followMenu();
followTextFocus();
followConnection();
createRoot(root).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
