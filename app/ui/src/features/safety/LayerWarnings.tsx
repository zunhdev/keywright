import { TriangleAlert } from "lucide-react";

import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Icon } from "../../design-system/Icon.tsx";
import { useMessages } from "../../i18n/messages.ts";
import type { KeyRef, Keymap, Layer } from "../../ipc/bindings.ts";
import { switchLayer } from "../keyboard-canvas/edits.ts";

import styles from "./Safety.module.css";

/** Shows the keys a warning is about: their layer, with them selected. */
function showKeys(keys: readonly KeyRef[]): void {
  const [first] = keys;
  if (first === undefined) return;
  switchLayer(first.layer);
  useStore
    .getState()
    .select(keys.filter((key) => key.layer === first.layer).map((key) => key.position));
}

/**
 * What's wrong with the keymap as it is, for a layer (design §5.1, SAF-2): no key can
 * unlock the keyboard, no key turns the layer on, or there's no way back from it, with
 * Show Keys for the keys that strand it.
 */
export function LayerWarnings({ keymap, layer }: { keymap: Keymap; layer: Layer }) {
  const messages = useMessages();
  const items = [
    ...(keymap.unlockable ? [] : [{ text: messages.format("warning.unlock"), keys: [] }]),
    ...layer.warnings.map((warning) =>
      warning.kind === "unreachable"
        ? { text: messages.format("warning.unreachable"), keys: [] }
        : { text: messages.format("warning.no-way-back"), keys: warning.keys },
    ),
  ];
  if (items.length === 0) return null;
  return (
    <section className={styles.warnings} aria-label={messages.format("warning.title")}>
      <h3 className={styles.title}>{messages.format("warning.title")}</h3>
      {items.map((item) => (
        <div key={item.text} className={styles.finding}>
          <span className={styles.icon}>
            <Icon icon={TriangleAlert} />
          </span>
          <span className={styles.text}>
            {item.text}
            {item.keys.length === 0 ? null : (
              <Button variant="quiet" size="compact" onPress={() => showKeys(item.keys)}>
                {messages.format("warning.show-keys")}
              </Button>
            )}
          </span>
        </div>
      ))}
    </section>
  );
}
