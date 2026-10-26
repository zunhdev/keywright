import { MacCheckStep } from "./MacCheckStep.tsx";
import { MenuBarStep } from "./MenuBarStep.tsx";
import { TryIt } from "./TryIt.tsx";

/**
 * The first run's own surfaces in the main window (design §8): the Mac check, Try it's
 * coach marks and the menu bar question. Its first two steps are the no-keyboard screen's
 * welcome and the unlock sheet's extra line, which those show themselves.
 */
export function FirstRun() {
  return (
    <>
      <MacCheckStep />
      <TryIt />
      <MenuBarStep />
    </>
  );
}
