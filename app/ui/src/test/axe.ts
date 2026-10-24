import { expect } from "bun:test";

import axe from "axe-core";

/**
 * Checks `container` with axe, and fails the test with each accessibility problem it
 * finds (tech spec §13.3, NFR-5). happy-dom doesn't lay pages out, so axe can't judge
 * colour contrast here: the contrast check covers the design tokens instead.
 *
 * Its `region` rule, which wants all of a page's content in landmarks, is off: tests
 * render components on their own, and overlays outside the page's `main`.
 */
export async function expectNoAxeViolations(container: Element): Promise<void> {
  const results = await axe.run(container, { rules: { region: { enabled: false } } });
  const violations = results.violations.map(
    (violation) =>
      `${violation.id}: ${violation.help} (${violation.nodes.map((node) => node.target.join(" ")).join(", ")})`,
  );
  expect(violations).toEqual([]);
}
