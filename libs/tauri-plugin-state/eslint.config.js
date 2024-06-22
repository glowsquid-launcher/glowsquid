import baseConfig from '@repo/eslint-config'

/** @type {import('eslint').Linter.FlatConfig[]} */
export default [
    {
        ignores: ['node_modules', 'dist-js'],
    },
    ...baseConfig,
]
