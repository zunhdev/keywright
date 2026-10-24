import { useMessages } from "../../i18n/messages.ts";
import { LOCALE } from "../../i18n/messages.ts";
import type { Keymap, Layer } from "../../ipc/bindings.ts";
import { LayerName } from "../keyboard-canvas/Keycap.tsx";

import styles from "./Inspector.module.css";

/**
 * The inspector with no key selected (design §5.1): the layer shown, how many of its keys
 * are changed, and the knobs, which can't be changed yet (VIEW-4). How the layer is
 * reached, its Mac compatibility and its warnings come with the layers sidebar (KEY-45),
 * the compatibility check (KEY-62) and the lockout guard (KEY-51).
 */
export function LayerOverview({ keymap, layer }: { keymap: Keymap; layer: Layer }) {
  const messages = useMessages();
  const list = new Intl.ListFormat(LOCALE, { type: "conjunction" });
  return (
    <div className={styles.inspector}>
      <header className={styles.header}>
        <h2 className={styles.title}>
          <LayerName id={layer.id} name={layer.name} />
        </h2>
        <p className={styles.place}>
          {messages.format("overview.keys", {
            keys: layer.keys.length,
            changed: layer.keys.filter((key) => key.changed).length,
          })}
        </p>
      </header>
      <p className={styles.note}>{messages.format("overview.hint")}</p>
      {keymap.knobs.length === 0 ? null : (
        <section className={styles.fields}>
          <h3 className={styles.label}>{messages.format("overview.knobs")}</h3>
          <ul className={styles.list}>
            {keymap.knobs.map((knob) => (
              <li key={knob.position}>
                {messages.format("canvas.knob", { turn: list.format(knob.turn) })}
              </li>
            ))}
          </ul>
          <p className={styles.note}>{messages.format("overview.knobs-note")}</p>
        </section>
      )}
    </div>
  );
}
