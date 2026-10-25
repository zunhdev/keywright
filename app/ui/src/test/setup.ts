/**
 * Runs before each test file, after happy-dom (bunfig.toml): each test starts with an
 * empty page and storage, a fresh store, no toasts and no Tauri mocks.
 */
import { afterEach } from "bun:test";

import { clearMocks } from "@tauri-apps/api/mocks";
import { cleanup } from "@testing-library/react";

import { useStore } from "../app/store.ts";
import { toasts } from "../design-system/Toast.tsx";

afterEach(() => {
  cleanup();
  localStorage.clear();
  useStore.setState(useStore.getInitialState(), true);
  // A toast a test leaves would show in the next test's.
  toasts.clear();
  // Tests fake the Rust side with `mockIPC` from @tauri-apps/api/mocks.
  clearMocks();
});
