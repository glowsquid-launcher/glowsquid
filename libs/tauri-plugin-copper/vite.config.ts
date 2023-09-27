/// <reference types="vitest" />
import { nxViteTsPaths } from "@nx/vite/plugins/nx-tsconfig-paths.plugin";
import * as path from "path";
import { defineConfig } from "vite";
import dts from "vite-plugin-dts";

export default defineConfig({
  // See: https://vitejs.dev/guide/build.html#library-mode
  build: {
    lib: {
      // Could also be a dictionary or array of multiple entry points.
      entry: "src/index.ts",
      fileName: "index",
      // Don't forget to update your package.json as well.
      formats: ["es", "cjs"],
      // Change this to the formats you want to support.
      name: "tauri-plugin-copper",
    },
    rollupOptions: {
      // External packages that should not be bundled into your library.
      external: [],
    },
  },

  cacheDir: "../../node_modules/.vite/tauri-plugin-copper",

  // Configuration for building your library.
  plugins: [
    dts({
      entryRoot: "src",
      skipDiagnostics: true,
      tsConfigFilePath: path.join(__dirname, "tsconfig.lib.json"),
    }),

    nxViteTsPaths(),
  ],

  test: {
    cache: {
      dir: "../../node_modules/.vitest",
    },
    environment: "node",
    globals: true,
    include: ["src/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}"],
  },
});
