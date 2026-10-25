import { Bell, Keyboard, Layers, Lock, Pencil, Search, Trash2, Usb } from "lucide-react";
import { useState } from "react";
import { DialogTrigger, I18nProvider, type Key } from "react-aria-components";

import { Button } from "../design-system/Button.tsx";
import { Icon } from "../design-system/Icon.tsx";
import { List, ListItem } from "../design-system/List.tsx";
import { MenuButton, MenuItem, MenuSeparator } from "../design-system/Menu.tsx";
import { Popover } from "../design-system/Popover.tsx";
import { Segment, SegmentedControl } from "../design-system/SegmentedControl.tsx";
import { Sheet } from "../design-system/Sheet.tsx";
import { ToastRegion, toasts } from "../design-system/Toast.tsx";
import { Toolbar, ToolbarSeparator } from "../design-system/Toolbar.tsx";
import { LOCALE } from "../i18n/messages.ts";
import { Controls } from "./Controls.tsx";
import { Connection, Keycaps, Pickers, Sofle } from "./Keyboards.tsx";

import styles from "./Gallery.module.css";

const COLOURS = [
  "bg-window",
  "bg-panel",
  "bg-canvas",
  "surface-key",
  "surface-hover",
  "surface-pressed",
  "border-subtle",
  "border-key",
  "text-primary",
  "text-secondary",
  "accent",
  "accent-hover",
  "accent-subtle",
  "accent-contrast",
  "warning",
  "danger",
  "success",
];

const TYPE = ["display", "sentence", "title", "body-strong", "body", "caption"];

/** The symbols keycaps show (design §10.1), to check at every key size. */
const SYMBOLS = ["⌃", "⌥", "⇧", "⌘", "⎋", "⇥", "↩", "⌫", "⌦", "␣"];

/** Key sizes, in px: the smallest, between, the default, and zoomed in (design §4.1). */
const KEY_SIZES = [36, 44, 56, 72];

/** Applies an appearance to the whole page, as Settings › Appearance will. */
function useAppearance(): [Key, (appearance: Key) => void] {
  const [appearance, setAppearance] = useState<Key>("system");
  const choose = (next: Key) => {
    setAppearance(next);
    if (next === "system") delete document.documentElement.dataset["theme"];
    else document.documentElement.dataset["theme"] = String(next);
  };
  return [appearance, choose];
}

