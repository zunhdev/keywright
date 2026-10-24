/**
 * The UI's development server (tech spec §12.2). Bun bundles the page when it's asked
 * for, and updates it in place when a file changes: React components keep their state
 * (React Fast Refresh, which needs the `react-refresh` package). `mise run dev` starts
 * it, and the app's window loads it: `devUrl` in app/src-tauri/tauri.conf.json names
 * this address.
 */
import gallery from "./src/gallery.html";
import page from "./src/index.html";

const server = Bun.serve({
  // Reachable from this Mac only.
  hostname: "127.0.0.1",
  port: 1420,
  // The design system's gallery is for development only: the release build has just the app.
  routes: { "/gallery": gallery, "/*": page },
  // `console` repeats the page's console messages here, since the window has no
  // terminal of its own.
  development: { hmr: true, console: true },
});

console.log(`The UI's dev server is at ${server.url}, and its gallery at ${server.url}gallery`);
