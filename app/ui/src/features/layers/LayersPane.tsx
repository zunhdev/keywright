import { GripVertical, Minus, Plus } from "lucide-react";
import { type MouseEvent, useState } from "react";
import { Button as AriaButton, Meter, useDragAndDrop, VisuallyHidden } from "react-aria-components";

import { shownLayer, useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Icon } from "../../design-system/Icon.tsx";
import { List, ListItem } from "../../design-system/List.tsx";
import { ContextMenu, MenuItem } from "../../design-system/Menu.tsx";
import { useMessages } from "../../i18n/messages.ts";
import type { Keymap, Layer } from "../../ipc/bindings.ts";
import { switchLayer } from "../keyboard-canvas/edits.ts";
import { LayerName } from "../keyboard-canvas/Keycap.tsx";
import { findKeys } from "../search/search.ts";
import { addLayer, moveLayer, removeLayer } from "./actions.ts";
import { BASE, dropIndex, turnedOnByKeys } from "./order.ts";
import { RemoveLayerSheet } from "./RemoveLayerSheet.tsx";
import { RenameField } from "./RenameField.tsx";

import styles from "./Layers.module.css";

/** The layers before there's a keymap: the same empty list each time, for the store. */
const NO_LAYERS: readonly Layer[] = [];

/** Runs `then` two frames from now, once a closing menu has given focus back. */
function afterFocusSettles(then: () => void): void {
  requestAnimationFrame(() => {
    requestAnimationFrame(then);
  });
}

/**
 * The layers sidebar (design §3.1, LAY-1–LAY-6, VIEW-6): each layer with its colour, name,
 * how it's reached, a dot if it's changed, and while searching, how many keys it found.
 * Choosing one shows it; double-clicking or ↩ renames it in place; dragging reorders,
 * and its right-click menu renames, restores or removes it. Below, how many layers the
 * firmware has room for, with Add and Remove.
 */
export function LayersPane() {
  const messages = useMessages();
  const keymap = useStore((store) => store.keymap);
  const layers = keymap?.layers ?? NO_LAYERS;
  const shown = useStore(shownLayer);
  const editable = useStore((store) => store.session?.state.kind === "ready");
  const search = useStore((store) => store.search);
  const macActions = useStore((store) => store.macActions);
  const found =
    keymap === null || search.trim() === "" ? null : findKeys(keymap, search, macActions);
  const [renaming, setRenaming] = useState<number | null>(null);
  const [menu, setMenu] = useState<{ x: number; y: number; layer: Layer } | null>(null);
  const [removing, setRemoving] = useState<Layer | null>(null);
  const { dragAndDropHooks } = useDragAndDrop({
    // Not while locked, nor while a name is being typed.
    isDisabled: !editable || renaming !== null,
    getItems: (keys) => [...keys].map((key) => ({ "text/plain": String(key) })),
    onReorder: (event) => {
      const [moved] = event.keys;
      const to = dropIndex(
        layers,
        Number(moved),
        Number(event.target.key),
        event.target.dropPosition,
      );
      if (to !== null) void moveLayer(Number(moved), to);
    },
  });
  const remove = (layer: Layer) => {
    if (turnedOnByKeys(layer)) setRemoving(layer);
    else void removeLayer(layer.id);
  };
  const onContextMenu = (event: MouseEvent, layer: Layer) => {
    event.preventDefault();
    // The row takes focus, as a right-clicked row does on a Mac, so the menu gives it back
    // there, and renaming starts from it.
    const target = event.target instanceof Element ? event.target.closest("[role='row']") : null;
    if (target instanceof HTMLElement) target.focus();
    switchLayer(layer.id);
    setMenu({ x: event.clientX, y: event.clientY, layer });
  };

  return (
    <>
      <h2 className={styles.caption}>{messages.format("layers.title")}</h2>
      <List
        aria-label={messages.format("layers.title")}
        items={layers}
        // The rows show the search's counts and the field being renamed, which aren't in
        // the layers themselves.
        dependencies={[found, renaming, editable]}
        selectionMode="single"
        selectionBehavior="replace"
        disallowEmptySelection
        selectedKeys={shown === null ? [] : [shown.id]}
        onSelectionChange={(keys) => {
          const [id] = keys;
          if (typeof id === "number") switchLayer(id);
        }}
        onAction={(id) => {
          if (editable && typeof id === "number") setRenaming(id);
        }}
        dragAndDropHooks={dragAndDropHooks}
        renderEmptyState={() => <p className={styles.empty}>{messages.format("layers.empty")}</p>}
      >
        {(layer) => {
          const count = found?.get(layer.id)?.length;
          const [way] = layer.reach;
          return (
            <ListItem id={layer.id} textValue={layer.name}>
              {/* Dragging reorders; VoiceOver and the keyboard drag with this button. */}
              <AriaButton
                slot="drag"
                className={styles.drag}
                aria-label={messages.format("layers.move", { name: layer.name })}
              >
                <Icon icon={GripVertical} />
              </AriaButton>
              <span
                className={styles.row}
                onContextMenu={(event) => {
                  onContextMenu(event, layer);
                }}
              >
                <span className={styles.top}>
                  {renaming === layer.id && keymap !== null ? (
                    <RenameField
                      id={layer.id}
                      name={layer.name}
                      max={keymap.layerNameMax}
                      focusOnOpen
                      onDone={() => {
                        setRenaming(null);
                      }}
                    />
                  ) : (
                    <LayerName id={layer.id} name={layer.name} />
                  )}
                  {layer.changed ? (
                    <>
                      <span className={styles.changed} aria-hidden="true" />
                      <VisuallyHidden>{messages.format("layers.changed")}</VisuallyHidden>
                    </>
                  ) : null}
                  {/* While searching, each layer says how many keys it found (design §4.6). */}
                  {count === undefined ? null : (
                    <span
                      className={styles.matches}
                      data-none={count === 0 || undefined}
                      aria-label={messages.format("layers.matches", { count })}
                    >
                      {count}
                    </span>
                  )}
                </span>
                {way === undefined ? null : (
                  <span className={styles.reach} title={way.text}>
                    {way.text}
                  </span>
                )}
              </span>
            </ListItem>
          );
        }}
      </List>
      {keymap === null ? null : (
        <Capacity keymap={keymap} editable={editable} shown={shown} onRemove={remove} />
      )}
      {menu === null ? null : (
        <ContextMenu
          aria-label={messages.format("layers.menu", { name: menu.layer.name })}
          point={menu}
          onClose={() => {
            setMenu(null);
          }}
          onAction={(item) => {
            // Renaming starts once the menu has given focus back, so its field keeps it.
            if (item === "rename") afterFocusSettles(() => setRenaming(menu.layer.id));
            if (item === "remove") remove(menu.layer);
          }}
        >
          <MenuItem id="rename" isDisabled={!editable}>
            {messages.format("layers.rename")}
          </MenuItem>
          {/* Restoring a stock layer comes with stock keys (KEY-52). */}
          <MenuItem id="restore" isDisabled>
            {messages.format("layers.restore")}
          </MenuItem>
          <MenuItem id="remove" isDisabled={!editable || menu.layer.id === BASE}>
            {messages.format("layers.remove")}
          </MenuItem>
        </ContextMenu>
      )}
      {removing === null ? null : (
        <RemoveLayerSheet
          layer={removing}
          onClose={() => {
            setRemoving(null);
          }}
        />
      )}
    </>
  );
}

