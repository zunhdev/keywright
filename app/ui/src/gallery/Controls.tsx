/**
 * The gallery's form controls (design §10.3): the combobox, the select and the segmented
 * control, tabs, as Settings has them (design §9.4), the checkbox, and a coach mark, as the
 * first run has them (design §8).
 */
import { Button } from "../design-system/Button.tsx";
import { Checkbox } from "../design-system/Checkbox.tsx";
import { CoachMark } from "../design-system/CoachMark.tsx";
import { ComboBox, ComboBoxItem } from "../design-system/ComboBox.tsx";
import { Segment, SegmentedControl } from "../design-system/SegmentedControl.tsx";
import { Select, SelectItem } from "../design-system/Select.tsx";
import { Tab, TabList, TabPanel, Tabs } from "../design-system/Tabs.tsx";

import styles from "./Gallery.module.css";

/** A coach mark pointing at a button, as Try it's points at Save. */
function CoachMarkSample() {
  return (
    <>
      <Button id="gallery-save" variant="primary" size="compact">
        Save
      </Button>
      <CoachMark
        target="#gallery-save"
        placement="bottom end"
        action={<Button size="compact">OK</Button>}
      >
        Changes work right away. Save to keep them when your keyboard restarts or sleeps.
      </CoachMark>
    </>
  );
}

/** The form controls, each as the app uses it. */
export function Controls() {
  return (
    <>
      <section className={styles.section}>
        <h2>Combobox, select and segmented control</h2>
        <div className={styles.row}>
          <ComboBox label="Action">
            <ComboBoxItem id="key">Key or shortcut</ComboBoxItem>
            <ComboBoxItem id="tap-hold">Tap &amp; hold</ComboBoxItem>
            <ComboBoxItem id="layer">Layer</ComboBoxItem>
            <ComboBoxItem id="mac">Mac actions</ComboBoxItem>
            <ComboBoxItem id="media">Media</ComboBoxItem>
          </ComboBox>
          <Select label="Secondary labels" defaultValue="none">
            <SelectItem id="none">None</SelectItem>
            <SelectItem id="abc">ABC</SelectItem>
            <SelectItem id="russian">Russian</SelectItem>
          </Select>
          <SegmentedControl aria-label="Layer mode" defaultSelectedKeys={["hold"]}>
            <Segment id="hold">Hold</Segment>
            <Segment id="switch">Switch</Segment>
            <Segment id="toggle">Toggle</Segment>
          </SegmentedControl>
        </div>
      </section>

      <section className={styles.section}>
        <h2>Tabs</h2>
        <div className={styles.panel}>
          <Tabs>
            <TabList aria-label="Settings">
              <Tab id="general">General</Tab>
              <Tab id="keyboard">Keyboard</Tab>
            </TabList>
            <TabPanel id="general">
              <p>Show in menu bar · Open at login</p>
            </TabPanel>
            <TabPanel id="keyboard">
              <p>Secondary labels</p>
            </TabPanel>
          </Tabs>
        </div>
      </section>

      <section className={styles.section}>
        <h2>Checkbox and coach mark</h2>
        <div className={styles.row}>
          <Checkbox defaultSelected>Show in menu bar</Checkbox>
          <Checkbox>Unchecked</Checkbox>
          <CoachMarkSample />
        </div>
      </section>
    </>
  );
}
