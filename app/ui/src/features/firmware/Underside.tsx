import { useEffect, useRef } from "react";

import type { HalfName } from "../../ipc/bindings.ts";

import styles from "./Firmware.module.css";

/**
 * The keyboard from below (design §9.3), from its support pack's SVG drawing, with `half`'s
 * reset button and USB port highlighted and the other half dimmed. The drawing's own
 * `<style>` is left out, since the window's Content Security Policy refuses inline styles:
 * the module's CSS draws its lines instead. Its title and description stay, for VoiceOver.
 */
export function Underside({ svg, half }: { svg: string; half: HalfName }) {
  const container = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const parent = container.current;
    if (parent === null) return;
    const drawing = new DOMParser().parseFromString(svg, "image/svg+xml").documentElement;
    if (drawing.nodeName !== "svg") return;
    for (const style of drawing.querySelectorAll("style")) style.remove();
    parent.replaceChildren(document.importNode(drawing, true));
    const mark = (id: string, as: "highlighted" | "dimmed") => {
      const element = parent.querySelector(`[id="${id}"]`);
      if (element instanceof SVGElement) element.dataset[as] = "";
    };
    mark(`reset-button-${half}`, "highlighted");
    mark(`usb-port-${half}`, "highlighted");
    mark(`${half === "left" ? "right" : "left"}-half`, "dimmed");
  }, [svg, half]);
  return <div ref={container} className={styles.underside} />;
}
