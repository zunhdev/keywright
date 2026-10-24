import { shownLayer, useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { ContextMenu, MenuItem, MenuSeparator } from "../../design-system/Menu.tsx";
import { Sheet } from "../../design-system/Sheet.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { copyAction, pasteAction, setKeys } from "./edits.ts";

import styles from "./KeyMenu.module.css";

/**
 * The selected keys' context menu (design §4.5). Restoring stock and showing a key in
 * Review come with stock keys (KEY-52) and the review sheet (KEY-50).
 */
export function KeyMenu({
  point,
  onClose,
}: {
  point: { x: number; y: number };
  onClose: () => void;
}) {
  const messages = useMessages();
  const copied = useStore((store) => store.copied);
  const changed = useStore((store) => {
    const keys = shownLayer(store)?.keys ?? [];
    return store.selection.some((position) => keys[position]?.changed === true);
  });
  return (
    <ContextMenu
      aria-label={messages.format("key-menu.label")}
      point={point}
      onClose={onClose}
      onAction={(item) => {
        if (item === "record") useStore.getState().startRecording();
        if (item === "copy") void copyAction();
        if (item === "paste") void pasteAction();
        if (item === "same-as-layer-below") void setKeys("same-as-layer-below");
        if (item === "does-nothing") void setKeys("does-nothing");
      }}
    >
      <MenuItem id="record" shortcut="R">
        {messages.format("key-menu.record")}
      </MenuItem>
      <MenuItem id="copy" shortcut="⌘C">
        {messages.format("key-menu.copy")}
      </MenuItem>
      <MenuItem id="paste" shortcut="⌘V" isDisabled={!copied}>
        {messages.format("key-menu.paste")}
      </MenuItem>
      <MenuSeparator />
      <MenuItem id="same-as-layer-below" shortcut="T">
        {messages.format("key-menu.same-as-layer-below")}
      </MenuItem>
      <MenuItem id="does-nothing" shortcut="⌫">
        {messages.format("key-menu.does-nothing")}
      </MenuItem>
      <MenuSeparator />
      <MenuItem id="restore" isDisabled>
        {messages.format("key-menu.restore")}
      </MenuItem>
      {changed ? (
        <MenuItem id="review" isDisabled>
          {messages.format("key-menu.review")}
        </MenuItem>
      ) : null}
    </ContextMenu>
  );
}

/** Where the canvas remembers that ⌫'s hint was confirmed, so it shows only once. */
const HINT_SEEN = "keywright.does-nothing-hint-seen";

/** Whether ⌫'s hint has been confirmed once. Without storage, it shows each time. */
export function doesNothingHintSeen(): boolean {
  try {
    return localStorage.getItem(HINT_SEEN) !== null;
  } catch {
    return false;
  }
}

/**
 * The hint the first ⌫ on the canvas shows (design §4.7): what "does nothing" means, and
 * how it differs from "same as layer below". Confirming makes the selected keys do
 * nothing, and the hint doesn't show again.
 */
export function DoesNothingHint({ onClose }: { onClose: () => void }) {
  const messages = useMessages();
  return (
    <Sheet
      title={messages.format("does-nothing.title")}
      isOpen
      onOpenChange={(open) => {
        if (!open) onClose();
      }}
    >
      {(close) => (
        <>
          <p className={styles.body}>{messages.format("does-nothing.body")}</p>
          <div className={styles.buttons}>
            <Button variant="quiet" onPress={close}>
              {messages.format("does-nothing.cancel")}
            </Button>
            <Button
              variant="primary"
              onPress={() => {
                try {
                  localStorage.setItem(HINT_SEEN, "yes");
                } catch {
                  // Without storage, the hint shows again next time.
                }
                close();
                void setKeys("does-nothing");
              }}
            >
              {messages.format("does-nothing.confirm")}
            </Button>
          </div>
        </>
      )}
    </Sheet>
  );
}
