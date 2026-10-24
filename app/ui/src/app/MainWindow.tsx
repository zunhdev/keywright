import { useEffect, useState } from "react";

import { List } from "../design-system/List.tsx";
import { useMessages } from "../i18n/messages.ts";
import { inspectorPlacement } from "./layout.ts";
import { Splitter } from "./Splitter.tsx";
import { useStore } from "./store.ts";
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
 * them in that order. The panes show placeholders until the layers list, the canvas and
 * the inspector are built.
 */
export function MainWindow() {
  const messages = useMessages();
  const width = useWindowWidth();
  const session = useStore((store) => store.session);
  const panels = useStore((store) => store.panels);
  const paneWidths = useStore((store) => store.paneWidths);
  const fitWindow = useStore((store) => store.fitWindow);
  useEffect(() => {
    fitWindow(width);
  }, [width, fitWindow]);
  const inspector = panels.inspector ? inspectorPlacement(width, paneWidths, panels.layers) : null;

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
            items={[]}
            renderEmptyState={() => (
              <p className={styles.empty}>{messages.format("layers.empty")}</p>
            )}
          >
            {() => null}
          </List>
          <Splitter panel="layers" />
        </aside>
      ) : null}
      <main className={styles.canvas} aria-label={messages.format("canvas.label")}>
        {/* A live region (`output` has the status role), so VoiceOver says each new state. */}
        <output className={styles.state}>
          {session ? messages.format(`session.${session.state.kind}`) : ""}
        </output>
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
