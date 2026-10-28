import { X } from "lucide-react";
import type { ComponentProps } from "react";
import {
  Text,
  UNSTABLE_Toast as AriaToast,
  UNSTABLE_ToastContent as ToastContent,
  UNSTABLE_ToastQueue as ToastQueue,
  UNSTABLE_ToastRegion as AriaToastRegion,
  VisuallyHidden,
} from "react-aria-components";
import { useToastQueue } from "react-stately/useToastState";

import { Button } from "./Button.tsx";
import { Icon } from "./Icon.tsx";

import styles from "./Toast.module.css";

/** What a toast says. */
interface Toast {
  title: string;
  description?: string;
  /** Whether a ✓ follows the title, as after saving. It's for the eye: the title says it. */
  done?: boolean;
}

/**
 * The app's toasts, such as "Saved to keyboard" (design §6):
 * `toasts.add({ title }, { timeout: 5000 })`. React Aria's toasts are still marked
 * unstable, so this keeps their API in one place.
 */
export const toasts = new ToastQueue<Toast>();

/**
 * Where toasts appear, and the log that says them. The app renders it once.
 *
 * React Aria's toast is an alert dialog holding an alert, which WebKit says at once, twice.
 * Toasts should wait their turn (design §12), so the toast's own announcements are off, and
 * a polite log says each toast once, as it appears.
 */
export function ToastRegion() {
  const { visibleToasts } = useToastQueue(toasts);
  return (
    <>
      <AriaToastRegion queue={toasts} className={styles.region}>
        {({ toast }) => (
          <AriaToast toast={toast} className={styles.toast} render={quiet}>
            <ToastContent className={styles.content} aria-live="off">
              <Text slot="title">
                {toast.content.title}
                {toast.content.done === true ? <span aria-hidden="true"> ✓</span> : null}
              </Text>
              {toast.content.description === undefined ? null : (
                <Text slot="description">{toast.content.description}</Text>
              )}
            </ToastContent>
            <Button slot="close" variant="quiet" size="compact">
              <Icon icon={X} />
            </Button>
          </AriaToast>
        )}
      </AriaToastRegion>
      {/* A sheet hides the rest of the window from VoiceOver, but not this, as it doesn't the
          toasts: React Aria keeps what's marked as its top layer. */}
      <VisuallyHidden role="log" aria-live="polite" data-react-aria-top-layer="true">
        {visibleToasts.map((toast) => (
          <div key={toast.key}>{said(toast.content)}</div>
        ))}
      </VisuallyHidden>
    </>
  );
}

/** A toast's element, with its live region off: React Aria makes it an alert dialog. */
function quiet(props: ComponentProps<"div">) {
  return <div {...props} aria-live="off" />;
}

/** What the log says of a toast: its title, then its description. */
function said({ title, description }: Toast): string {
  if (description === undefined) return title;
  return /[.!?]$/u.test(title) ? `${title} ${description}` : `${title}. ${description}`;
}
