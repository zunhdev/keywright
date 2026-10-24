import { createRef } from "react";

/** The toolbar's search field, for Find (⌘F). */
export const searchField = createRef<HTMLInputElement>();

/** Focuses the search field, with its text selected to type over (⌘F). */
export function focusSearch(): void {
  searchField.current?.focus();
  searchField.current?.select();
}
