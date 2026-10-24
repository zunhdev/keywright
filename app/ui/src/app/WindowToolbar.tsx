import { Button } from "../design-system/Button.tsx";
import { Toolbar } from "../design-system/Toolbar.tsx";
import { KeySearch } from "../features/search/KeySearch.tsx";
import { useMessages } from "../i18n/messages.ts";
import { ConnectionChip } from "./ConnectionChip.tsx";
import { useStore } from "./store.ts";

import styles from "./WindowToolbar.module.css";

/**
 * The window's toolbar (design §3.2), which is also where the window is dragged: the
 * keyboard's name, the connection chip, search, the changes button, and Save.
 */
export function WindowToolbar() {
  const messages = useMessages();
  const session = useStore((store) => store.session);
  const changes = session?.changes ?? 0;
  return (
    <header className={styles.toolbar} data-tauri-drag-region>
      <h1 className={styles.title} data-tauri-drag-region>
        {session?.keyboard ?? messages.format("app.name")}
      </h1>
      <Toolbar aria-label={messages.format("toolbar.label")}>
        <ConnectionChip />
        <div className={styles.search} data-tauri-drag-region>
          <KeySearch />
        </div>
        {changes > 0 ? (
          <Button size="compact">{messages.format("toolbar.changes", { count: changes })}</Button>
        ) : null}
        {/* Saving always goes through Review (design §6). */}
        <Button variant="primary" size="compact" isDisabled={changes === 0}>
          {messages.format("toolbar.save")}
        </Button>
      </Toolbar>
    </header>
  );
}
