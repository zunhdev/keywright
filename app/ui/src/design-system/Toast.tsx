import { X } from "lucide-react";
import {
  Text,
  UNSTABLE_Toast as AriaToast,
  UNSTABLE_ToastContent as ToastContent,
  UNSTABLE_ToastQueue as ToastQueue,
  UNSTABLE_ToastRegion as AriaToastRegion,
} from "react-aria-components";

import { Button } from "./Button.tsx";
import { Icon } from "./Icon.tsx";

import styles from "./Toast.module.css";

/** What a toast says. */
interface Toast {
  title: string;
  description?: string;
}

/**
 * The app's toasts, such as "Saved to keyboard" (design §6):
 * `toasts.add({ title }, { timeout: 5000 })`. React Aria's toasts are still marked
 * unstable, so this keeps their API in one place.
 */
export const toasts = new ToastQueue<Toast>();

/** Where toasts appear. The app renders it once. */
export function ToastRegion() {
  return (
    <AriaToastRegion queue={toasts} className={styles.region}>
      {({ toast }) => (
        <AriaToast toast={toast} className={styles.toast}>
          <ToastContent className={styles.content}>
            <Text slot="title">{toast.content.title}</Text>
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
  );
}
