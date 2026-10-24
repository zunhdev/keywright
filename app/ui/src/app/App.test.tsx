import { expect, test } from "bun:test";

import { render, screen } from "@testing-library/react";

import { expectNoAxeViolations } from "../test/axe.ts";
import { App } from "./App.tsx";

test("the main heading names the app", () => {
  render(<App />);
  expect(screen.getByRole("heading", { level: 1 }).textContent).toBe("Keywright");
});

test("the app has no accessibility problems", async () => {
  const { container } = render(<App />);
  await expectNoAxeViolations(container);
});
