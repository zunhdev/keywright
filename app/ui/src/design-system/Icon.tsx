import type { LucideIcon } from "lucide-react";

/** The design's icon sizes (§10.3): 16 px in toolbars, lists and menus, 20 px in empty states. */
const SIZES = { small: 16, large: 20 } as const;

/**
 * A Lucide icon at one of the design's sizes, with its 1.75 px stroke. It's decorative:
 * words next to it, or the label of the button it's in, say what it means.
 */
export function Icon({
  icon: Glyph,
  size = "small",
}: {
  icon: LucideIcon;
  size?: keyof typeof SIZES;
}) {
  return <Glyph size={SIZES[size]} strokeWidth={1.75} aria-hidden="true" focusable="false" />;
}
