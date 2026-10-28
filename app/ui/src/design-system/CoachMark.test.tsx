import { expect, mock, test } from "bun:test";

import { act, render, screen, waitForElementToBeRemoved } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";

import { expectNoAxeViolations } from "../test/axe.ts";
import { Button } from "./Button.tsx";
import { CoachMark } from "./CoachMark.tsx";

test("a coach mark's button comes last for Tab, which goes on from it in the page's order", async () => {
  render(
    <>
      <button type="button">Before</button>
      <button type="button" id="tab">
        Tab
      </button>
      <button type="button">After</button>
      <CoachMark target="#tab" action={<Button>Skip</Button>}>
        Click a key.
      </CoachMark>
    </>,
  );
  // What had focus when it showed: a popover sends Tab from its button to what follows it.
  screen.getByRole("button", { name: "Tab" }).focus();
  await screen.findByRole("note");
  await userEvent.tab();
  expect(document.activeElement?.textContent).toBe("After");
  await userEvent.tab();
  expect(document.activeElement?.textContent).toBe("Skip");
  await userEvent.tab();
  expect(document.activeElement?.textContent === "After").toBe(false);
  // It's not a popover, so there's no Dismiss button that couldn't dismiss it.
  expect(screen.queryByRole("button", { name: "Dismiss" }) === null).toBe(true);
});

test("a coach mark points at what's drawn, leaves focus where it is, and offers its button", async () => {
  const onPress = mock<() => void>();
  render(
    <CoachMark target="#tab" action={<Button onPress={onPress}>Skip</Button>}>
      Click a key.
    </CoachMark>,
  );
  // Booleans, not elements: a failing element match hangs while bun prints it.
  expect(screen.queryByRole("note") === null).toBe(true);

  // It shows once what it points at is drawn, and goes with it.
  const target = document.createElement("button");
  target.id = "tab";
  target.textContent = "Tab";
  act(() => {
    document.body.append(target);
  });
  target.focus();
  const coachMark = await screen.findByRole("note");
  expect(coachMark.textContent).toContain("Click a key.");
  expect(document.activeElement === target).toBe(true);
  await expectNoAxeViolations(document.body);
  await userEvent.click(screen.getByRole("button", { name: "Skip" }));
  expect(onPress).toHaveBeenCalledTimes(1);
  const removed = waitForElementToBeRemoved(coachMark);
  act(() => {
    target.remove();
  });
  await removed;
});
