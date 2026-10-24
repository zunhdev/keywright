/**
 * The design system's gallery (design §10), for development only: the dev server serves
 * it at /gallery, outside the app, so it opens in Safari too. The release build has only
 * the app.
 */
import "@fontsource-variable/inter/wght.css";
import "@fontsource-variable/jetbrains-mono/wght.css";
import "../design-system/tokens.css";
import "../app/global.css";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { Gallery } from "./Gallery.tsx";

const root = document.querySelector("#root");
if (!root) {
  throw new Error("gallery.html should have an element with the id root");
}
createRoot(root).render(
  <StrictMode>
    <Gallery />
  </StrictMode>,
);
