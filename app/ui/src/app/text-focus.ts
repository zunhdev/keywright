/**
 * Whether a text field has keyboard focus, for the Edit menu: while one has, its items
 * edit the text rather than keys (design §9.6).
 */
import { useStore } from "./store.ts";

/** Input types that don't take typed text. */
const NOT_TEXT = new Set([
  "button",
  "checkbox",
  "color",
  "file",
  "image",
  "radio",
  "range",
  "reset",
  "submit",
]);

/** Whether an element takes typed text: a text field, a text area, or editable content. */
export function takesText(element: EventTarget | null): boolean {
  if (element instanceof HTMLInputElement) return !NOT_TEXT.has(element.type);
  if (element instanceof HTMLTextAreaElement) return true;
  return element instanceof HTMLElement && element.isContentEditable;
}

/** Focus came to an element. */
function focusIn(event: FocusEvent): void {
  useStore.setState({ textFocused: takesText(event.target) });
}

/** Focus left for another element, or for nothing, such as another window. */
function focusOut(event: FocusEvent): void {
  useStore.setState({ textFocused: takesText(event.relatedTarget) });
}

/** Keeps the store's `textFocused` in step with keyboard focus, until the returned function. */
export function followTextFocus(): () => void {
  document.addEventListener("focusin", focusIn);
  document.addEventListener("focusout", focusOut);
  return () => {
    document.removeEventListener("focusin", focusIn);
    document.removeEventListener("focusout", focusOut);
  };
}
