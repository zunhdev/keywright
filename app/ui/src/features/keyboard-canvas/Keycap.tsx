import type { CSSProperties } from "react";
import { Button } from "react-aria-components";

import { layerColour } from "../../design-system/layers.ts";
import { TooltipTrigger } from "../../design-system/Tooltip.tsx";
import { type Messages, useMessages } from "../../i18n/messages.ts";
import type { Key, LayerMode } from "../../ipc/bindings.ts";
import { fitSize, layoutLabel } from "./label.ts";

import styles from "./Keycap.module.css";

/** How a key stands out from the others (design §4.3), besides hovering. */
export interface KeycapState {
  /** Selected alone, or with other keys. */
  readonly selected?: "single" | "multiple";
  /** Something dragged over it would swap with it, or, with ⌥, be copied onto it. */
  readonly dropTarget?: "swap" | "copy";
  /** While searching: whether it matches, or is dimmed because it doesn't. */
  readonly search?: "match" | "no-match";
}

export interface KeycapProps extends KeycapState {
  /** What the key shows and does on the layer drawn. */
  readonly face: Key;
  /** The key's box, in px: its slot on the keyboard, gap included. */
  readonly width: number;
  readonly height: number;
  /** The key size, 1u, in px. */
  readonly unit: number;
  /** Whether it's a knob's press, which is drawn round. */
  readonly round?: boolean;
  /** Whether Tab reaches it: one key on the canvas is the canvas's tab stop. */
  readonly tabbable?: boolean;
  /** Where it sits, rotated, on the canvas. */
  readonly style?: CSSProperties;
  readonly onPress?: () => void;
}

/** How many characters a layer's colour square and its gap take, about. */
const SQUARE = 1.5;

/** The gap between neighbouring keys, as a share of 1u. */
const GAP = 0.05;

/** A knob's press, inside its ring: how far in from its slot, as a share of 1u. */
const KNOB_INSET = 0.14;

/** How much of a round keycap's width a line of text can use, across its middle. */
const ROUND_TEXT = 0.85;

/** The padding inside a keycap, as a share of 1u. */
const PADDING = 0.08;

/**
 * A key on the canvas (design §4.2, §4.3): a button whose keycap shows what the key does,
 * and whose tooltip says it in a sentence. Badges mark a key that's changed, one that
 * doesn't work on a Mac, a "same as layer below" key and an action Keywright doesn't know.
 */
export function Keycap({
  face,
  width,
  height,
  unit,
  round = false,
  tabbable = false,
  selected,
  dropTarget,
  search,
  style,
  onPress,
}: KeycapProps) {
  const messages = useMessages();
  // Rounded, so a key's edges land on whole tenths of a pixel.
  const gap = Math.round((round ? KNOB_INSET : GAP) * unit * 10) / 10;
  const cap = { width: width - 2 * gap, height: height - 2 * gap };
  const textWidth = (cap.width - 2 * PADDING * unit) * (round ? ROUND_TEXT : 1);
  const badge = face.inherited
    ? "▽"
    : face.macWarning
      ? "⚠"
      : face.keycap.kind === "other"
        ? "⋯"
        : null;
  return (
    <TooltipTrigger tip={face.sentence}>
      <Button
        className={styles.key}
        style={
          { ...style, width, height, "--gap": `${gap}px`, "--unit": `${unit}px` } as CSSProperties
        }
        aria-label={face.sentence}
        excludeFromTabOrder={!tabbable}
        data-round={round || undefined}
        data-selected={selected}
        data-drop-target={dropTarget}
        data-search={search}
        {...(onPress === undefined ? {} : { onPress })}
      >
        <span className={styles.cap} data-kind={face.keycap.kind} aria-hidden="true">
          {dropTarget === undefined ? (
            <span className={styles.face} data-inherited={face.inherited || undefined}>
              <Face
                face={face}
                width={textWidth}
                height={cap.height}
                unit={unit}
                messages={messages}
              />
            </span>
          ) : (
            <span className={styles.preview}>{messages.format(`keycap.drop.${dropTarget}`)}</span>
          )}
          {badge === null ? null : (
            <span className={styles.badge} data-corner="top-left" data-badge={badge}>
              {badge}
            </span>
          )}
          {face.changed ? (
            <span className={styles.badge} data-corner="top-right" data-badge="changed">
              •
            </span>
          ) : null}
        </span>
      </Button>
    </TooltipTrigger>
  );
}

/** What a keycap shows, for each kind of action. */
function Face({
  face,
  width,
  height,
  unit,
  messages,
}: {
  face: Key;
  width: number;
  height: number;
  unit: number;
  messages: Messages;
}) {
  // Labels scale with the key size, not a tall key's height.
  const reference = Math.min(height, unit);
  const keycap = face.keycap;
  if (keycap.kind === "label") {
    return <Label text={keycap.text} width={width} height={reference} />;
  }
  if (keycap.kind === "tap-hold") {
    const hold = messages.format("keycap.hold");
    const holding = keycap.hold.kind === "key" ? keycap.hold.text : keycap.hold.name;
    const strip = fitSize(
      `${hold} ${holding}`,
      width,
      0.17 * unit,
      keycap.hold.kind === "layer" ? SQUARE : 0,
    );
    return (
      <>
        <Label text={keycap.tap} width={width} height={reference} />
        <span className={styles.strip} style={{ fontSize: strip }}>
          {hold}{" "}
          {keycap.hold.kind === "key" ? (
            keycap.hold.text
          ) : (
            <LayerName id={keycap.hold.layer} name={keycap.hold.name} />
          )}
        </span>
      </>
    );
  }
  if (keycap.kind === "layer") {
    return (
      <>
        <span
          className={styles.layer}
          style={{ fontSize: fitSize(keycap.name, width, 0.2 * reference, SQUARE) }}
        >
          <LayerName id={keycap.layer} name={keycap.name} />
        </span>
        <Mode mode={keycap.mode} messages={messages} />
      </>
    );
  }
  if (keycap.kind === "sticky") {
    return (
      <>
        <Label text={keycap.text} width={width} height={reference} />
        <Mode mode="next-key" messages={messages} />
      </>
    );
  }
  if (keycap.kind === "nothing") {
    return (
      <span className={styles.nothing}>
        <span className={styles.hatch} />
        <span className={styles.dash}>–</span>
      </span>
    );
  }
  // An action Keywright doesn't know: the last kind.
  return <span className={styles.other}>{keycap.name}</span>;
}

/** A label laid out to fit (design §4.2). */
function Label({ text, width, height }: { text: string; width: number; height: number }) {
  const {
    lines: [first, second],
    size,
  } = layoutLabel(text, width, height);
  return (
    <span className={styles.label} style={{ fontSize: size }}>
      <span className={styles.line}>{first}</span>
      {second === undefined ? null : <span className={styles.line}>{second}</span>}
    </span>
  );
}

/** A layer's colour square and name, as on layer keys and in the hold strip (design §4.4). */
export function LayerName({ id, name }: { id: number; name: string }) {
  return (
    <span className={styles.layerName}>
      <span className={styles.square} style={{ background: layerColour(id) }} />
      <span className={styles.line}>{name}</span>
    </span>
  );
}

/** How a layer key turns its layer on, or that a sticky modifier is for the next key. */
function Mode({ mode, messages }: { mode: LayerMode; messages: Messages }) {
  return <span className={styles.mode}>{messages.format(`keycap.mode.${mode}`)}</span>;
}
