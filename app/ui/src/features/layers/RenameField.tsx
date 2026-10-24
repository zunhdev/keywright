import { type KeyboardEvent, useEffect, useId, useRef, useState } from "react";
import { Input, Label, Text, TextField } from "react-aria-components";

import { useMessages } from "../../i18n/messages.ts";
import { commands, type NameProblem } from "../../ipc/bindings.ts";
import { renameLayer } from "./actions.ts";

import styles from "./Layers.module.css";

/** A name's length as the keyboard counts it: in bytes of UTF-8. */
const bytes = (name: string) => new TextEncoder().encode(name).length;

/** What's wrong with a name for the keyboard, as the Rust side says, while it's typed. */
function useNameProblem(name: string): NameProblem | null {
  const [problem, setProblem] = useState<NameProblem | null>(null);
  useEffect(() => {
    let current = true;
    void commands
      .layerNameProblem(name)
      .then((answer) => {
        if (current) setProblem(answer);
        return answer;
      })
      // Without an answer, the keyboard's refusal still says so when renaming.
      .catch(() => null);
    return () => {
      current = false;
    };
  }, [name]);
  return problem;
}

/**
 * A layer's name to edit (LAY-1, design §5.1), with how much of the keyboard's room it
 * takes, such as 5/18, and what's wrong with it, if anything. ↩ or leaving the field
 * renames the layer; Esc puts the name back. `onDone` runs after either, as the sidebar's
 * inline field closes then. `focusOnOpen` gives it focus as it opens, to type in.
 */
export function RenameField({
  id,
  name,
  max,
  label,
  focusOnOpen = false,
  onDone,
}: {
  id: number;
  name: string;
  max: number;
  label?: string;
  focusOnOpen?: boolean;
  onDone?: () => void;
}) {
  const messages = useMessages();
  const [value, setValue] = useState(name);
  const problem = useNameProblem(value);
  const counter = useId();
  const input = useRef<HTMLInputElement>(null);
  // A field that opens to be typed in takes focus on the next frame: a menu that opened it
  // gives focus back as it closes, which passes through here. Until then, losing focus
  // doesn't close it.
  const ready = useRef(!focusOnOpen);
  useEffect(() => {
    const frame = focusOnOpen
      ? requestAnimationFrame(() => {
          ready.current = true;
          input.current?.focus();
        })
      : null;
    return () => {
      if (frame !== null) cancelAnimationFrame(frame);
    };
  }, [focusOnOpen]);
  const commit = () => {
    if (problem === null && value !== name) void renameLayer(id, value);
    else setValue(name);
    onDone?.();
  };
  const onKeyDown = (event: KeyboardEvent) => {
    // The sidebar's list mustn't take the keys typed here.
    event.stopPropagation();
    if (event.key === "Enter") commit();
    if (event.key === "Escape") {
      setValue(name);
      onDone?.();
    }
  };
  return (
    <TextField
      className={styles.rename}
      value={value}
      onChange={setValue}
      {...(label === undefined ? { "aria-label": messages.format("rename.label") } : {})}
      isInvalid={problem !== null}
      onBlur={() => {
        if (ready.current) commit();
      }}
      onKeyDown={onKeyDown}
    >
      {label === undefined ? null : <Label className={styles.renameLabel}>{label}</Label>}
      <span className={styles.renameBox}>
        <Input ref={input} className={styles.renameInput} aria-describedby={counter} />
        <span id={counter} className={styles.count} data-over={bytes(value) > max || undefined}>
          {messages.format("rename.count", { count: bytes(value), max })}
        </span>
      </span>
      {problem === null ? null : (
        <Text slot="errorMessage" className={styles.problem}>
          {problem.kind === "too-long"
            ? messages.format("rename.too-long", { max: problem.max })
            : messages.format("rename.character", { character: problem.character })}
        </Text>
      )}
    </TextField>
  );
}