/** Every token and primitive of the design system, on one page. */
export function Gallery() {
  const [appearance, setAppearance] = useAppearance();
  return (
    <I18nProvider locale={LOCALE}>
      <main className={styles.gallery}>
        <header className={styles.header}>
          <h1>Keywright design system</h1>
          <SegmentedControl
            aria-label="Appearance"
            selectedKeys={[appearance]}
            onSelectionChange={(keys) => {
              const [next] = keys;
              if (next !== undefined) setAppearance(next);
            }}
          >
            <Segment id="system">System</Segment>
            <Segment id="light">Light</Segment>
            <Segment id="dark">Dark</Segment>
          </SegmentedControl>
          <p>
            For Increase Contrast and Reduce Motion, turn them on in System Settings › Accessibility
            › Display: the page follows them.
          </p>
        </header>

        <section className={styles.section}>
          <h2>Colour</h2>
          <div className={styles.swatches}>
            {COLOURS.map((name) => (
              <div key={name} className={styles.swatch}>
                <span className={styles.chip} style={{ background: `var(--${name})` }} />
                {name}
              </div>
            ))}
          </div>
        </section>

        <section className={styles.section}>
          <h2>Layers</h2>
          <div className={styles.swatches}>
            {Array.from({ length: 8 }, (_, index) => (
              <div key={index} className={styles.swatch}>
                <span
                  className={styles.layerSquare}
                  style={{ background: `var(--layer-${index + 1})` }}
                />
                Layer {index + 1}
              </div>
            ))}
          </div>
        </section>

        <section className={styles.section}>
          <h2>Type</h2>
          {TYPE.map((name) => (
            <div key={name} style={{ font: `var(--type-${name})` }}>
              {name}: Tap for Esc, hold for ⌃ · 1234567890
            </div>
          ))}
          <div className={styles.mono}>JetBrains Mono, for ZMK names: &amp;kp LG(C)</div>
        </section>

        <section className={styles.section}>
          <h2>Symbols on keys</h2>
          <div className={styles.canvas}>
            {KEY_SIZES.map((size) => (
              <div key={size} className={styles.row}>
                {SYMBOLS.map((symbol) => (
                  <div
                    key={symbol}
                    className={styles.key}
                    style={{ width: size, height: size, fontSize: size * 0.4 }}
                  >
                    {symbol}
                  </div>
                ))}
                <div
                  className={styles.key}
                  style={{ width: size * 2, height: size, fontSize: size * 0.22 }}
                >
                  ⇧⌘4
                </div>
                <div
                  className={`${styles.key} ${styles.ghost}`}
                  style={{ width: size, height: size, fontSize: size * 0.4 }}
                >
                  q
                </div>
                <span className={styles.keyCaption}>{size} px</span>
              </div>
            ))}
          </div>
        </section>

        <section className={styles.section}>
          <h2>Keycaps</h2>
          <Keycaps />
        </section>

        <section className={styles.section}>
          <h2>Keyboard</h2>
          <Sofle />
        </section>

        <Connection />

        <section className={styles.section}>
          <h2>Key picker</h2>
          <Pickers />
        </section>

        <section className={styles.section}>
          <h2>Icons</h2>
          <div className={styles.row}>
            {[Keyboard, Layers, Lock, Usb, Search, Pencil, Trash2, Bell].map((glyph) => (
              <Icon key={glyph.displayName} icon={glyph} />
            ))}
            {[Keyboard, Usb].map((glyph) => (
              <Icon key={glyph.displayName} icon={glyph} size="large" />
            ))}
          </div>
        </section>

        <section className={styles.section}>
          <h2>Buttons</h2>
          <div className={styles.row}>
            <Button variant="primary">Save</Button>
            <Button>Review</Button>
            <Button variant="quiet">Cancel</Button>
            <Button variant="danger">Discard all…</Button>
            <Button size="compact">Compact</Button>
            <Button isDisabled variant="primary">
              Nothing to save
            </Button>
            <Button variant="quiet" aria-label="Search keys">
              <Icon icon={Search} />
            </Button>
          </div>
        </section>

        <section className={styles.section}>
          <h2>Toolbar and menu</h2>
          <Toolbar aria-label="Editing">
            <Button variant="quiet" size="compact" aria-label="Lock">
              <Icon icon={Lock} />
            </Button>
            <ToolbarSeparator />
            <MenuButton label="Actions" aria-label="Actions">
              <MenuItem shortcut="⌘Z">Undo</MenuItem>
              <MenuItem shortcut="⇧⌘Z">Redo</MenuItem>
              <MenuSeparator />
              <MenuItem shortcut="⌘C">Copy Action</MenuItem>
              <MenuItem isDisabled>Paste Action</MenuItem>
            </MenuButton>
            <Button variant="primary" size="compact">
              Save
            </Button>
          </Toolbar>
        </section>

        <Controls />

        <section className={styles.section}>
          <h2>List</h2>
          <div className={styles.panel}>
            <List aria-label="Layers" selectionMode="single" defaultSelectedKeys={["base"]}>
              {["Base", "Lower", "Raise", "Adjust"].map((name, index) => (
                <ListItem
                  key={name}
                  id={name.toLowerCase()}
                  textValue={name}
                  actions={
                    <Button variant="quiet" size="compact" aria-label={`Rename ${name}`}>
                      <Icon icon={Pencil} />
                    </Button>
                  }
                >
                  <span
                    className={styles.layerSquare}
                    style={{ background: `var(--layer-${index + 1})` }}
                  />
                  {name}
                </ListItem>
              ))}
            </List>
          </div>
        </section>

        <section className={styles.section}>
          <h2>Popover, sheet and toast</h2>
          <div className={styles.row}>
            <DialogTrigger>
              <Button>USB · Unlocked</Button>
              <Popover label="Connection">
                Connected over USB. Your keyboard locks itself after 10 minutes idle.
              </Popover>
            </DialogTrigger>
            <DialogTrigger>
              <Button>Review changes…</Button>
              <Sheet title="Review changes">
                {(close) => (
                  <>
                    <p>3 changes on Base and Lower.</p>
                    <div className={styles.row}>
                      <Button variant="quiet" onPress={close}>
                        Cancel
                      </Button>
                      <Button variant="primary" onPress={close}>
                        Save to keyboard
                      </Button>
                    </div>
                  </>
                )}
              </Sheet>
            </DialogTrigger>
            <Button
              onPress={() => {
                toasts.add({ title: "Saved to keyboard" }, { timeout: 5000 });
              }}
            >
              Show a toast
            </Button>
          </div>
        </section>
      </main>
      <ToastRegion />
    </I18nProvider>
  );
}
