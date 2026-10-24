import { type ReactNode, useEffect, useState } from "react";

/**
 * `children`, once `delay` ms have passed since this was drawn, and `fallback` until then.
 * Give it a `key` that changes with what it waits for, to wait again.
 */
export function Delayed({
  delay,
  fallback = null,
  children,
}: {
  delay: number;
  fallback?: ReactNode;
  children: ReactNode;
}) {
  const [shown, setShown] = useState(false);
  useEffect(() => {
    const timer = setTimeout(() => {
      setShown(true);
    }, delay);
    return () => {
      clearTimeout(timer);
    };
  }, [delay]);
  return shown ? children : fallback;
}
