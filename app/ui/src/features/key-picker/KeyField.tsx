import { ChevronDown } from "lucide-react";
import { useState } from "react";
import { Dialog, DialogTrigger, Popover } from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Icon } from "../../design-system/Icon.tsx";
import { useMessages } from "../../i18n/messages.ts";
import type { KeyEntry, Page } from "../../ipc/bindings.ts";
import { characterCount } from "../keyboard-canvas/label.ts";
import { catalogKey, type Chosen, keyId } from "./boards.ts";
import { KeyPicker } from "./KeyPicker.tsx";
import { rememberKey } from "./recents.ts";

import overlay from "../../design-system/overlay.module.css";
import styles from "./KeyPicker.module.css";

/**
 * A key field (design §5.6): a button with the key's keycap and name, which opens the key
 * picker beside it. ↓ opens it too, and a character typed on it opens it searching for
 * that character. A key chosen goes to `onChange`, with whether ⇧ is to be held with it,
 * as a character needs, or left alone (`null`), and joins the recent keys.
 */
export function KeyField({
  label,
  value,
  shifted = false,
  onChange,
}: {
  label: string;
  value: { page: Page; id: number } | null;
  /** Whether ⇧ is held with the key, so that the picker shows ⇧[ as {. */
  shifted?: boolean;
  onChange: (key: KeyEntry, shift: boolean | null) => void;
}) {
  const messages = useMessages();
  const keys = useStore((store) => store.keyCatalog);
  const [open, setOpen] = useState(false);
  const [initialQuery, setInitialQuery] = useState("");
  // A right-hand modifier shows as its left-hand twin; the field's owner keeps the side.
  const chosen: Chosen | null =
    value === null ? null : { key: keyId(catalogKey(value)), shift: shifted };
  const key = keys.find((each) => keyId(each) === chosen?.key);
  const name =
    key === undefined
      ? messages.format("picker.choose")
      : key.cap === key.name
        ? key.name
        : `${key.cap} ${key.name}`;
  const openWith = (query: string) => {
    setInitialQuery(query);
    setOpen(true);
  };
  return (
    <div className={styles.field}>
      <span className={styles.fieldLabel}>{label}</span>
      <DialogTrigger
        isOpen={open}
        onOpenChange={(next) => {
          if (next) openWith("");
          else setOpen(false);
        }}
      >
        <Button
          aria-label={messages.format("picker.field", {
            label,
            key: key === undefined ? name : key.spoken,
          })}
          onKeyDown={(event) => {
            if (event.key === "ArrowDown") {
              openWith("");
            } else if (
              event.key !== " " &&
              characterCount(event.key) === 1 &&
              !event.metaKey &&
              !event.ctrlKey &&
              !event.altKey
            ) {
              openWith(event.key);
            } else {
              event.continuePropagation();
            }
          }}
        >
          {key === undefined || key.cap === key.name ? null : (
            <span className={styles.fieldCap}>{key.cap}</span>
          )}
          <span className={styles.fieldName}>{key?.name ?? name}</span>
          <Icon icon={ChevronDown} />
        </Button>
        <Popover placement="left top" offset={8} className={`${overlay.popover} ${styles.popover}`}>
          <Dialog aria-label={messages.format("picker.title")} className={styles.dialog}>
            <KeyPicker
              chosen={chosen}
              initialQuery={initialQuery}
              onChoose={(picked, shift) => {
                rememberKey({ key: keyId(picked), shift: shift === true });
                setOpen(false);
                onChange(picked, shift);
              }}
            />
          </Dialog>
        </Popover>
      </DialogTrigger>
    </div>
  );
}
