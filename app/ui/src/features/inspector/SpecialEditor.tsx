import { useState } from "react";

import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Sheet } from "../../design-system/Sheet.tsx";
import { useMessages } from "../../i18n/messages.ts";
import type { Action, Choice } from "../../ipc/bindings.ts";
import { ChoiceList } from "./ChoiceList.tsx";
import { ModifierPicker } from "./fields.tsx";

import styles from "./Inspector.module.css";

/**
 * Special (design §5.4): Caps Word, Repeat, Unlock editing, Update mode and Restart, and a
 * sticky modifier. Update mode asks first, since pressing it stops the keyboard.
 */
export function SpecialEditor({
  current,
  onChange,
}: {
  current: Action | null;
  onChange: (action: Action, choice?: Choice) => void;
}) {
  const messages = useMessages();
  const kinds = useStore((store) => store.keymap?.kinds) ?? [];
  const [confirming, setConfirming] = useState<Choice | null>(null);
  return (
    <div className={styles.fields}>
      <ChoiceList
        label={messages.format("category.special")}
        category="special"
        current={current}
        onChoose={(choice) => {
          if (choice.action.kind === "bootloader") setConfirming(choice);
          else onChange(choice.action, choice);
        }}
      />
      {kinds.includes("sticky-key") ? (
        <div className={styles.fields}>
          <ModifierPicker
            label={messages.format("special.sticky")}
            value={current?.kind === "sticky-key" ? current.key : null}
            onChange={(key) => {
              onChange({ kind: "sticky-key", key });
            }}
          />
          <p className={styles.note}>{messages.format("special.sticky-note")}</p>
        </div>
      ) : null}
      {confirming === null ? null : (
        <Sheet
          title={messages.format("update-mode.title")}
          isOpen
          onOpenChange={(open) => {
            if (!open) setConfirming(null);
          }}
        >
          {(close) => (
            <>
              <p className={styles.sheetBody}>{messages.format("update-mode.body")}</p>
              <div className={styles.buttons}>
                <Button variant="quiet" onPress={close}>
                  {messages.format("update-mode.cancel")}
                </Button>
                <Button
                  variant="danger"
                  onPress={() => {
                    close();
                    onChange(confirming.action, confirming);
                  }}
                >
                  {messages.format("update-mode.confirm")}
                </Button>
              </div>
            </>
          )}
        </Sheet>
      )}
    </div>
  );
}
