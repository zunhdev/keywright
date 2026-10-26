import { useEffect } from "react";

import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { CoachMark } from "../../design-system/CoachMark.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { isCovered, passStep, SAVE_BUTTON, tabKey } from "./first-run.ts";

/** Skip, on the first coach mark: no shortcut is waited for any more, then. */
function skip(): void {
  useStore.getState().stopRecording();
  passStep();
}

/**
 * The first run's Try it (design §8): a coach mark on the Tab key, "Click a key, then
 * press any shortcut to put it there.", where a click on a key records a shortcut for it;
 * after the first change, one on Save. They show while the keyboard can be edited and no
 * sheet covers the window, and VoiceOver says them as they come.
 */
export function TryIt() {
  const messages = useMessages();
  const step = useStore((store) => store.firstRun?.step);
  const shown = useStore(
    (store) =>
      (store.firstRun?.step === "try-key" || store.firstRun?.step === "try-save") &&
      store.session?.state.kind === "ready" &&
      store.keymap !== null &&
      !isCovered(store),
  );
  const keymap = useStore((store) => store.keymap);
  // What moves the key or Save, so the coach mark is placed again.
  const zoom = useStore((store) => store.zoom);
  const panels = useStore((store) => store.panels);
  const paneWidths = useStore((store) => store.paneWidths);
  const layout = `${zoom}:${panels.layers}:${panels.inspector}:${paneWidths.layers}:${paneWidths.inspector}`;
  const text = messages.format(step === "try-save" ? "first-run.try-save" : "first-run.try-key");

  useEffect(() => {
    if (shown) useStore.setState({ announcement: text });
  }, [shown, text]);

  if (!shown || keymap === null) return null;
  return step === "try-save" ? (
    <CoachMark
      key={layout}
      target={`#${SAVE_BUTTON}`}
      placement="bottom end"
      action={
        <Button size="compact" onPress={() => passStep({ done: true })}>
          {messages.format("first-run.ok")}
        </Button>
      }
    >
      {text}
    </CoachMark>
  ) : (
    <CoachMark
      key={layout}
      target={`[data-position="${tabKey(keymap)}"]`}
      action={
        <Button size="compact" onPress={skip}>
          {messages.format("first-run.skip")}
        </Button>
      }
    >
      {text}
    </CoachMark>
  );
}
