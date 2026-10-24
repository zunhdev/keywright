import { type RefObject, useLayoutEffect, useRef, useState } from "react";

import { shownLayer, useStore } from "../../app/store.ts";
import { useMessages } from "../../i18n/messages.ts";
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
 */
export function KeyboardCanvas() {
  const messages = useMessages();
  const viewport = useRef<HTMLDivElement>(null);
  const space = useSize(viewport);
  const session = useStore((store) => store.session);
  const keymap = useStore((store) => store.keymap);
  const layer = useStore(shownLayer);
  const announcement = useStore((store) => store.announcement);
  const editable = session?.state.kind === "ready";
  const drawn = keymap !== null && layer !== null;
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
      ) : null}
      {/* What VoiceOver says when it can, such as the layer just shown (design §12). */}
      <div className={styles.announcement} aria-live="polite">
        {announcement}
      </div>
      {/* A live region (`output` has the status role), so VoiceOver says each new state.
          Over a keyboard that can't be changed, it's the scrim's message. */}
      <output
        className={styles.state}
        data-over={drawn && !editable ? "keyboard" : drawn ? "nothing" : undefined}
      >
        {session ? messages.format(`session.${session.state.kind}`) : ""}
      </output>
    </div>
  );
}
