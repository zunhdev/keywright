/**
 * Runs before each test file, after happy-dom (bunfig.toml): each test starts with an
 * empty page and no Tauri mocks.
 */
import { afterEach } from "bun:test";

import { clearMocks } from "@tauri-apps/api/mocks";
import { cleanup } from "@testing-library/react";

afterEach(() => {
  cleanup();
  // Tests fake the Rust side with `mockIPC` from @tauri-apps/api/mocks.
  clearMocks();
});
