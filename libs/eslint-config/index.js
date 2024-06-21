import eslint from '@eslint/js'
import prettier from 'eslint-config-prettier'
import svelte from 'eslint-plugin-svelte'
import typescript from 'typescript-eslint'
import stylistic from '@stylistic/eslint-plugin'
import svelteParser from 'svelte-eslint-parser'

export default [
    eslint.configs.recommended,
    ...typescript.configs.recommended,
    prettier,
    stylistic.configs.customize({
        indent: 4,
    }),
    /** @type {import("eslint").Linter.FlatConfig} */ ({
        files: ['**/*.ts', '**/*.svelte'],
        rules: {
            'no-undef': 'off',
        },
    }),
    /** @type {import("eslint").Linter.FlatConfig} */ ({
        files: ['**/*.ts'],
        languageOptions: {
            parser: typescript.parser,
            parserOptions: {
                ecmaVersion: 2020,
                extraFileExtensions: ['.svelte'],
                sourceType: 'module',
            },
        },
    }),
]

/** @returns {import("eslint").Linter.FlatConfig[]} */
export const createSvelteConfig = svelteConfig => ([
    ...svelte.configs['flat/recommended'],
    {
        files: ['**/*.svelte'],
        languageOptions: {
            parser: svelteParser,
            parserOptions: {
                parser: typescript.parser,
                svelteConfig,
                svelteFeatures: {
                    runes: true,
                    experimentalGenerics: true,
                },
            },
        },
    },
    {
        files: ['**/*.svelte.ts', '*.svelte.ts'],
        languageOptions: {
            parser: svelteParser,
            svelteConfig,
            parserOptions: {
                parser: typescript.parser,
                svelteConfig,
            },
        },
    },

])
