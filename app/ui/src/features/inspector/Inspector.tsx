import { shownLayer, useStore } from "../../app/store.ts";
import { useMessages } from "../../i18n/messages.ts";
import { KeyInspector } from "./KeyInspector.tsx";
import { LayerOverview } from "./LayerOverview.tsx";

import styles from "./Inspector.module.css";

/**
 * The inspector pane's content (design §5): always something, the selected keys, or the
 * layer shown when no key is. While the keymap can't be changed, such as when the
 * keyboard is locked, it can't be used.
 */
export function Inspector() {
  const messages = useMessages();
  const keymap = useStore((store) => store.keymap);
  const layer = useStore(shownLayer);
  const selection = useStore((store) => store.selection);
  const editable = useStore((store) => store.session?.state.kind === "ready");
  if (keymap === null || layer === null) {
    return <p className={styles.empty}>{messages.format("inspector.empty")}</p>;
  }
  const keys = selection.filter((position) => layer.keys[position] !== undefined);
  return (
    <div className={styles.pane} inert={!editable} data-disabled={editable ? undefined : true}>
      {keys.length === 0 ? (
        <LayerOverview keymap={keymap} layer={layer} />
      ) : (
        // A new selection, or another layer, starts the inspector afresh.
        <KeyInspector
          key={`${layer.id}:${keys.join(",")}`}
          keymap={keymap}
          layer={layer}
          positions={keys}
        />
      )}
    </div>
  );
}
