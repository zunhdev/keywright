/**
 * Turns off React's performance tracks, which its development build fills from
 * `console.timeStamp`. To log each render, it diffs the component's props, and diffing
 * React Aria's collection nodes throws ("childNodes is not supported"), which breaks
 * comboboxes whose options change as you type. Keywright doesn't use the tracks, and
 * release builds don't have them. This runs before React loads: it decides then.
 */
Reflect.deleteProperty(console, "timeStamp");
