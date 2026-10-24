import type { Variables } from "@internationalized/string";

/** A variable's value as text, for strings with variables. */
export const text = (args: Variables, name: string) => String(args?.[name] ?? "");
