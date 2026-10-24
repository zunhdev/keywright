import { useEffect, useState } from "react";

import { List, ListItem } from "../design-system/List.tsx";
import { KeyboardCanvas } from "../features/keyboard-canvas/KeyboardCanvas.tsx";
import { LayerName } from "../features/keyboard-canvas/Keycap.tsx";
import { useMessages } from "../i18n/messages.ts";
import type { Layer } from "../ipc/bindings.ts";
import { inspectorPlacement } from "./layout.ts";
import { Splitter } from "./Splitter.tsx";
import { keyboardWidth, shownLayer, useStore } from "./store.ts";
import { WindowToolbar } from "./WindowToolbar.tsx";

import styles from "./MainWindow.module.css";

/** The layers before there's a keymap: the same empty list each time, for the store. */
const NO_LAYERS: readonly Layer[] = [];

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
 * them in that order. The inspector shows a placeholder until it's built.
 */
export function MainWindow() {
  const messages = useMessages();
  const width = useWindowWidth();
  const layers = useStore((store) => store.keymap?.layers ?? NO_LAYERS);
  const shown = useStore(shownLayer);
  const showLayer = useStore((store) => store.showLayer);
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
          <h2 className={styles.caption}>{messages.format("layers.title")}</h2>
          <List
            aria-label={messages.format("layers.title")}
            items={layers}
            selectionMode="single"
            disallowEmptySelection
            selectedKeys={shown === null ? [] : [shown.id]}
            onSelectionChange={(keys) => {
              const [id] = keys;
              if (typeof id === "number") showLayer(id);
            }}
            renderEmptyState={() => (
              <p className={styles.empty}>{messages.format("layers.empty")}</p>
            )}
          >
            {(layer) => (
              <ListItem id={layer.id} textValue={layer.name}>
                <LayerName id={layer.id} name={layer.name} />
              </ListItem>
            )}
          </List>
          <Splitter panel="layers" />
        </aside>
      ) : null}
      <main className={styles.canvas} aria-label={messages.format("canvas.label")}>
        <KeyboardCanvas />
      </main>
      {inspector === null ? null : (
        <aside
          className={styles.inspector}
          aria-label={messages.format("inspector.label")}
          data-placement={inspector}
          style={{ width: paneWidths.inspector }}
        >
          <p className={styles.empty}>{messages.format("inspector.empty")}</p>
          <Splitter panel="inspector" />
        </aside>
      )}
    </div>
  );
}
