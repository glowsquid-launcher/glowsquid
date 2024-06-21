import baseConfig, { createSvelteConfig } from '@repo/eslint-config'
import svelteConfig from './svelte.config.js'

/** @type {import('eslint').Linter.FlatConfig[]} */
export default [
    {
        ignores: ['node_modules'],
    },
    ...baseConfig,
    ...createSvelteConfig(svelteConfig),
]
