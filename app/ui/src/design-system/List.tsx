import type { ReactNode } from "react";
import {
  GridList,
  GridListItem,
  type GridListItemProps,
  type GridListProps,
} from "react-aria-components";

import styles from "./List.module.css";

/**
 * A list whose rows can be selected and have actions, such as the layers sidebar (design
 * §12): one tab stop, with the arrow keys moving between rows and into their actions.
 * It needs an `aria-label`.
 */
export function List<T extends object>(props: Omit<GridListProps<T>, "className" | "style">) {
  return <GridList {...props} className={styles.list} />;
}

export interface ListItemProps extends Omit<GridListItemProps, "className" | "style" | "children"> {
  /** What the row says, which is also what VoiceOver reads and typing finds. */
  textValue: string;
  children: ReactNode;
  /** Buttons for the row, which show when it's hovered, focused or selected. */
  actions?: ReactNode;
}

/** One row of a `List`. */
export function ListItem({ children, actions, ...props }: ListItemProps) {
  return (
    <GridListItem {...props} className={styles.item}>
      {children}
      {actions === undefined ? null : <div className={styles.actions}>{actions}</div>}
    </GridListItem>
  );
}
