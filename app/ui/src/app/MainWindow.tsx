import { useEffect, useState } from "react";

import { DiscardSheet } from "../features/changes/DiscardSheet.tsx";
import { QuitDialog } from "../features/changes/QuitDialog.tsx";
import { ReviewSheet } from "../features/changes/ReviewSheet.tsx";
import { UnlockSheet } from "../features/connection/UnlockSheet.tsx";
import { Inspector } from "../features/inspector/Inspector.tsx";
import { KeyboardCanvas } from "../features/keyboard-canvas/KeyboardCanvas.tsx";
import { LayersPane } from "../features/layers/LayersPane.tsx";
import { useMessages } from "../i18n/messages.ts";
import { inspectorPlacement } from "./layout.ts";
import { Splitter } from "./Splitter.tsx";
import { keyboardWidth, useStore } from "./store.ts";
import { WindowToolbar } from "./WindowToolbar.tsx";

import styles from "./MainWindow.module.css";

/** The window's width, in px, as it changes. */
function useWindowWidth(): number {
  const [width, setWidth] = useState(() => window.innerWidth);
  useEffect(() => {
    const measure = () => {
      setWidth(window.innerWidth);
    };
    window.addEventListener("resize", measure);
    return () => {
      window.removeEventListener("resize", measure);
    };
  }, []);
  return width;
}

/**
 * The main window (design §3): the toolbar over three panes, the layers, the keyboard
 * canvas and the inspector, in that order on the page, so keyboard focus moves through
 * them in that order.
 */
export function MainWindow() {
  const messages = useMessages();
  const width = useWindowWidth();
  const keyboard = useStore((store) => keyboardWidth(store.keymap));
  const panels = useStore((store) => store.panels);
  const paneWidths = useStore((store) => store.paneWidths);
  const fitWindow = useStore((store) => store.fitWindow);
  useEffect(() => {
    fitWindow(width, keyboard);
  }, [width, keyboard, fitWindow]);
  const inspector = panels.inspector
    ? inspectorPlacement(width, paneWidths, panels.layers, keyboard)
    : null;

  return (
    <div className={styles.window}>
      <WindowToolbar />
      {panels.layers ? (
        <aside
          className={styles.layers}
          aria-label={messages.format("layers.title")}
          style={{ width: paneWidths.layers }}
        >
          <LayersPane />
          <Splitter panel="layers" />
        </aside>
      ) : null}
      <main className={styles.canvas} aria-label={messages.format("canvas.label")}>
        <KeyboardCanvas />
      </main>
      <UnlockSheet />
      <ReviewSheet />
      <DiscardSheet />
      <QuitDialog />
      {inspector === null ? null : (
        <aside
          className={styles.inspector}
          aria-label={messages.format("inspector.label")}
          data-placement={inspector}
          style={{ width: paneWidths.inspector }}
        >
          <Inspector />
          <Splitter panel="inspector" />
        </aside>
      )}
    </div>
  );
}
