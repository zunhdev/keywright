/**
 * CSS Modules for the dev server, around a Bun bug: with hot reloading on, Bun's dev
 * server leaves `import styles from "./x.module.css"` undefined
 * (https://github.com/oven-sh/bun/issues/18258, still in Bun 1.4.4). bunfig.toml loads
 * this plugin into the dev server only: `bun build` and `bun test` handle CSS Modules
 * themselves.
 *
 * It compiles each `*.module.css` with lightningcss, the CSS compiler Bun's own bundler
 * is built on, into a JavaScript module. That module adds the rules to the page, and
 * exports each class name's scoped name. Edits replace the rules in place.
 */
import type { BunPlugin } from "bun";
import { transform } from "lightningcss";

const plugin: BunPlugin = {
  name: "keywright-dev-css-modules",
  setup(build) {
    build.onLoad({ filter: /\.module\.css$/u }, async ({ path }) => {
      const { code, exports } = transform({
        filename: path,
        code: await Bun.file(path).bytes(),
        cssModules: true,
      });
      // A class that `composes` others is all of their names.
      const classes = Object.fromEntries(
        Object.entries(exports ?? {}).map(([name, { name: scoped, composes }]) => [
          name,
          [scoped, ...composes.map((composed) => composed.name)].join(" "),
        ]),
      );
      return {
        loader: "js",
        contents: `
const style = document.createElement("style");
style.textContent = ${JSON.stringify(code.toString())};
document.head.append(style);
if (import.meta.hot) {
  import.meta.hot.dispose(() => style.remove());
  import.meta.hot.accept();
}
export default ${JSON.stringify(classes)};
`,
      };
    });
  },
};

export default plugin;
