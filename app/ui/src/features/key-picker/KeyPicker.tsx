import { useMemo, useRef, useState } from "react";
import {
  Input,
  SearchField,
  Tabs as AriaTabs,
  TabPanel as AriaTabPanel,
} from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { Tab, TabList } from "../../design-system/Tabs.tsx";
import { useMessages } from "../../i18n/messages.ts";
import type { KeyEntry } from "../../ipc/bindings.ts";
import {
  boards,
  type Chosen,
  flow,
  keyId,
  PICKER_HEIGHT,
  PICKER_WIDTH,
  type Placed,
  TABS,
  type TabId,
  tabOf,
} from "./boards.ts";
import { BoardView, UNIT } from "./BoardView.tsx";
import { recentKeys } from "./recents.ts";
import { searchKeys } from "./search.ts";

import styles from "./KeyPicker.module.css";

/** What the footer says about a key: its keycap and name, or Shift's character and the key. */
function described(placed: Placed): string {
  const { key, shift } = placed;
  const named = key.cap === key.name ? key.name : `${key.cap}  ${key.name}`;
  return shift ? `${key.shifted ?? ""}  ⇧${named}` : named;
}

/**
 * What choosing a key means for ⇧: a character holds it just when it needs it, and
 * another key leaves the modifiers as they are (`null`).
 */
const shiftOf = (placed: Placed) => (placed.character ? placed.shift : null);

/**
 * The key picker (design §5.6): a search, then the tabs, Recent and the tab's keys, or
 * the keys found while searching, and a footer that names the key under the pointer or
 * the focus. It exists while it's open, so each time it starts afresh, on the tab with
 * the field's key and with the latest recent keys.
 */
export function KeyPicker({
  chosen,
  initialQuery,
  onChoose,
}: {
  /** The field's key, if it has one. */
  chosen: Chosen | null;
  /** What the search starts with, such as a character typed on the field. */
  initialQuery: string;
  /** Says which key was chosen, and whether ⇧ is to be held with it, or left alone. */
  onChoose: (key: KeyEntry, shift: boolean | null) => void;
}) {
  const messages = useMessages();
  const keys = useStore((store) => store.keyCatalog);
  const keyboard = useStore((store) => store.keyboardType);
  const all = useMemo(() => boards(keys, keyboard), [keys, keyboard]);
  const [tab, setTab] = useState<TabId>(() => tabOf(all, chosen) ?? "keyboard");
  const [query, setQuery] = useState(initialQuery);
  const [hovered, setHovered] = useState<Placed | null>(null);
  const [focused, setFocused] = useState<Placed | null>(null);
  const [recent] = useState(() => {
    const catalog = new Map(keys.map((key) => [keyId(key), key] as const));
    const recents = recentKeys().flatMap(({ key: id, shift }) => {
      const key = catalog.get(id);
      return key === undefined ? [] : [{ key, shift, character: shift }];
    });
    return flow([{ title: null, keys: recents }], PICKER_WIDTH);
  });
  const found = useMemo(() => {
    const more = new Set(all.more.keys.map((placed) => keyId(placed.key)));
    return searchKeys(keys, query, more);
  }, [all, keys, query]);
  const results = useMemo(() => {
    const picks = found.map(({ key, shift }) => ({ key, shift, character: shift }));
    return flow([{ title: null, keys: picks }], PICKER_WIDTH, { captions: true });
  }, [found]);
  const searching = query.trim() !== "";
  const search = useRef<HTMLInputElement>(null);
  const shown = useRef<HTMLDivElement>(null);

  const choose = (placed: Placed) => {
    onChoose(placed.key, shiftOf(placed));
  };
  const handlers = {
    chosen,
    onChoose: choose,
    onHover: setHovered,
    onFocus: setFocused,
    onType: (character: string) => {
      setQuery((typed) => typed + character);
      search.current?.focus();
    },
  };
  const footer = hovered ?? focused;
  // Every tab is as big as the Keyboard tab (design §5.6).
  const panel = { width: PICKER_WIDTH * UNIT, height: PICKER_HEIGHT * UNIT };
  return (
    <div className={styles.picker}>
      <SearchField
        aria-label={messages.format("picker.search")}
        value={query}
        onChange={setQuery}
        onSubmit={() => {
          const first = results.keys[0];
          if (first !== undefined) choose(first);
        }}
        onKeyDown={(event) => {
          if (event.key !== "ArrowDown") {
            event.continuePropagation();
            return;
          }
          // The key with the tab stop, or the list, which then focuses its selected or
          // first key.
          const list = shown.current;
          (list?.querySelector<HTMLElement>('[tabindex="0"]') ?? list)?.focus();
        }}
        className={styles.search}
        // The picker opened to choose a key: typing goes straight to the search.
        // oxlint-disable-next-line jsx-a11y/no-autofocus
        autoFocus
      >
        <Input
          ref={search}
          className={styles.searchInput}
          placeholder={messages.format("picker.search")}
        />
      </SearchField>
      <div className={styles.body}>
        {/* The tabs keep their place while searching, so the picker keeps its size. */}
        <div className={styles.tabs} inert={searching} data-hidden={searching || undefined}>
          <AriaTabs
            selectedKey={tab}
            onSelectionChange={(id) => {
              setTab(TABS.find((each) => each === id) ?? "keyboard");
            }}
            className={styles.tabsInner}
          >
            <TabList aria-label={messages.format("picker.tabs")}>
              {TABS.map((id) => (
                <Tab key={id} id={id}>
                  {messages.format(`picker.tab.${id}`)}
                </Tab>
              ))}
            </TabList>
            {recent.keys.length === 0 ? null : (
              <div className={styles.recent}>
                <span className={styles.recentLabel}>{messages.format("picker.recent")}</span>
                <BoardView board={recent} label={messages.format("picker.recent")} {...handlers} />
              </div>
            )}
            {TABS.map((id) => (
              <AriaTabPanel key={id} id={id} className={styles.panel} style={panel}>
                {id === "more" ? (
                  <p className={styles.note}>{messages.format("picker.more-note")}</p>
                ) : null}
                <BoardView
                  ref={searching ? null : shown}
                  board={all[id]}
                  label={messages.format(`picker.tab.${id}`)}
                  {...handlers}
                />
              </AriaTabPanel>
            ))}
          </AriaTabs>
        </div>
        {searching ? (
          <div className={styles.results}>
            <p className={styles.count} aria-live="polite">
              {found.length === 0
                ? messages.format("picker.none", { query: query.trim() })
                : messages.format("picker.found", { count: found.length })}
            </p>
            {found.length === 0 ? null : (
              <div className={styles.scroll}>
                <BoardView
                  // Other results start again at the first.
                  key={query}
                  ref={shown}
                  board={results}
                  label={messages.format("picker.results")}
                  {...handlers}
                />
              </div>
            )}
          </div>
        ) : null}
      </div>
      <p className={styles.footer}>{footer === null ? " " : described(footer)}</p>
    </div>
  );
}
