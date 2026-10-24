import { type RefObject, useLayoutEffect, useRef, useState } from "react";

import { shownLayer, useStore } from "../../app/store.ts";
import { useMessages } from "../../i18n/messages.ts";
import { Connecting } from "../connection/Connecting.tsx";
import { Delayed } from "../connection/Delayed.tsx";
import { NoKeyboard } from "../connection/NoKeyboard.tsx";
import { hasCard, isSilent, silence, StateCard } from "../connection/StateCard.tsx";
import { RelockBanner } from "../connection/UnlockSheet.tsx";
import { Keyboard } from "./Keyboard.tsx";

import styles from "./KeyboardCanvas.module.css";

/** The size of `ref`'s element, in px, as it changes. */
function useSize(ref: RefObject<HTMLElement | null>): { width: number; height: number } {
  const [size, setSize] = useState({ width: 0, height: 0 });
  useLayoutEffect(() => {
    const element = ref.current;
    const measure = () => {
      if (element !== null) setSize({ width: element.clientWidth, height: element.clientHeight });
    };
    const observer = new ResizeObserver(measure);
    if (element !== null) {
      measure();
      observer.observe(element);
    }
    return () => {
      observer.disconnect();
    };
  }, [ref]);
  return size;
}

/**
 * The keyboard canvas (design §4): the shown layer's name, then the keyboard, drawn from
 * its own layout, with what each key does on that layer. Keys are drawn as big as fits,
 * from 36 px up to 56 px, then zoomed. While the keymap can't be changed, such as when the
 * keyboard is locked, the canvas shows under a scrim that says why.
 *
 * Before there's a keymap, it shows the connection's state (design §7): how to connect a
 * keyboard, or a skeleton while one connects. A card says what's wrong and what to do,
 * after a moment for a silent port, and a banner says when the keyboard locked itself.
 */
export function KeyboardCanvas() {
  const messages = useMessages();
  const viewport = useRef<HTMLDivElement>(null);
  const space = useSize(viewport);
  const session = useStore((store) => store.session);
  const keymap = useStore((store) => store.keymap);
  const layer = useStore(shownLayer);
  const announcement = useStore((store) => store.announcement);
  const relocked = useStore((store) => store.relocked);
  const editable = session?.state.kind === "ready";
  const drawn = keymap !== null && layer !== null;
  const kind = session?.state.kind ?? "no-device";
  const card = session !== null && hasCard(session);
  // A port that's silent may be about to answer (design §7): it gets a moment first.
  const silent = isSilent(session);
  return (
    <div ref={viewport} className={styles.viewport}>
      {drawn ? (
        <Keyboard
          keymap={keymap}
          layer={layer}
          editable={editable}
          space={space}
          messages={messages}
        />
      ) : kind === "no-device" ? (
        <NoKeyboard />
      ) : card && !silent ? null : (
        <Connecting />
      )}
      {drawn && relocked && kind === "locked" ? <RelockBanner /> : null}
      {card && silent ? (
        <Delayed key={kind} delay={silence.grace}>
          <StateCard session={session} />
        </Delayed>
      ) : card ? (
        <StateCard session={session} />
      ) : null}
      {/* What VoiceOver says when it can, such as the layer just shown (design §12). */}
      <div className={styles.announcement} aria-live="polite">
        {announcement}
      </div>
      {/* A live region (`output` has the status role), so VoiceOver says each new state.
          Over a keyboard that can't be changed, it's the scrim's message. */}
      <output
        className={styles.state}
        data-over={
          drawn && !editable ? "keyboard" : drawn || kind !== "probing" ? "nothing" : undefined
        }
        data-scrim-only={(drawn && (card || relocked)) || undefined}
      >
        {session ? messages.format(`session.${session.state.kind}`) : ""}
      </output>
    </div>
  );
}
