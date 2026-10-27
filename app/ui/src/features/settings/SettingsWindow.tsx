import { Tab, TabList, TabPanel, Tabs } from "../../design-system/Tabs.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { AdvancedTab } from "./AdvancedTab.tsx";
import { BackupsTab } from "./BackupsTab.tsx";
import { GeneralTab } from "./GeneralTab.tsx";
import { KeyboardTab } from "./KeyboardTab.tsx";

import styles from "./Settings.module.css";

/**
 * The Settings window (design §9.4): General, Keyboard, Backups and Advanced. Every setting
 * takes effect at once, and the Rust side keeps it in `settings.json`.
 */
export function SettingsWindow() {
  const messages = useMessages();
  return (
    <main className={styles.window}>
      <Tabs>
        <TabList aria-label={messages.format("settings.tabs")}>
          <Tab id="general">{messages.format("settings.general")}</Tab>
          <Tab id="keyboard">{messages.format("settings.keyboard")}</Tab>
          <Tab id="backups">{messages.format("settings.backups")}</Tab>
          <Tab id="advanced">{messages.format("settings.advanced")}</Tab>
        </TabList>
        <TabPanel id="general">
          <GeneralTab />
        </TabPanel>
        <TabPanel id="keyboard">
          <KeyboardTab />
        </TabPanel>
        <TabPanel id="backups">
          <BackupsTab />
        </TabPanel>
        <TabPanel id="advanced">
          <AdvancedTab />
        </TabPanel>
      </Tabs>
    </main>
  );
}
