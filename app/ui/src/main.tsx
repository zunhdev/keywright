/**
 * The UI's entry point, which index.html loads: it renders the app into the page
 * (tech spec §12.2).
 */
import "./app/global.css";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { App } from "./app/App.tsx";

const root = document.querySelector("#root");
if (!root) {
  throw new Error("index.html should have an element with the id root");
}
createRoot(root).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
