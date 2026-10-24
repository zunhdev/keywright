import { KEY_RECORDER } from "../../app/store.ts";
import { useMessages } from "../../i18n/messages.ts";
import type { Action, Choice } from "../../ipc/bindings.ts";
import type { CategoryId } from "./categories.ts";
import { ChoiceList } from "./ChoiceList.tsx";
import { ShortcutField } from "./fields.tsx";
import { LayersEditor } from "./LayersEditor.tsx";
import { OtherEditor } from "./OtherEditor.tsx";
import { SpecialEditor } from "./SpecialEditor.tsx";
import { TapHoldEditor } from "./TapHoldEditor.tsx";

import styles from "./Inspector.module.css";

/**
 * The editor for one kind of action (design §5.4), showing `current`, the key's action,
 * where it's of that kind. Each change goes to `onChange`, with the choice it came from if
 * it was one, for the picker's recent actions.
 */
export function ActionEditor({
  category,
  current,
  onChange,
}: {
  category: CategoryId;
  current: Action | null;
  onChange: (action: Action, choice?: Choice) => void;
}) {
  const messages = useMessages();
  if (category === "key") {
    return (
      <ShortcutField
        label={messages.format("key.field")}
        value={
          current?.kind === "key-press" && current.key.page === "keyboard" ? current.key : null
        }
        recorder={KEY_RECORDER}
        onChange={(key) => {
          onChange({ kind: "key-press", key });
        }}
      />
    );
  }
  if (category === "media" || category === "bluetooth" || category === "lighting") {
    return (
      <ChoiceList
        label={messages.format(`category.${category}`)}
        category={category}
        current={current}
        onChoose={(choice) => {
          onChange(choice.action, choice);
        }}
      />
    );
  }
  if (category === "tap-hold") return <TapHoldEditor current={current} onChange={onChange} />;
  if (category === "layers") return <LayersEditor current={current} onChange={onChange} />;
  if (category === "special") return <SpecialEditor current={current} onChange={onChange} />;
  if (category === "same-as-below") {
    return <p className={styles.note}>{messages.format("same-as-below.explanation")}</p>;
  }
  if (category === "does-nothing") {
    return <p className={styles.note}>{messages.format("does-nothing.explanation")}</p>;
  }
  return <OtherEditor current={current} onChange={onChange} />;
}
