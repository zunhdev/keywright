# Keywright's UI

The React and TypeScript UI that runs in the app's window (tech spec §12.2). It never talks to the keyboard: it shows what the Rust side reports, and sends it commands.

`mise run dev` runs it inside the app, and updates it as you edit. `mise run test:ts` runs its tests, and `mise run build:debug` checks it under the app's Content Security Policy.

## Layout

```
app/ui/
├── dev.ts              # the dev server
├── bunfig.toml         # test settings: preloads, random order, the 70% coverage gate
└── src/
    ├── index.html      # the page; loads main.tsx
    ├── main.tsx        # renders the app
    ├── app/            # the app shell, its layout and providers
    ├── features/       # one folder per feature: keyboard canvas, inspector, layers, review…
    ├── design-system/  # tokens, fonts, icons and the primitives built on React Aria
    ├── ipc/            # the generated bindings to the Rust side, and the store they fill
    ├── i18n/           # the string catalogues and `useMessages`
    └── test/           # test setup and helpers
```

A folder appears with its first file.

## Conventions

- **Components** come from [React Aria Components](https://react-spectrum.adobe.com/react-aria/components.html), styled with CSS Modules (`Name.module.css` next to `Name.tsx`).
- **Strings** live in `i18n/en.ts`, never in components: `useMessages().format("app.name")`. Keys are typed, so a missing string fails `tsc`.
- **The React Compiler** memoizes components in the build, so code doesn't need `useMemo` or `useCallback` for speed.
- **Tests** sit next to the code (`Name.test.tsx`), and use Testing Library and happy-dom. Check every rendered component with `expectNoAxeViolations` from `test/axe.ts`. Fake the Rust side with `mockIPC` from `@tauri-apps/api/mocks`.
