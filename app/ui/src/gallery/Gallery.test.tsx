import { expect, test } from "bun:test";

import { render, screen } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";

import { expectNoAxeViolations } from "../test/axe.ts";
import { Gallery } from "./Gallery.tsx";

test("the gallery shows every primitive, with no accessibility problems", async () => {
  const { container } = render(<Gallery />);
  expect(screen.getByRole("heading", { level: 1 }).textContent).toBe("Keywright design system");
  expect(screen.getByRole("toolbar", { name: "Editing" })).toBeDefined();
  expect(screen.getByRole("combobox", { name: "Action" })).toBeDefined();
  expect(screen.getAllByRole("grid", { name: "Layers" })).toHaveLength(2);
  await expectNoAxeViolations(container);
});

test("the gallery's appearance switch sets the page's theme, and System clears it", async () => {
  render(<Gallery />);
  await userEvent.click(screen.getByRole("radio", { name: "Dark" }));
  expect(document.documentElement.dataset["theme"]).toBe("dark");
  await userEvent.click(screen.getByRole("radio", { name: "System" }));
  expect(document.documentElement.dataset["theme"]).toBeUndefined();
});
