import { useStore } from "../../app/store.ts";
import { List, ListItem } from "../../design-system/List.tsx";
import { useMessages } from "../../i18n/messages.ts";
import type { Action, Choice } from "../../ipc/bindings.ts";
import { choicesOf, sameAction } from "./categories.ts";
import { MAC_ICONS } from "./mac-icons.ts";

import styles from "./Inspector.module.css";

/** Whether a choice forgets Bluetooth pairings, which the list shows as destructive. */
const destructive = (action: Action) =>
  action.kind === "bluetooth" && (action.command === "clear" || action.command === "clear-all");

/**
 * The actions of one group that the firmware can do, such as Media & system's (design
 * §5.4), each with what it does: choosing one sets it. A Mac action has its icon, and
 * says if it uses a default macOS shortcut, which people can change.
 */
export function ChoiceList({
  label,
  category,
  current,
  onChoose,
}: {
  label: string;
  category: Choice["category"];
  current: Action | null;
  onChoose: (choice: Choice) => void;
}) {
  const messages = useMessages();
  const keymap = useStore((store) => store.keymap);
  const macActions = useStore((store) => store.macActions);
  const choices = choicesOf(keymap, macActions).filter((choice) => choice.category === category);
  const selected = choices.findIndex((choice) => sameAction(current, choice.action));
  return (
    <List
      aria-label={label}
      items={choices.map((choice, index) => ({ id: index, choice }))}
      selectionMode="single"
      selectedKeys={selected === -1 ? [] : [selected]}
      onSelectionChange={(keys) => {
        const [index] = keys;
        const choice = choices[Number(index)];
        if (choice !== undefined) onChoose(choice);
      }}
    >
      {({ id, choice }) => {
        const Icon = choice.icon === null ? undefined : MAC_ICONS[choice.icon];
        return (
          <ListItem id={id} textValue={choice.label}>
            <span className={styles.choiceRow}>
              {Icon === undefined ? null : <Icon aria-hidden className={styles.choiceIcon} />}
              <span
                className={styles.choice}
                data-destructive={destructive(choice.action) || undefined}
              >
                <span>{choice.label}</span>
                <span className={styles.description}>{choice.sentence}</span>
                {choice.defaultShortcut === null ? null : (
                  <span className={styles.description}>
                    {messages.format("mac.default-shortcut", {
                      shortcut: choice.defaultShortcut,
                      name: choice.label,
                    })}
                  </span>
                )}
              </span>
            </span>
          </ListItem>
        );
      }}
    </List>
  );
}
