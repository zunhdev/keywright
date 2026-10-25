/**
 * The gallery's form controls (design §10.3): the combobox, the select and the segmented
 * control, and tabs, as Settings has them (design §9.4).
 */
import { ComboBox, ComboBoxItem } from "../design-system/ComboBox.tsx";
import { Segment, SegmentedControl } from "../design-system/SegmentedControl.tsx";
import { Select, SelectItem } from "../design-system/Select.tsx";
import { Tab, TabList, TabPanel, Tabs } from "../design-system/Tabs.tsx";

import styles from "./Gallery.module.css";

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
    </>
  );
}
