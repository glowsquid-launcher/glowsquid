import { clearMocks } from "@tauri-apps/api/mocks";
import { randomFillSync, subtle } from "crypto";
import { beforeAll, describe, expect, it, vi } from "vitest";

import * as bindings  from "./bindings";

beforeAll(() => {
  window.crypto = {
    getRandomValues(buffer) {
      if (!(buffer instanceof Uint8Array)) {
        throw new TypeError("Expected Uint8Array");
      }
      return randomFillSync(buffer);
    },
    randomUUID() {
      return "00000000-0000-0000-0000-000000000000";
    },
    subtle,
  };
});

afterEach(() => clearMocks())


describe("tauriPluginCopper", () => {
  it("should call into test_connection", async () => {
    const mock = vi.fn()

    window.__TAURI_INVOKE__ = async (cmd) => {
      if (cmd === "plugin:copper|test_connection") {
        mock()
        return null
      }

      throw new Error(`Unexpected command: ${cmd}`)
    }

    expect(bindings.testConnection()).resolves.toBe(null)

    expect(mock).toHaveBeenCalled()
  });
});
