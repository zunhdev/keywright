import { expect, test } from "bun:test";

import { act, render, screen } from "@testing-library/react";

import { expectNoAxeViolations } from "../test/axe.ts";
import { App } from "./App.tsx";
import { useStore } from "./store.ts";

test("before the session reports, the window names the app and says nothing more", () => {
  render(<App />);
  expect(screen.getByRole("heading", { level: 1 }).textContent).toBe("Keywright");
  expect(screen.getByRole("status").textContent).toBe("");
});

test("the window names the keyboard and says what the session is doing, as it changes", () => {
  useStore.setState({ session: { state: { kind: "no-device" }, keyboard: null, pack: null } });
  render(<App />);
  expect(screen.getByRole("status").textContent).toBe("Not connected");

  act(() => {
    useStore.setState({
      session: { state: { kind: "locked" }, keyboard: "Sofle Choc BT", pack: null },
    });
  });
  expect(screen.getByRole("heading", { level: 1 }).textContent).toBe("Sofle Choc BT");
  expect(screen.getByRole("status").textContent).toBe("Locked");
});

test("the app has no accessibility problems", async () => {
  useStore.setState({
    session: { state: { kind: "ready" }, keyboard: "Sofle Choc BT", pack: null },
  });
  const { container } = render(<App />);
  await expectNoAxeViolations(container);
});
