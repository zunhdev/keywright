import type { CSSProperties } from "react";

import type { PhysicalKey } from "../../ipc/bindings.ts";
import { bounds, place } from "../keyboard-canvas/geometry.ts";

import styles from "./Connection.module.css";

/** No keys pointed out. */
const NO_MARKS: ReadonlyMap<number, number> = new Map();

/**
 * A keyboard drawn in outline from its layout (design §7): as an illustration with no
 * keyboard, as a skeleton while connecting, and with its unlock keys numbered. `width` is
 * the most it takes, in px; keys are at most 28 px.
 */
export function BoardOutline({
  keys,
  width,
  marks = NO_MARKS,
  skeleton = false,
}: {
  keys: readonly PhysicalKey[];
  width: number;
  /** Keys to point out, by key position, with their step's number. */
  marks?: ReadonlyMap<number, number>;
  /** Whether it's a placeholder while the keyboard connects, which shimmers. */
  skeleton?: boolean;
}) {
  const extent = bounds(keys);
  const size = Math.min(28, extent.width > 0 ? width / extent.width : 28);
  return (
    <div
      className={styles.board}
      data-skeleton={skeleton || undefined}
      style={{ width: extent.width * size, height: extent.height * size }}
      aria-hidden="true"
    >
      {keys.map((key, position) => {
        const box = place(key, extent, size);
        const mark = marks.get(position);
        const style: CSSProperties = {
          left: box.left,
          top: box.top,
          width: box.width,
          height: box.height,
          transform: box.rotation === 0 ? "none" : `rotate(${box.rotation}deg)`,
          transformOrigin: `${box.originX}px ${box.originY}px`,
        };
        return (
          <span
            // A key's position is what it is: the layout lists keys by it.
            // oxlint-disable-next-line react/no-array-index-key
            key={position}
            className={styles.key}
            data-marked={mark !== undefined || undefined}
            style={style}
          >
            {mark === undefined ? null : <span className={styles.mark}>{mark}</span>}
          </span>
        );
      })}
    </div>
  );
}
