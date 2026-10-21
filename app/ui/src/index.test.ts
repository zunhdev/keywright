import { expect, test } from "bun:test";

import { productName } from "./index.ts";

test("the placeholder names the product", () => {
  expect(productName()).toBe("Keywright");
});
