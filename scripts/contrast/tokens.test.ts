import { expect, test } from "bun:test";

import { parseTokens } from "./tokens.ts";

const css = `
:root {
  color-scheme: light dark;
  --bg: light-dark(#ffffff, #111111);
  --text: light-dark( #222222 , #eeeeee );
  --line: #888888;
  --space: 4px;
}
@media (prefers-contrast: more) {
  :root {
    --line: light-dark(#555555, #aaaaaa);
  }
}
`;

test("each mode gets its own colours, and Increase Contrast overrides only what it names", () => {
  const tokens = parseTokens(css);
  expect(Object.fromEntries(tokens.light)).toEqual({
    bg: "#ffffff",
    text: "#222222",
    line: "#888888",
  });
  expect(tokens.dark.get("text")).toBe("#eeeeee");
  expect(tokens.dark.get("line")).toBe("#888888");
  expect(tokens["light, Increase Contrast"].get("line")).toBe("#555555");
  expect(tokens["dark, Increase Contrast"].get("line")).toBe("#aaaaaa");
  expect(tokens["dark, Increase Contrast"].get("bg")).toBe("#111111");
});

test("a missing Increase Contrast rule is an error, not a pass", () => {
  expect(() => parseTokens(":root { --bg: #ffffff; }")).toThrow(
    "tokens.css has no @media (prefers-contrast: more) rule",
  );
});
