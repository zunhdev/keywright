import { ChevronDown } from "lucide-react";
import { useState } from "react";
import {
  Autocomplete,
  Header,
  Input,
  Menu,
  MenuItem,
  MenuSection,
  MenuTrigger,
  Popover,
  SearchField,
  Text,
} from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Icon } from "../../design-system/Icon.tsx";
import { useMessages } from "../../i18n/messages.ts";
import type { Action } from "../../ipc/bindings.ts";
import { CATEGORIES, type CategoryId } from "./categories.ts";
import { sameAction } from "./ChoiceList.tsx";
import { pickerButton } from "./focus.ts";
import { type Recent, recentActions } from "./recents.ts";

import overlay from "../../design-system/overlay.module.css";
import styles from "./Inspector.module.css";

/** How many actions a search lists at most. */
const MATCHES = 20;

/** An action the picker lists, with its words. */
interface Match extends Recent {
  readonly id: string;
}

/** Whether `text` contains `query`, ignoring case. */
const contains = (text: string, query: string) =>
  text.toLocaleLowerCase().includes(query.toLocaleLowerCase());

/**
 * The action picker (design §5.2): a button that names the key's kind of action, and
 * opens a search over a menu. Before anything is typed, the menu lists the actions picked
 * last and the kinds of action; as you type, the kinds and every action and key whose
 * name, or what it does, matches. Choosing a kind shows its editor; choosing an action
 * sets it.
 */
export function ActionPicker({
  category,
  categories,
  onCategory,
  onAction,
}: {
  category: CategoryId;
  categories: readonly CategoryId[];
  onCategory: (category: CategoryId) => void;
  onAction: (action: Action, recent: Recent) => void;
}) {
  const messages = useMessages();
  const current = CATEGORIES.find(([id]) => id === category);
  const label = messages.format(`category.${category}`);
  return (
    <div className={styles.fields}>
      <span className={styles.label}>{messages.format("inspector.picker")}</span>
      <MenuTrigger>
        <Button ref={pickerButton} aria-label={`${messages.format("inspector.picker")}: ${label}`}>
          {current === undefined ? null : <Icon icon={current[1]} />}
          <span className={styles.pickerLabel}>{label}</span>
          <Icon icon={ChevronDown} />
        </Button>
        <Popover className={`${overlay.popover} ${styles.pickerPopover}`} placement="bottom start">
          <PickerMenu categories={categories} onCategory={onCategory} onAction={onAction} />
        </Popover>
      </MenuTrigger>
    </div>
  );
}

/**
 * The picker's search and menu, which exist while it's open: each time it opens, it starts
 * with an empty search and the latest recent actions.
 */
function PickerMenu({
  categories,
  onCategory,
  onAction,
}: {
  categories: readonly CategoryId[];
  onCategory: (category: CategoryId) => void;
  onAction: (action: Action, recent: Recent) => void;
}) {
  const messages = useMessages();
  const [query, setQuery] = useState("");
  const choices = useStore((store) => store.keymap?.choices) ?? [];
  const others = useStore((store) => store.keymap?.others) ?? [];
  const keys = useStore((store) => store.keyCatalog);
  const searching = query.trim() !== "";
  const kinds = CATEGORIES.filter(
    ([id]) =>
      categories.includes(id) && (!searching || contains(messages.format(`category.${id}`), query)),
  );
  const recents = searching ? [] : recentActions();
  const matches: Match[] = searching
    ? [
        ...choices.map((choice, index) => ({
          id: `choice:${index}`,
          label: choice.label,
          sentence: choice.sentence,
          action: choice.action,
        })),
        ...keys.map((key) => ({
          id: `key:${key.page}:${key.id}`,
          label: key.cap === key.name ? key.name : `${key.cap} ${key.name}`,
          sentence: messages.format(`group.${key.group}`),
          action: { kind: "key-press", key: { mods: 0, page: key.page, id: key.id } } as const,
        })),
        ...others.map((other) => ({
          id: `other:${other.id}`,
          label: other.name,
          sentence: messages.format("category.other"),
          action: {
            kind: "other",
            behavior: other.id,
            param1: { kind: "number", value: 0 },
            param2: { kind: "number", value: 0 },
          } as const,
        })),
      ]
        .filter((match) => contains(match.label, query) || contains(match.sentence, query))
        // A media key is both a choice and a key: list it once.
        .filter(
          (match, index, all) =>
            all.findIndex((other) => sameAction(other.action, match.action)) === index,
        )
        .slice(0, MATCHES)
    : [];
  const listed: Match[] = [
    ...recents.map((recent, index) => ({
      id: `recent:${index}`,
      label: recent.label,
      sentence: recent.sentence,
      action: recent.action,
    })),
    ...matches,
  ];
  return (
    <Autocomplete inputValue={query} onInputChange={setQuery}>
      <SearchField
        aria-label={messages.format("inspector.search")}
        className={styles.search}
        // The menu opened to be searched: typing goes straight there.
        // oxlint-disable-next-line jsx-a11y/no-autofocus
        autoFocus
      >
        <Input className={styles.searchInput} placeholder={messages.format("inspector.search")} />
      </SearchField>
      <Menu
        aria-label={messages.format("inspector.picker")}
        className={overlay.collection}
        onAction={(id) => {
          const kind = CATEGORIES.find(([each]) => `kind:${each}` === id)?.[0];
          if (kind !== undefined) {
            onCategory(kind);
            return;
          }
          const match = listed.find((each) => each.id === id);
          if (match !== undefined) onAction(match.action, match);
        }}
      >
        {recents.length === 0 ? null : (
          <MenuSection>
            <Header className={overlay.header}>{messages.format("inspector.recent")}</Header>
            {recents.map((recent, index) => (
              <MatchItem
                key={JSON.stringify(recent.action)}
                id={`recent:${index}`}
                match={recent}
              />
            ))}
          </MenuSection>
        )}
        {kinds.length === 0 ? null : (
          <MenuSection>
            <Header className={overlay.header}>{messages.format("inspector.kinds")}</Header>
            {kinds.map(([id, icon]) => (
              <MenuItem
                key={id}
                id={`kind:${id}`}
                textValue={messages.format(`category.${id}`)}
                className={overlay.item}
              >
                <Icon icon={icon} />
                <Text slot="label">{messages.format(`category.${id}`)}</Text>
              </MenuItem>
            ))}
          </MenuSection>
        )}
        {matches.length === 0 ? null : (
          <MenuSection>
            <Header className={overlay.header}>{messages.format("inspector.matches")}</Header>
            {matches.map((match) => (
              <MatchItem key={match.id} id={match.id} match={match} />
            ))}
          </MenuSection>
        )}
      </Menu>
    </Autocomplete>
  );
}

/** An action in the picker's menu: what it's called, and what it does. */
function MatchItem({ id, match }: { id: string; match: Recent }) {
  return (
    <MenuItem id={id} textValue={match.label} className={overlay.item}>
      <span className={styles.choice}>
        <Text slot="label">{match.label}</Text>
        <Text slot="description" className={overlay.description}>
          {match.sentence}
        </Text>
      </span>
    </MenuItem>
  );
}
