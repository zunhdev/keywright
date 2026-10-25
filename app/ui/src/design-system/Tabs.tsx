import {
  Tab as AriaTab,
  TabList as AriaTabList,
  TabPanel as AriaTabPanel,
  type TabListProps,
  type TabPanelProps,
  type TabProps,
  Tabs as AriaTabs,
  type TabsProps,
} from "react-aria-components";

import styles from "./Tabs.module.css";

/**
 * Pages of a window, one shown at a time, such as Settings' General and Keyboard (design
 * §9.4): a `TabList` of `Tab`s, then a `TabPanel` for each, with the same `id`.
 */
export function Tabs(props: Omit<TabsProps, "className" | "style">) {
  return <AriaTabs {...props} className={styles.tabs} />;
}

/** The tabs, side by side, centred at the top. It needs an `aria-label`. */
export function TabList<T extends object>(props: Omit<TabListProps<T>, "className" | "style">) {
  return <AriaTabList {...props} className={styles.list} />;
}

/** A tab, with the `id` its panel has. */
export function Tab(props: Omit<TabProps, "className" | "style"> & { id: string }) {
  return <AriaTab {...props} className={styles.tab} />;
}

/** What a tab shows, with the `id` of its tab. */
export function TabPanel(props: Omit<TabPanelProps, "className" | "style"> & { id: string }) {
  return <AriaTabPanel {...props} className={styles.panel} />;
}
