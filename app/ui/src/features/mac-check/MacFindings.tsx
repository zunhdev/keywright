import { TriangleAlert } from "lucide-react";

import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Icon } from "../../design-system/Icon.tsx";
import { useMessages } from "../../i18n/messages.ts";
import type { MacFinding } from "../../ipc/bindings.ts";
import { showKeys } from "../keyboard-canvas/edits.ts";
import { dismiss, fixKeys, keyOf } from "./actions.ts";

import styles from "./MacCheck.module.css";

/** Whether the keymap can be changed now, so a fix can be tried. */
function useEditable(): boolean {
  return useStore((store) => store.session?.state.kind === "ready");
}

/**
 * One finding (design §11.3): what the key types and where it is, which shows it, why it
 * doesn't work on a Mac, and Use ⌘C, where there's a fix, and Dismiss.
 */
function Finding({ found }: { found: MacFinding }) {
  const messages = useMessages();
  const editable = useEditable();
  return (
    <li className={styles.finding}>
      <span className={styles.icon}>
        <Icon icon={TriangleAlert} />
      </span>
      <span className={styles.text}>
        <strong>{found.name}</strong>
        <Button
          variant="quiet"
          size="compact"
          aria-label={messages.format("mac.show-key", { place: found.place })}
          onPress={() => showKeys([keyOf(found)])}
        >
          {found.place}
        </Button>
        <span>{found.reason}</span>
        <span className={styles.buttons}>
          {found.fix === null ? null : (
            <Button size="compact" isDisabled={!editable} onPress={() => void fixKeys([found])}>
              {messages.format("mac.fix", { fix: found.fix })}
            </Button>
          )}
          <Button variant="quiet" size="compact" onPress={() => void dismiss([found])}>
            {messages.format("mac.dismiss")}
          </Button>
        </span>
      </span>
    </li>
  );
}

/** Findings of the Mac compatibility check, each with its fix (MAC-7). */
export function MacFindings({ findings }: { findings: readonly MacFinding[] }) {
  return (
    <ul className={styles.findings}>
      {findings.map((found) => (
        <Finding
          key={`${found.finding.layer}:${found.finding.position}:${found.finding.page}:${found.finding.id}`}
          found={found}
        />
      ))}
    </ul>
  );
}

/**
 * Fix All: puts in every finding's Mac replacement as one change to try (design §8), and
 * says it's to review. Nothing while no finding has a fix.
 */
export function FixAll({ findings }: { findings: readonly MacFinding[] }) {
  const messages = useMessages();
  const editable = useEditable();
  if (!findings.some((found) => found.fix !== null)) return null;
  return (
    <div className={styles.fixAll}>
      <p className={styles.note}>{messages.format("mac.fix-note")}</p>
      <Button
        variant="primary"
        size="compact"
        isDisabled={!editable}
        onPress={() => void fixKeys(findings)}
      >
        {messages.format("mac.fix-all")}
      </Button>
    </div>
  );
}
