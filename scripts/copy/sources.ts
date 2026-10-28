/**
 * The words the UI shows, gathered for the copy check: the strings of the English
 * catalogue, and the words in what the app sends the UI.
 */

/** Some text the UI shows, and where it comes from, such as `error.busy`. */
export interface Text {
  readonly where: string;
  readonly text: string;
}

/** A catalogue's string: as it is, or a function of its variables. */
type Message = string | ((args: Record<string, number>) => string);

/** Variables that all have the same value, whatever their names. */
const every = (value: number | undefined) =>
  new Proxy<Record<string, number>>({}, { get: () => value });

/**
 * Every string in `catalogue`, by key. A string with variables is rendered without them,
 * then with each one at 1 and at 2: the catalogue's strings branch only on numbers, for
 * their plurals, so that takes each through all its branches, and its fallbacks too.
 */
export function catalogueTexts(catalogue: Readonly<Record<string, Message>>): Text[] {
  return Object.entries(catalogue).flatMap(([where, message]) => {
    const texts =
      typeof message === "string"
        ? [message]
        : [undefined, 1, 2].map((value) => message(every(value)));
    return [...new Set(texts)].map((text) => ({ where, text }));
  });
}

/**
 * The strings in `value`, something the app sends the UI, each with where it is, such as
 * `keymap.layers[0].name`. Strings under a field in `skip` are left out.
 */
export function payloadTexts(value: unknown, where: string, skip: ReadonlySet<string>): Text[] {
  if (typeof value === "string") {
    return [{ where, text: value }];
  }
  if (Array.isArray(value)) {
    return value.flatMap((item: unknown, index) => payloadTexts(item, `${where}[${index}]`, skip));
  }
  if (typeof value === "object" && value !== null) {
    return Object.entries(value).flatMap(([field, item]) =>
      skip.has(field) ? [] : payloadTexts(item, `${where}.${field}`, skip),
    );
  }
  return [];
}
