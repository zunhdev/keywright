import { useMessages } from "../../i18n/messages.ts";
import { LOCALE } from "../../i18n/messages.ts";
import type { Keymap, Layer } from "../../ipc/bindings.ts";
import { LayerName } from "../keyboard-canvas/Keycap.tsx";
import { RenameField } from "../layers/RenameField.tsx";
import { FixAll, MacFindings } from "../mac-check/MacFindings.tsx";
import { LayerWarnings } from "../safety/LayerWarnings.tsx";

import styles from "./Inspector.module.css";

/**
 * The inspector with no key selected (design §5.1): the layer shown, how many of its keys
 * are changed, its name to edit, how it's reached (LAY-4), its keys that don't work on a
 * Mac, with their fixes (MAC-7), what's wrong with it (SAF-2), and the knobs, which can't
 * be changed yet (VIEW-4).
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
      <RenameField
        // Another layer, or a name changed elsewhere, starts the field afresh.
        key={`${layer.id}:${layer.name}`}
        id={layer.id}
        name={layer.name}
        max={keymap.layerNameMax}
        label={messages.format("overview.name")}
      />
      <section className={styles.fields}>
        <h3 className={styles.label}>{messages.format("overview.reach")}</h3>
        <ul className={styles.list}>
          {layer.reach.map((way) => (
            <li key={`${way.text}:${way.position ?? ""}`}>
              {way.text}
              {way.place === null ? null : (
                <span className={styles.description}> · {way.place}</span>
              )}
            </li>
          ))}
        </ul>
      </section>
      {layer.macFindings.length === 0 ? null : (
        <section className={styles.fields} aria-label={messages.format("mac.section")}>
          <h3 className={styles.label}>{messages.format("mac.section")}</h3>
          <MacFindings findings={layer.macFindings} />
          {layer.macFindings.filter((found) => found.fix !== null).length > 1 ? (
            <FixAll findings={layer.macFindings} />
          ) : null}
        </section>
      )}
      <LayerWarnings keymap={keymap} layer={layer} />
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
