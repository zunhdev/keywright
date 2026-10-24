/**
 * Checks the UI's design tokens for contrast (tech spec §13.3, design §10.2): every pair
 * of colours the UI draws together meets WCAG AA, in light and dark, and with Increase
 * Contrast; and the first layer colours stay apart with colour-vision deficiencies.
 * `mise run lint:contrast` runs it, and CI too.
 */

import { join } from "node:path";

import { check, PAIRS } from "./contrast/rules.ts";
import { parseTokens } from "./contrast/tokens.ts";

const tokensFile = join(import.meta.dir, "../app/ui/src/design-system/tokens.css");
const failures = check(parseTokens(await Bun.file(tokensFile).text()));

if (failures.length > 0) {
  for (const { mode, problem } of failures) {
    console.error(`${mode}: ${problem}`);
  }
  console.error(`\n${failures.length} problem(s) in app/ui/src/design-system/tokens.css.`);
  process.exit(1);
}
const pairs = PAIRS.reduce((count, pair) => count + pair.backgrounds.length, 0);
console.log(
  `The design tokens pass: ${pairs} kinds of pair, in light, dark and Increase Contrast.`,
);
