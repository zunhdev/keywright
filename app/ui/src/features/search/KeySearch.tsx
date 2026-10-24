import { Search, X } from "lucide-react";
import type { KeyboardEvent } from "react";
import { Button, Input, SearchField } from "react-aria-components";

import { shownLayer, useStore } from "../../app/store.ts";
import { Icon } from "../../design-system/Icon.tsx";
import { messages as strings, useMessages } from "../../i18n/messages.ts";
import { switchLayer } from "../keyboard-canvas/edits.ts";
import { searchField } from "./focus.ts";
import { findKeys, stepMatch } from "./search.ts";

import styles from "./KeySearch.module.css";

/**
 * Goes to the next match, or the previous one, showing its layer and selecting it, and
 * tells VoiceOver which it is (↩, ⇧↩).
 */
function jump(direction: 1 | -1): void {
  const store = useStore.getState();
  const { keymap, search, macActions, selection } = store;
  const layer = shownLayer(store);
  if (keymap === null || layer === null) return;
  const found = findKeys(keymap, search, macActions);
  const next = stepMatch(
    keymap,
    found,
    { layer: layer.id, position: selection[0] ?? null },
    direction,
  );
  if (next === null) return;
  if (next.layer !== layer.id) switchLayer(next.layer);
  useStore.getState().select([next.position]);
  const all = keymap.layers.flatMap((each) =>
    (found.get(each.id) ?? []).map((position) => ({ layer: each.id, position })),
  );
  const shown = keymap.layers.find((each) => each.id === next.layer);
  useStore.setState({
    announcement: strings().format("search.match", {
      index:
        all.findIndex((place) => place.layer === next.layer && place.position === next.position) +
        1,
      count: all.length,
      place: keymap.keys[next.position]?.place ?? "",
      layer: shown?.name ?? "",
      spoken: shown?.keys[next.position]?.spoken ?? "",
    }),
  });
}

/** ↩ goes to the next match, and ⇧↩ to the previous one. */
function jumpOnReturn(event: KeyboardEvent): void {
  if (event.key !== "Enter") return;
  event.preventDefault();
  jump(event.shiftKey ? -1 : 1);
}

/**
 * The toolbar's search (design §4.6, VIEW-5): it finds keys across all layers, by what they
 * do, their keycap, their names, or the Mac action they are. The canvas dims the others,
 * and the layers count each one's matches. ↩ and ⇧↩ go to the next and previous match;
 * Esc clears it; Find (⌘F) comes here.
 */
export function KeySearch() {
  const messages = useMessages();
  const search = useStore((store) => store.search);
  const keymap = useStore((store) => store.keymap);
  const macActions = useStore((store) => store.macActions);
  const count =
    keymap === null || search.trim() === ""
      ? null
      : [...findKeys(keymap, search, macActions).values()].reduce(
          (total, positions) => total + positions.length,
          0,
        );
  return (
    <SearchField
      aria-label={messages.format("search.label")}
      className={styles.search}
      value={search}
      onChange={(value) => {
        useStore.setState({ search: value });
      }}
      isDisabled={keymap === null}
      onKeyDown={jumpOnReturn}
    >
      <span className={styles.icon}>
        <Icon icon={Search} />
      </span>
      <Input
        ref={searchField}
        className={styles.input}
        placeholder={messages.format("search.placeholder")}
      />
      {count === null ? (
        <kbd className={styles.shortcut} aria-hidden>
          ⌘F
        </kbd>
      ) : (
        <>
          <span className={styles.count} aria-live="polite">
            {messages.format("search.count", { count })}
          </span>
          <Button className={styles.clear} aria-label={messages.format("search.clear")}>
            <X size={12} strokeWidth={2} aria-hidden="true" focusable="false" />
          </Button>
        </>
      )}
    </SearchField>
  );
}
