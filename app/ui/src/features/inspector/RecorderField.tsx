import { useEffect, useState } from "react";
import { VisuallyHidden } from "react-aria-components";

import { activeRecorder, useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { messages as strings, useMessages } from "../../i18n/messages.ts";
import { commands, type Shortcut, type ShortcutWords } from "../../ipc/bindings.ts";
import { useRecorder } from "./recorder.ts";

import styles from "./Inspector.module.css";

/** What the core calls `shortcut`: its keycap label, and the Mac action it is, if any. */
function useShortcutWords(shortcut: Shortcut | null): ShortcutWords | null {
  const [words, setWords] = useState<ShortcutWords | null>(null);
  const key = JSON.stringify(shortcut);
  useEffect(() => {
    let current = true;
    if (shortcut !== null) {
      // Without an answer, the field shows no words, but still records.
      void commands
        .shortcutWords(shortcut)
        .then((answer) => {
          if (current) setWords(answer);
          return answer;
        })
        .catch(() => null);
    }
    return () => {
      current = false;
    };
    // The shortcut, by value.
    // oxlint-disable-next-line react/exhaustive-deps
  }, [key]);
  return shortcut === null ? null : words;
}

/**
 * A shortcut and the Mac action it is, as `words` give them: `⇧⌘4 · Screenshot of an
 * area` to see, or `Shift-Command-4, Screenshot of an area` for VoiceOver.
 */
function withPreset(words: ShortcutWords, spoken = false): string {
  if (words.preset === null) return spoken ? words.spoken : words.label;
  return spoken ? `${words.spoken}, ${words.preset}` : `${words.label} · ${words.preset}`;
}

/**
 * Has VoiceOver say what was recorded, in words (design §12), through the window's
 * announcements: the field itself isn't a live region, or each key selected would be said.
 */
function announceRecorded(shortcut: Shortcut): void {
  void commands
    .shortcutWords(shortcut)
    .then((words) => {
      const recorded = strings().format("recorder.recorded", {
        shortcut: withPreset(words, true),
      });
      useStore.setState({ announcement: recorded });
      return words;
    })
    .catch(() => null);
}

/**
 * The key or shortcut, as the keycap shows it, with the Mac action it is, and Record
 * (design §5.3): the next shortcut pressed, by physical key, becomes the value. While it
 * records, the field says so; Esc cancels. VoiceOver hears it in words rather than
 * symbols, and hears when recording starts and what was recorded.
 */
export function RecorderField({
  value,
  recorder,
  onRecord,
}: {
  value: Shortcut | null;
  /** The recorder's name in the store, which says which recorder records. */
  recorder: string;
  onRecord: (shortcut: Shortcut) => void;
}) {
  const messages = useMessages();
  const recording = useStore((store) => activeRecorder(store) === recorder);
  const codes = useStore((store) => store.domCodes);
  const words = useShortcutWords(value);
  useRecorder({
    active: recording,
    codes,
    onRecord: (shortcut) => {
      useStore.getState().stopRecording();
      onRecord(shortcut);
      announceRecorded(shortcut);
    },
    onCancel: () => {
      useStore.getState().stopRecording();
    },
  });
  useEffect(() => {
    if (recording) useStore.setState({ announcement: strings().format("recorder.press-spoken") });
  }, [recording]);
  // Until the core's words come, a key shows nothing rather than "No key yet".
  const none = messages.format("recorder.none");
  const shown = value === null ? none : words === null ? "" : withPreset(words);
  const spoken = value === null ? none : words === null ? "" : withPreset(words, true);
  return (
    <div className={styles.fields}>
      <div className={styles.recorder}>
        <div className={styles.recorded} data-recording={recording || undefined}>
          {/* One line, which a long shortcut's words end in an ellipsis to keep. While
              recording, Cancel beside it is the way out, and Esc presses it, as on any Mac
              Cancel button; VoiceOver hears so. */}
          <span aria-hidden="true" className={styles.shown} title={recording ? undefined : shown}>
            {recording ? messages.format("recorder.press") : shown}
          </span>
          <VisuallyHidden>
            {recording ? messages.format("recorder.press-spoken") : spoken}
          </VisuallyHidden>
        </div>
        {/* While recording, the keys go to the recorder, so only a click can cancel here. */}
        <Button
          size="compact"
          onPress={() => {
            const store = useStore.getState();
            if (recording) store.stopRecording();
            else store.startRecording(recorder);
          }}
        >
          {messages.format(recording ? "recorder.stop" : "recorder.record")}
        </Button>
      </div>
      <p className={styles.note}>{messages.format("recorder.hint")}</p>
    </div>
  );
}
