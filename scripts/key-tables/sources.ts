/**
 * The upstream files the key tables are generated from (tech spec §8.2).
 *
 * Each file is vendored, unchanged, under `upstream/`, and pinned here to a revision and
 * a SHA-256 hash. The generator refuses to run if a copy doesn't match its hash, so an
 * edit to a vendored file can't slip into the tables. `upstream/README.md` says why
 * these sources and how to update them.
 */

/** An upstream project that files are copied from. */
export interface Project {
  /** The project's name, as the generated files cite it. */
  readonly name: string;
  /** The release or branch the revision belongs to, for people reading the headers. */
  readonly release: string;
  /** The full commit hash the files are copied from. */
  readonly revision: string;
  /** The URL of a file at the pinned revision, given its path in the project. */
  readonly fileUrl: (path: string) => string;
}

/** A vendored file. */
export interface UpstreamFile {
  readonly project: Project;
  /** The file's path in the upstream project. */
  readonly path: string;
  /** Where the copy lives, relative to the repository root. */
  readonly copy: string;
  /** The SHA-256 hash of the file at the pinned revision. */
  readonly sha256: string;
}

const zmkRevision = "edf5c0814fd3ea202e43aad2d68fd32e882a518c";

/** ZMK, at the release Keebart's stock firmware for the Sofle Choc Pro BT is built from. */
export const zmk: Project = {
  name: "ZMK",
  release: "v0.3.0",
  revision: zmkRevision,
  fileUrl: (path) => `https://github.com/zmkfirmware/zmk/blob/${zmkRevision}/${path}`,
};

const chromiumRevision = "863ae6f44a6d93d0aaa3674b0554f4a9298d5fd1";

/** Chromium, at the last change to its key code table. */
const chromium: Project = {
  name: "Chromium",
  release: "main as of 2025-01-03",
  revision: chromiumRevision,
  fileUrl: (path) => `https://chromium.googlesource.com/chromium/src/+/${chromiumRevision}/${path}`,
};

const upstream = "scripts/key-tables/upstream";

/** ZMK's names for keys, and the binding parameter each name stands for. */
export const zmkKeys: UpstreamFile = {
  project: zmk,
  path: "app/include/dt-bindings/zmk/keys.h",
  copy: `${upstream}/zmk/keys.h`,
  sha256: "ba77257a1ee189c1b360c8860bc51a0226afb1b5c6ba53899dc309e1e73fdd9e",
};

/** ZMK's constants for the HID usages, named after the HID Usage Tables. */
export const zmkHidUsage: UpstreamFile = {
  project: zmk,
  path: "app/include/dt-bindings/zmk/hid_usage.h",
  copy: `${upstream}/zmk/hid_usage.h`,
  sha256: "bdc33ad32cae4132d518930d1d546a31694a4c9be23ba3e91ddac072ddd4364d",
};

/** ZMK's constants for the HID usage pages. */
export const zmkHidUsagePages: UpstreamFile = {
  project: zmk,
  path: "app/include/dt-bindings/zmk/hid_usage_pages.h",
  copy: `${upstream}/zmk/hid_usage_pages.h`,
  sha256: "f73be7e38cc51706fc804867639e03f9d114e2467c4c6f1255618f31a2da9cb8",
};

/** ZMK's modifier bits, and the functions such as `LS()` that add them to a key. */
export const zmkModifiers: UpstreamFile = {
  project: zmk,
  path: "app/include/dt-bindings/zmk/modifiers.h",
  copy: `${upstream}/zmk/modifiers.h`,
  sha256: "c2c741d1306546ac10bbb5925299f2f1accd7ea97788adf8a9cb99a8281d773f",
};

/** Chromium's table of `KeyboardEvent.code` values, with their USB and Mac key codes. */
export const chromiumDomCodes: UpstreamFile = {
  project: chromium,
  path: "ui/events/keycodes/dom/dom_code_data.inc",
  copy: `${upstream}/chromium/dom_code_data.inc`,
  sha256: "841e2c58e6388f14736717eb6e365c8b8e9767788e36339bf1d35a37ab48e5b2",
};
