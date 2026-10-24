/**
 * Runs before each test file, after happy-dom (bunfig.toml): each test starts with an
 * empty page and storage, a fresh store and no Tauri mocks.
 */
import { afterEach } from "bun:test";

import { clearMocks } from "@tauri-apps/api/mocks";
import { cleanup } from "@testing-library/react";

import { useStore } from "../app/store.ts";

afterEach(() => {
  cleanup();
  localStorage.clear();
  useStore.setState(useStore.getInitialState(), true);
  // Tests fake the Rust side with `mockIPC` from @tauri-apps/api/mocks.
  clearMocks();
});
