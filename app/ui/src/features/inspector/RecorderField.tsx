import { useEffect, useState } from "react";

import { activeRecorder, useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { useMessages } from "../../i18n/messages.ts";
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
 * The key or shortcut, as the keycap shows it, with the Mac action it is, and Record
 * (design §5.3): the next shortcut pressed, by physical key, becomes the value. While it
 * records, the field says so; Esc cancels.
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
    },
    onCancel: () => {
      useStore.getState().stopRecording();
    },
  });
  // Until the core's words come, a key shows nothing rather than "No key yet".
  const shown =
    value === null
      ? messages.format("recorder.none")
      : words === null
        ? ""
        : words.preset === null
          ? words.label
          : `${words.label} · ${words.preset}`;
  return (
    <div className={styles.fields}>
      <div className={styles.recorder}>
        {/* VoiceOver hears when recording starts, and what was recorded. */}
        <div className={styles.recorded} data-recording={recording || undefined} aria-live="polite">
          {recording ? (
            <>
              {messages.format("recorder.press")}{" "}
              <span className={styles.description}>{messages.format("recorder.cancel")}</span>
            </>
          ) : (
            shown
          )}
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
