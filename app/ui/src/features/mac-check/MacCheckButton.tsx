import { TriangleAlert } from "lucide-react";
import { DialogTrigger } from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Icon } from "../../design-system/Icon.tsx";
import { Popover } from "../../design-system/Popover.tsx";
import { TooltipTrigger } from "../../design-system/Tooltip.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { LayerName } from "../keyboard-canvas/Keycap.tsx";
import { FixAll, MacFindings } from "./MacFindings.tsx";

import styles from "./MacCheck.module.css";

/**
 * The Mac compatibility check's summary (MAC-7, design §3.1): in the toolbar, a ⚠ with
 * how many keys don't work on a Mac, while some do. It opens them all, by layer, each with
 * its fix, and Fix All. The layer overview lists a layer's own (design §5.1).
 */
export function MacCheckButton() {
  const messages = useMessages();
  const keymap = useStore((store) => store.keymap);
  const layers = keymap?.layers.filter((layer) => layer.macFindings.length > 0) ?? [];
  const findings = layers.flatMap((layer) => layer.macFindings);
  if (findings.length === 0) return null;
  const title = messages.format("mac.title", { count: findings.length });
  return (
    <DialogTrigger>
      <TooltipTrigger tip={title}>
        <Button variant="quiet" size="compact" aria-label={title}>
          <span className={styles.badge}>
            <Icon icon={TriangleAlert} />
            {findings.length}
          </span>
        </Button>
      </TooltipTrigger>
      <Popover label={title}>
        <div className={styles.summary}>
          <h2 className={styles.title}>{title}</h2>
          <div className={styles.layers}>
            {layers.map((layer) => (
              <section key={layer.id} className={styles.layer}>
                <h3 className={styles.layerName}>
                  <LayerName id={layer.id} name={layer.name} />
                </h3>
                <MacFindings findings={layer.macFindings} />
              </section>
            ))}
          </div>
          <FixAll findings={findings} />
        </div>
      </Popover>
    </DialogTrigger>
  );
}
