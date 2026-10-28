import { afterEach, describe, expect, mock, test } from "bun:test";

import { act, render, screen } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { Lock } from "lucide-react";
import { DialogTrigger, type Key, type Selection } from "react-aria-components";

import { expectNoAxeViolations } from "../test/axe.ts";
import { Button } from "./Button.tsx";
import { Checkbox } from "./Checkbox.tsx";
import { ComboBox, ComboBoxItem } from "./ComboBox.tsx";
import { Icon } from "./Icon.tsx";
import { List, ListItem } from "./List.tsx";
import { MenuButton, MenuItem, MenuSeparator } from "./Menu.tsx";
import { NumberField } from "./NumberField.tsx";
import { Popover } from "./Popover.tsx";
import { Segment, SegmentedControl, ToggleGroup } from "./SegmentedControl.tsx";
import { Sheet } from "./Sheet.tsx";
import { ToastRegion, toasts } from "./Toast.tsx";
import { Toolbar, ToolbarSeparator } from "./Toolbar.tsx";

afterEach(() => {
  act(() => {
    toasts.clear();
  });
});

describe("Button", () => {
  test("presses, and says which variant and size it is for the styles", async () => {
    const onPress = mock<() => void>();
    render(
      <Button variant="primary" size="compact" onPress={onPress}>
        Save
      </Button>,
    );
    const button = screen.getByRole("button", { name: "Save" });
    expect(button.dataset["variant"]).toBe("primary");
    expect(button.dataset["size"]).toBe("compact");
    await userEvent.click(button);
    expect(onPress).toHaveBeenCalledTimes(1);
  });

  test("a disabled button doesn't press", async () => {
    const onPress = mock<() => void>();
    render(
      <Button isDisabled onPress={onPress}>
        Save
      </Button>,
    );
    await userEvent.click(screen.getByRole("button", { name: "Save" }));
    expect(onPress).not.toHaveBeenCalled();
  });
});

test("an icon is hidden from VoiceOver: what it's in says what it means", async () => {
  const { container } = render(
    <Button aria-label="Lock">
      <Icon icon={Lock} />
    </Button>,
  );
  expect(container.querySelector("svg")?.getAttribute("aria-hidden")).toBe("true");
  await expectNoAxeViolations(container);
});

test("a toolbar is one tab stop, and the arrow keys move along it", async () => {
  render(
    <Toolbar aria-label="Editing">
      <Button>Undo</Button>
      <ToolbarSeparator />
      <Button>Redo</Button>
    </Toolbar>,
  );
  await userEvent.tab();
  expect(document.activeElement).toBe(screen.getByRole("button", { name: "Undo" }));
  await userEvent.keyboard("{ArrowRight}");
  expect(document.activeElement).toBe(screen.getByRole("button", { name: "Redo" }));
  await expectNoAxeViolations(screen.getByRole("toolbar"));
});

test("a menu opens from its button, shows shortcuts, and runs the chosen action", async () => {
  const onAction = mock<(key: Key) => void>();
  render(
    <MenuButton label="Actions" onAction={onAction}>
      <MenuItem id="undo" shortcut="⌘Z">
        Undo
      </MenuItem>
      <MenuSeparator />
      <MenuItem id="copy" shortcut="⌘C">
        Copy Action
      </MenuItem>
    </MenuButton>,
  );
  await userEvent.click(screen.getByRole("button", { name: "Actions" }));
  const item = screen.getByRole("menuitem", { name: /Copy Action/u });
  expect(item.textContent).toContain("⌘C");
  await expectNoAxeViolations(document.body);
  await userEvent.click(item);
  expect(onAction.mock.calls[0]?.[0]).toBe("copy");
});

test("a combobox suggests the options that match what's typed", async () => {
  render(
    <ComboBox label="Action">
      <ComboBoxItem id="key">Key or shortcut</ComboBoxItem>
      <ComboBoxItem id="layer">Layer</ComboBoxItem>
    </ComboBox>,
  );
  await userEvent.type(screen.getByRole("combobox", { name: "Action" }), "lay");
  const options = screen.getAllByRole("option").map((option) => option.textContent);
  expect(options).toEqual(["Layer"]);
  await expectNoAxeViolations(document.body);
});

test("a sheet takes focus, names itself, and gives focus back when it closes", async () => {
  render(
    <DialogTrigger>
      <Button>Review changes…</Button>
      <Sheet title="Review changes">
        {(close) => (
          <Button variant="primary" onPress={close}>
            Save to keyboard
          </Button>
        )}
      </Sheet>
    </DialogTrigger>,
  );
  const trigger = screen.getByRole("button", { name: "Review changes…" });
  await userEvent.click(trigger);
  const sheet = screen.getByRole("dialog", { name: "Review changes" });
  expect(sheet.contains(document.activeElement)).toBe(true);
  await expectNoAxeViolations(document.body);

  await userEvent.keyboard("{Escape}");
  expect(screen.queryByRole("dialog")).toBeNull();
  expect(document.activeElement).toBe(trigger);
});

