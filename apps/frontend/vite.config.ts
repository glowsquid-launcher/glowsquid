import { nxViteTsPaths } from "@nx/vite/plugins/nx-tsconfig-paths.plugin";
import { sveltekit } from "@sveltejs/kit/vite";
import browserslist from "browserslist";
import { browserslistToTargets } from "lightningcss";
import { defineConfig } from "vitest/config";

// https://tauri.app/v1/guides/faq#recommended-browserlist
const targets = browserslistToTargets(
  browserslist(["last 3 Chrome versions", "safari 13"]),
);

export default defineConfig({
  build: {
    cssMinify: "lightningcss",
    target: ["es2021", "safari13"],
  },
  cacheDir: "../../node_modules/.vite/frontend",
  css: {
    devSourcemap: true,
    lightningcss: {
      drafts: {
        nesting: true,
      },
      targets,
    },
    transformer: "lightningcss",
  },
  plugins: [nxViteTsPaths(), sveltekit()],
  server: {
    fs: {
      allow: ["."],
      deny: ["dist", ".direnv"],
    },
  },
  test: {
    cache: {
      dir: "../../node_modules/.vitest",
    },
    coverage: {
      provider: 'v8'
    },
    globals: true,
    include: ["src/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}"],
  },
});
