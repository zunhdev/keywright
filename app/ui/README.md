# Keywright's UI

The React and TypeScript UI that runs in the app's window (tech spec §12.2). It never talks to the keyboard: it shows what the Rust side reports, and sends it commands.

`mise run dev` runs it inside the app, and updates it as you edit. With `KEYWRIGHT_SIMULATOR=sofle mise run dev`, the app connects to a simulated Sofle, which unlocks itself after a few seconds (`sofle-unlocked` starts unlocked). `mise run test:ts` runs the tests, and `mise run build:debug` checks the UI under the app's Content Security Policy.

## Layout

```
app/ui/
├── dev.ts              # the dev server
├── dev-css-modules.ts  # the dev server's CSS Modules, around a Bun bug
├── bunfig.toml         # test settings (preloads, random order, the 70% coverage gate), and the dev server's plugin
└── src/
    ├── index.html      # the page; loads main.tsx
    ├── main.tsx        # renders the app
    ├── app/            # the app shell, its layout and providers, and the store
    ├── features/       # one folder per feature: keyboard canvas, inspector, layers, review…
    ├── design-system/  # tokens, fonts, icons and the primitives built on React Aria
    ├── ipc/            # the generated bindings to the Rust side, and what follows its events
    ├── i18n/           # the string catalogues and `useMessages`
    └── test/           # test setup and helpers
```

A folder appears with its first file.

## Conventions

- **Components** come from [React Aria Components](https://react-spectrum.adobe.com/react-aria/components.html), styled with CSS Modules (`Name.module.css` next to `Name.tsx`).
- **Strings** live in `i18n/en.ts`, never in components: `useMessages().format("app.name")`. Keys are typed, so a missing string fails `tsc`.
- **The React Compiler** memoizes components in the build, so code doesn't need `useMemo` or `useCallback` for speed.
- **State** is one Zustand store (`app/store.ts`). Its `session` mirrors the Rust side, which `ipc/` keeps in step: components only read it. The UI's own state, such as the selection or the zoom, has actions. Domain logic stays in Rust.
- **The Rust side** is called through `ipc/bindings.ts`, which is generated: `commands.sessionState()`, `events.sessionState.listen(…)`. After changing a command, event or payload in `app/src-tauri/src/ipc/`, run `mise run generate:bindings`; `mise run check` fails until you do. A command returns `{ status: "error", error: { code } }` on failure: word each code in `i18n/en.ts`.
- **Tests** sit next to the code (`Name.test.tsx`), and use Testing Library and happy-dom. Check every rendered component with `expectNoAxeViolations` from `test/axe.ts`. Fake the Rust side with `mockIPC` from `@tauri-apps/api/mocks`.
