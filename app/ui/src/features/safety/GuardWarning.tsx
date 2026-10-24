import { TriangleAlert } from "lucide-react";

import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Icon } from "../../design-system/Icon.tsx";
import { type Messages, useMessages } from "../../i18n/messages.ts";
import type { GuardWarning as Finding } from "../../ipc/bindings.ts";

import styles from "./Safety.module.css";

/** What a change would break, and why that matters. */
function words(finding: Finding, messages: Messages): [string, string] {
  if (finding.kind === "last-unlock-key") {
    return [messages.format("guard.last-unlock-key"), messages.format("guard.last-unlock-key-why")];
  }
  return [
    messages.format(`guard.${finding.kind}`, { name: finding.name }),
    messages.format(`guard.${finding.kind}-why`),
  ];
}

/** Drops the change the guard stopped. */
function keep(): void {
  useStore.setState({ guard: null });
}

/** Applies the change the guard stopped. */
async function changeAnyway(proceed: () => Promise<boolean>): Promise<void> {
  useStore.setState({ guard: null });
  await proceed();
}

/**
 * The lockout guard's warning, inline at the top of the inspector (design §5.5, SAF-2): a
 * change that would remove the last Unlock key, leave a layer out of reach, or leave no way
 * back from one wasn't applied. Keep the old action drops it; Change anyway applies it,
 * and undo can take it back.
 */
export function GuardWarning() {
  const messages = useMessages();
  const guard = useStore((store) => store.guard);
  if (guard === null) return null;
  return (
    <div className={styles.warning} role="alert">
      {guard.findings.map((finding) => {
        const [what, why] = words(finding, messages);
        return (
          <p key={what} className={styles.finding}>
            <span className={styles.icon}>
              <Icon icon={TriangleAlert} />
            </span>
            <span>
              <strong>{what}</strong> {why}
            </span>
          </p>
        );
      })}
      <div className={styles.buttons}>
        <Button size="compact" onPress={keep}>
          {messages.format("guard.keep")}
        </Button>
        <Button size="compact" variant="danger" onPress={() => void changeAnyway(guard.proceed)}>
          {messages.format("guard.anyway")}
        </Button>
      </div>
    </div>
  );
}
