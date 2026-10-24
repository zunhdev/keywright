/**
 * Gives the tests a browser's globals (`window`, `document`…) from happy-dom. It's
 * preloaded first (bunfig.toml), before anything that needs them is imported.
 */
import { GlobalRegistrator } from "@happy-dom/global-registrator";

GlobalRegistrator.register();
// As the app does, before anything loads React.
await import("../no-performance-tracks.ts");