/**
 * How many layers the firmware has room for, and how many are used (LAY-3), with Add and
 * Remove (the layer shown). With no room left, it says why, and what would change that.
 * The order's meaning comes last: a layer further down takes priority.
 */
function Capacity({
  keymap,
  editable,
  shown,
  onRemove,
}: {
  keymap: Keymap;
  editable: boolean;
  shown: Layer | null;
  onRemove: (layer: Layer) => void;
}) {
  const messages = useMessages();
  const used = keymap.layers.length;
  const total = used + keymap.freeLayerSlots;
  return (
    <footer className={styles.footer}>
      <Meter
        className={styles.meter}
        aria-label={messages.format("layers.used")}
        value={used}
        maxValue={total}
        valueLabel={messages.format("layers.capacity", { used, total })}
      >
        {({ valueText, percentage }) => (
          <>
            <span className={styles.meterText}>{valueText}</span>
            <span className={styles.meterBar}>
              <span className={styles.meterFill} style={{ width: `${percentage}%` }} />
            </span>
          </>
        )}
      </Meter>
      <div className={styles.buttons}>
        <Button
          variant="quiet"
          size="compact"
          aria-label={messages.format("layers.add")}
          isDisabled={!editable || keymap.freeLayerSlots === 0}
          onPress={() => void addLayer()}
        >
          <Icon icon={Plus} />
        </Button>
        <Button
          variant="quiet"
          size="compact"
          aria-label={messages.format("layers.remove-shown", { name: shown?.name ?? "" })}
          isDisabled={!editable || shown === null || shown.id === BASE || used <= 1}
          onPress={() => {
            if (shown !== null) onRemove(shown);
          }}
        >
          <Icon icon={Minus} />
        </Button>
      </div>
      {keymap.freeLayerSlots === 0 ? (
        <p className={styles.note}>{messages.format("layers.full", { total })}</p>
      ) : null}
      <p className={styles.note}>{messages.format("layers.priority")}</p>
    </footer>
  );
}
