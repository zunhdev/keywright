// Stylesheets, which Bun bundles natively.

/**
 * A CSS Module (`*.module.css`): the default export maps each class name in the file to
 * its scoped name. It comes first, so TypeScript prefers it to `*.css` below.
 */
declare module "*.module.css" {
  const classes: Readonly<Record<string, string>>;
  export default classes;
}

/** A global stylesheet, imported for its rules alone: `import "./global.css"`. */
declare module "*.css" {}
