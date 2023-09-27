import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/kit/vite";
import { preprocessMeltUI } from "@melt-ui/pp";
import sequence from "svelte-sequential-preprocessor";
import baseTsconfig from "./tsconfig.base.json" assert { type: "json" };

const dir = "apps/frontend";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: adapter({
      fallback: "index.html",
      pages: "dist/apps/frontend",
      assets: "dist/apps/frontend",
    }),

    alias: {
      $components: `${dir}/src/lib/components`,
    },

    files: {
      assets: `${dir}/static`,
      appTemplate: `${dir}/src/app.html`,
      errorTemplate: `${dir}/src/error.html`,
      hooks: {
        client: `${dir}/src/hooks.client`,
        server: `${dir}/src/hooks.server`,
      },
      lib: `${dir}/src/lib`,
      routes: `${dir}/src/routes`,
      params: `${dir}/src/params`,
      serviceWorker: `${dir}/src/service-worker`,
    },

    outDir: `dist/frontend/.svelte-kit`,

    typescript: {
      config(cfg) {
        const paths = Object.fromEntries(
          Object.entries(baseTsconfig.compilerOptions.paths).map(
            ([key, value]) => {
              return [key, value.map((v) => `../../../${v}`)];
            },
          ),
        );

        cfg.include.push("../../../apps/frontend/vite.config.ts");
        cfg.include.push("../../../svelte.config.js");
        cfg.include.push("../../../apps/frontend/eslint.config.js");
        cfg.include.push("../../../apps/frontend/src/app.d.ts");

        cfg.compilerOptions.paths = {
          ...cfg.compilerOptions.paths,
          ...paths,
        };
      },
    },
  },

  preprocess: sequence([vitePreprocess(), preprocessMeltUI()]),
};

export default config;
