import { useEffect, useState } from "react";

/**
 * Reads a tab's settings with `read`, and again each time the window shows, since some can
 * change outside Keywright, such as in System Settings. `null` until they're read.
 */
export function useSettings<T>(read: () => Promise<T>): [T | null, (settings: T) => void] {
  const [settings, setSettings] = useState<T | null>(null);
  useEffect(() => {
    const load = async () => {
      setSettings(await read());
    };
    void load();
    const onShow = () => {
      if (document.visibilityState === "visible") void load();
    };
    document.addEventListener("visibilitychange", onShow);
    return () => {
      document.removeEventListener("visibilitychange", onShow);
    };
  }, [read]);
  return [settings, setSettings];
}
