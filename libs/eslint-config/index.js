// @ts-check

import eslint from '@eslint/js'
import svelte from 'eslint-plugin-svelte'
import typescript from 'typescript-eslint'
import stylistic from '@stylistic/eslint-plugin'
import svelteParser from 'svelte-eslint-parser'

/** @type {import("eslint").Linter.FlatConfig[]} */
// @ts-expect-error this is a valid eslint config
export default [
    eslint.configs.recommended,
    ...typescript.configs.recommended,
    stylistic.configs.customize({
        flat: true,
        indent: 4,
    }),
    ({
        files: ['**/*.ts'],
        languageOptions: {
            parser: typescript.parser,
            parserOptions: {
                ecmaVersion: 'latest',
                extraFileExtensions: ['.svelte'],
                sourceType: 'module',
            },
        },
        rules: {
            'no-undef': 'off',
        },
    }),
]

/**
 * Creates a svelte eslint config
 *
 * @param {import('svelte-eslint-parser').SvelteConfig} svelteConfig
 * @returns {import("eslint").Linter.FlatConfig[]}
 * */
export const createSvelteConfig = svelteConfig => ([
    ...svelte.configs['flat/recommended'].map(({ rules, ...rest }) => ({
        rules: {
            ...rules,
        },
        ...rest,
    })),
    {
        files: ['*.svelte', '**/*.svelte'],
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
        rules: {
            'no-undef': 'off',
        },
        processor: svelte.processors.svelte,
    },
    {
        files: ['**/*.svelte.ts', '*.svelte.ts'],
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
])
