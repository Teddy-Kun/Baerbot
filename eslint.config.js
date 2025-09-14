import js from '@eslint/js';
import { includeIgnoreFile } from '@eslint/compat';
import svelte from 'eslint-plugin-svelte';
import globals from 'globals';
import { fileURLToPath } from 'node:url';
import ts from 'typescript-eslint';
import svelteConfig from './svelte.config.js';
import stylistic from '@stylistic/eslint-plugin'
const gitignorePath = fileURLToPath(new URL("./.gitignore", import.meta.url));

export default ts.config(
	includeIgnoreFile(gitignorePath),
	js.configs.recommended,
	...ts.configs.recommended,
	...svelte.configs.recommended,
	{
		languageOptions: {
			globals: {
				...globals.browser,
				...globals.node
			}
		}
	},
	{
		files: ["**/*.svelte", "**/*.svelte.ts", "**/*.svelte.js"],

		languageOptions: {
			parserOptions: {
				projectService: true,
				extraFileExtensions: ['.svelte'],
				parser: ts.parser,
				svelteConfig
			}
		},

		plugins: {
			'@stylistic': stylistic,
		},

		rules: {
			"@stylistic/indent": ["error", "tab"],
			"@stylistic/semi": "error",
			"@stylistic/quotes": ["error", "double"],
			"@stylistic/space-before-blocks": "error",
			"@stylistic/quote-props": ["error", "as-needed"],
			"@stylistic/no-multi-spaces": "error",

			"@typescript-eslint/explicit-function-return-type": "error",
			"@typescript-eslint/no-unused-vars": [
				"error",
				{
					argsIgnorePattern: "^_",
					varsIgnorePattern: "^(_|\\$\\$)",
				},
			],
			"svelte/block-lang": [
				"error",
				{
					enforceScriptPresent: true,
					script: ["ts"],
				},
			],
			"svelte/indent": [
				"error",
				{
					indent: "tab",
				},
			],
		}
	},
	{
		ignores: ['**/*/bindings.ts']
	}
);
