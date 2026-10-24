# Keywright's UI

The React and TypeScript UI that runs in the app's window (tech spec §12.2). It never talks to the keyboard: it shows what the Rust side reports, and sends it commands.

`mise run dev` runs it inside the app, and updates it as you edit. With `KEYWRIGHT_SIMULATOR=sofle mise run dev`, the app connects to a simulated Sofle, which unlocks itself after a few seconds (`sofle-unlocked` starts unlocked). `mise run test:ts` runs the tests, and `mise run build:debug` checks the UI under the app's Content Security Policy. While `mise run dev` runs, the design system's gallery is at <http://127.0.0.1:1420/gallery>: every token and primitive on one page, in any browser.

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
    ├── design-system/  # tokens.css, the icon wrapper, and the primitives built on React Aria
    ├── gallery/        # the design system's gallery, for development only
    ├── ipc/            # the generated bindings to the Rust side, and what follows its events
    ├── i18n/           # the string catalogues and `useMessages`
    └── test/           # test setup and helpers
```

A folder appears with its first file.

## Conventions

- **Components** are built from the design system's primitives (`Button`, `Toolbar`, `MenuButton`, `ComboBox`, `Sheet`, `Popover`, `List`, `SegmentedControl`, toasts), which wrap [React Aria Components](https://react-spectrum.adobe.com/react-aria/components.html). Style with CSS Modules (`Name.module.css` next to `Name.tsx`), and only with the tokens in `design-system/tokens.css`. Run `mise run generate:css-types` after changing a CSS Module's classes: its typings name them, so `tsc` catches a class that doesn't exist.
- **Colours** are checked: `mise run lint:contrast` fails if a token pair the UI draws drops below WCAG AA. A new pairing gets a rule in `scripts/contrast/rules.ts`.
- **Motion** moves by multiples of `--motion-distance`, which Reduce Motion sets to 0, so animations become fades.
- **Icons** come from Lucide, through `<Icon icon={…} />`, which sizes them and hides them from VoiceOver: label the button or text they sit in.
- **Strings** live in `i18n/en.ts`, never in components: `useMessages().format("app.name")`. Keys are typed, so a missing string fails `tsc`.
- **The React Compiler** memoizes components in the build, so code doesn't need `useMemo` or `useCallback` for speed.
- **State** is one Zustand store (`app/store.ts`). Its `session` and `keymap` mirror the Rust side, which `ipc/` keeps in step: components only read them. The UI's own state, such as the selection or the zoom, has actions. Domain logic stays in Rust.
- **Menu items** that Keywright owns arrive as `menu://item` events: `ipc/menu.ts` handles the ones the UI acts on, and enables and renames them with `commands.menuSet`.
- **The Rust side** is called through `ipc/bindings.ts`, which is generated: `commands.sessionState()`, `events.sessionState.listen(…)`. After changing a command, event or payload in `app/src-tauri/src/ipc/`, run `mise run generate:bindings`; `mise run check` fails until you do. A command returns `{ status: "error", error: { code } }` on failure: word each code in `i18n/en.ts`.
- **Tests** sit next to the code (`Name.test.tsx`), and use Testing Library and happy-dom. Check every rendered component with `expectNoAxeViolations` from `test/axe.ts`. Fake the Rust side with `mockIPC` from `@tauri-apps/api/mocks`. `test/sofle-keymap.ts` is the simulated Sofle's keymap as the Rust side sends it, written by `mise run generate:bindings`: draw it rather than making keymaps up. Keycaps have snapshot tests (`features/keyboard-canvas/__snapshots__/`): after changing how one looks, check the gallery, then run `bun test -u` to update them.
- **The keyboard canvas** (`features/keyboard-canvas/`) draws what the Rust side says each key shows: its keycap and its sentence come from `keywright-core`, in Mac terms. The canvas only places, sizes and styles them.
