/**
 * The UI's development server (tech spec §12.2). Bun bundles the page when it's asked
 * for, and updates it in place when a file changes. `mise run dev` starts it, and the
 * app's window loads it: `devUrl` in app/src-tauri/tauri.conf.json names this address.
 */
import page from "./src/index.html";

const server = Bun.serve({
  // Reachable from this Mac only.
  hostname: "127.0.0.1",
  port: 1420,
  routes: { "/*": page },
  // `console` repeats the page's console messages here, since the window has no
  // terminal of its own.
  development: { hmr: true, console: true },
});

console.log(`The UI's dev server is at ${server.url}`);
