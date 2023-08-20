import adapter from '@sveltejs/adapter-static';
import {vitePreprocess} from '@sveltejs/kit/vite';
import {preprocessMeltUI} from '@melt-ui/pp';
import sequence from 'svelte-sequential-preprocessor';

const dir = "apps/frontend"

/** @type {import('@sveltejs/kit').Config} */
const config = {
    kit: {
        adapter: adapter({
            fallback: 'index.html'
        }),

        alias: {
            $components: `${dir}/src/components`,
            $i18n: `${dir}/src/i18n/i18n-svelte`
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

        outDir: `${dir}/.svelte-kit`,

        typescript: {
          config(cfg) {
            cfg.extends = "tsconfig.base.json";
          }
        }
    },

    preprocess: sequence([vitePreprocess(), preprocessMeltUI()]),
};

export default config;
