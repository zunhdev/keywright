import { Button } from "../../design-system/Button.tsx";
import { Sheet } from "../../design-system/Sheet.tsx";
import { useMessages } from "../../i18n/messages.ts";
import type { Layer } from "../../ipc/bindings.ts";
import { removeLayer } from "./actions.ts";

import styles from "./Layers.module.css";

/**
 * Asks before removing a layer that keys turn on (LAY-5): which keys, where, and that they
 * won't do anything once it's gone. Removing can be undone (LAY-6).
 */
export function RemoveLayerSheet({ layer, onClose }: { layer: Layer; onClose: () => void }) {
  const messages = useMessages();
  const keys = layer.reach.filter((way) => way.position !== null);
  return (
    <Sheet
      title={messages.format("remove-layer.title", { name: layer.name })}
      isOpen
      onOpenChange={(open) => {
        if (!open) onClose();
      }}
    >
      {(close) => (
        <>
          <p className={styles.sheetBody}>{messages.format("remove-layer.body")}</p>
          <ul className={styles.keys}>
            {keys.map((way) => (
              <li key={way.position}>
                {way.place === null ? way.text : `${way.text}: ${way.place}`}
              </li>
            ))}
          </ul>
          <p className={styles.sheetBody}>{messages.format("remove-layer.undo")}</p>
          <div className={styles.sheetButtons}>
            <Button variant="quiet" onPress={close}>
              {messages.format("remove-layer.cancel")}
            </Button>
            <Button
              variant="danger"
              onPress={() => {
                close();
                void removeLayer(layer.id);
              }}
            >
              {messages.format("remove-layer.confirm")}
            </Button>
          </div>
        </>
      )}
    </Sheet>
  );
}
