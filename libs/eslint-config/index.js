import eslint from '@eslint/js';
import prettier from 'eslint-config-prettier';
import svelte from 'eslint-plugin-svelte';
import tseslint from 'typescript-eslint';

export default [
	eslint.configs.recommended,
	...tseslint.configs.recommended,
	...svelte.configs['flat/recommended'],
	prettier,
	/** @type {import("eslint").Linter.FlatConfig} */ ({
		files: ['**/*.ts', '**/*.svelte'],
        rules: {
            "no-undef": "off",
        }
    }),
	/** @type {import("eslint").Linter.FlatConfig} */ ({
		files: ['**/*.ts'],
		languageOptions: {
			parser: tseslint.parser,
			parserOptions: {
				ecmaVersion: 2020,
				extraFileExtensions: ['.svelte'],
				sourceType: 'module'
			}
		}
	}),
	/** @type {import("eslint").Linter.FlatConfig} */ ({
		files: ['**/*.svelte'],
		languageOptions: {
			parserOptions: {
				parser: tseslint.parser
			}
		}
	}),

];
