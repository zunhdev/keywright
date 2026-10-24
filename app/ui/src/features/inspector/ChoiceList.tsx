import { useStore } from "../../app/store.ts";
import { List, ListItem } from "../../design-system/List.tsx";
import type { Action, Choice } from "../../ipc/bindings.ts";

import styles from "./Inspector.module.css";

/** Whether two actions are the same. */
export const sameAction = (a: Action | null, b: Action) => JSON.stringify(a) === JSON.stringify(b);

/** Whether a choice forgets Bluetooth pairings, which the list shows as destructive. */
const destructive = (action: Action) =>
  action.kind === "bluetooth" && (action.command === "clear" || action.command === "clear-all");

/**
 * The actions of one group that the firmware can do, such as Media & system's (design
 * §5.4), each with what it does: choosing one sets it.
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
  const all = useStore((store) => store.keymap?.choices);
  const choices = (all ?? []).filter((choice) => choice.category === category);
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
      {({ id, choice }) => (
        <ListItem id={id} textValue={choice.label}>
          <span
            className={styles.choice}
            data-destructive={destructive(choice.action) || undefined}
          >
            <span>{choice.label}</span>
            <span className={styles.description}>{choice.sentence}</span>
          </span>
        </ListItem>
      )}
    </List>
  );
}