test("a popover shows its details by what opened it", async () => {
  render(
    <DialogTrigger>
      <Button>USB · Unlocked</Button>
      <Popover label="Connection">Connected over USB.</Popover>
    </DialogTrigger>,
  );
  await userEvent.click(screen.getByRole("button", { name: "USB · Unlocked" }));
  expect(screen.getByRole("dialog", { name: "Connection" }).textContent).toBe(
    "Connected over USB.",
  );
  await expectNoAxeViolations(document.body);
});

test("a list's rows can be selected, and their actions reached", async () => {
  const onSelectionChange = mock<(keys: Selection) => void>();
  render(
    <List aria-label="Layers" selectionMode="single" onSelectionChange={onSelectionChange}>
      <ListItem id="base" textValue="Base" actions={<Button aria-label="Rename Base">✎</Button>}>
        Base
      </ListItem>
      <ListItem id="lower" textValue="Lower">
        Lower
      </ListItem>
    </List>,
  );
  await userEvent.click(screen.getByRole("row", { name: "Lower" }));
  expect(onSelectionChange.mock.calls[0]?.[0]).toEqual(new Set(["lower"]));
  expect(screen.getByRole("button", { name: "Rename Base" })).toBeDefined();
  await expectNoAxeViolations(screen.getByRole("grid"));
});

test("a segmented control always has exactly one choice", async () => {
  render(
    <SegmentedControl aria-label="Appearance" defaultSelectedKeys={["system"]}>
      <Segment id="system">System</Segment>
      <Segment id="dark">Dark</Segment>
    </SegmentedControl>,
  );
  const [system, dark] = [
    screen.getByRole("radio", { name: "System" }),
    screen.getByRole("radio", { name: "Dark" }),
  ];
  await userEvent.click(dark);
  expect(dark.getAttribute("aria-checked")).toBe("true");
  expect(system.getAttribute("aria-checked")).toBe("false");
  // Choosing the chosen one again doesn't leave none.
  await userEvent.click(dark);
  expect(dark.getAttribute("aria-checked")).toBe("true");
  await expectNoAxeViolations(screen.getByRole("radiogroup"));
});

test("a toast says what happened, and can be closed", async () => {
  render(<ToastRegion />);
  act(() => {
    toasts.add({ title: "Saved to keyboard", description: "3 changes" });
  });
  const toast = screen.getByRole("alertdialog");
  expect(toast.textContent).toContain("Saved to keyboard");
  expect(toast.textContent).toContain("3 changes");
  await expectNoAxeViolations(document.body);
  await userEvent.click(screen.getByRole("button", { name: "Close" }));
  expect(screen.queryByRole("alertdialog")).toBeNull();
});

test("a number field steps within its range, and says each number", async () => {
  const onChange = mock<(value: number) => void>();
  render(
    <NumberField label="Profile" minValue={0} maxValue={1} defaultValue={0} onChange={onChange} />,
  );
  const field = screen.getByRole("textbox", { name: "Profile" });
  await userEvent.click(screen.getByRole("button", { name: /^Increase/u }));
  expect(onChange).toHaveBeenLastCalledWith(1);
  expect(screen.getByRole("button", { name: /^Increase/u }).hasAttribute("disabled")).toBe(true);
  expect(field).toHaveProperty("value", "1");
});

test("a toggle group's options are each on or off", async () => {
  let selected: Set<Key> = new Set();
  const onSelectionChange = (keys: Set<Key>) => {
    selected = keys;
  };
  render(
    <ToggleGroup aria-label="Modifiers" onSelectionChange={onSelectionChange}>
      <Segment id="control">⌃</Segment>
      <Segment id="shift">⇧</Segment>
    </ToggleGroup>,
  );
  await userEvent.click(screen.getByRole("button", { name: "⌃" }));
  await userEvent.click(screen.getByRole("button", { name: "⇧" }));
  expect([...selected]).toEqual(["control", "shift"]);
});

test("a checkbox is named by its label, and turns on and off", async () => {
  const onChange = mock<(on: boolean) => void>();
  render(
    <Checkbox defaultSelected onChange={onChange}>
      Show in menu bar
    </Checkbox>,
  );
  const box = screen.getByRole("checkbox", { name: "Show in menu bar" });
  expect(box instanceof HTMLInputElement && box.checked).toBe(true);
  await userEvent.click(box);
  expect(onChange.mock.calls).toEqual([[false]]);
  await expectNoAxeViolations(document.body);
});
