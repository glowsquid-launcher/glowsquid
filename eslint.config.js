import js from '@eslint/js';
import nxPlugin from '@nx/eslint-plugin';
import typescriptPlugin from '@typescript-eslint/eslint-plugin';
import typescriptParser from '@typescript-eslint/parser';
import prettier from 'eslint-config-prettier';
import perfectionistNatural from 'eslint-plugin-perfectionist/configs/recommended-natural';
import prettierPlugin from 'eslint-plugin-prettier';
import sveltePlugin from 'eslint-plugin-svelte';
import jsoncParser from 'jsonc-eslint-parser';
import { join } from 'path';
import svelteParser from 'svelte-eslint-parser';
import { fileURLToPath } from 'url';

const prettierConfig = {
	overrides: [
		{
			files: '*.svelte',
			options: {
				parser: 'svelte'
			}
		}
	],
	plugins: ['prettier-plugin-svelte'],
	printWidth: 100,
	singleQuote: true,
	trailingComma: 'none',
	useTabs: true
};

/**
 *
 * @param {string[]} tsconfigPath path to tsconfig.json (or multiple). Usually join(fileURLToPath(import.meta.url), '..', 'tsconfig.json')
 * @returns {import("eslint").Linter.FlatConfig[]}
 */
export function createConfig(tsconfigPath) {
	return [
		js.configs.recommended,
		{
			plugins: {
				'@nx': nxPlugin,
				'@typescript-eslint': typescriptPlugin
			},
			rules: {
				...typescriptPlugin.configs.recommended.rules,
				'@nx/enforce-module-boundaries': [
					'error',
					{
						allow: [],
						depConstraints: [
							{
								onlyDependOnLibsWithTags: ['*'],
								sourceTag: '*'
							}
						],
						enforceBuildableLibDependency: true
					}
				],
				'arrow-body-style': 'off',
				'no-undef': 'off',
				'prefer-arrow-callback': 'off'
			}
		},
		{
			files: ['**/*.ts'],
			languageOptions: {
				parser: typescriptParser,
				parserOptions: {
					ecmaVersion: 2020,
					extraFileExtensions: ['.svelte'],
					project: tsconfigPath,
					sourceType: 'module'
				}
			}
		},
		{
			files: ['**/*.svelte'],
			languageOptions: {
				parser: svelteParser,
				parserOptions: {
					parser: typescriptParser
				}
			},
			plugins: {
				// prettier: prettierPlugin,
				svelte: sveltePlugin
			},
			processor: 'svelte/svelte',
			rules: {
				...sveltePlugin.configs.recommended.rules
				// ...sveltePlugin.configs.prettier.rules,
			}
		},
		{
			files: ['*.json'],
			languageOptions: {
				parser: jsoncParser
			},
			plugins: {
				'@nx': nxPlugin
			},
			rules: { '@nx/dependency-checks': 'error' }
		},
		{
			files: ['*.ts', '*.tsx', '*.js', '*.jsx'],
			plugins: {
				prettier: prettierPlugin
			},
			rules: {
				'prettier/prettier': [
					'error',
					prettierConfig,
					{
						usePrettierrc: false
					}
				]
			}
		},
		perfectionistNatural,
		prettier
	];
}

export default createConfig(join(fileURLToPath(import.meta.url), '..', 'tsconfig.base.json'));
