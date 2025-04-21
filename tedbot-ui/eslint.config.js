import js from '@eslint/js';
import { includeIgnoreFile } from '@eslint/compat';
import svelte from 'eslint-plugin-svelte';
import globals from 'globals';
import { fileURLToPath } from 'node:url';
import ts from 'typescript-eslint';
import svelteConfig from './svelte.config.js';
import stylisticJs from "@stylistic/eslint-plugin-js";
import stylisticTs from "@stylistic/eslint-plugin-ts";
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
    ignores: ["eslint.config.js", "svelte.config.js"],

    languageOptions: {
	  parserOptions: {
	    projectService: true,
	    extraFileExtensions: ['.svelte'],
	    parser: ts.parser,
	    svelteConfig
	  }
	},

    plugins: {
        "@stylistic/js": stylisticJs,
        "@stylistic/ts": stylisticTs,
    },

    rules: {
        "@stylistic/ts/indent": ["error", "tab"],
			"@stylistic/ts/semi": "error",
			"@stylistic/ts/quotes": ["error", "double"],
			"@stylistic/ts/space-before-blocks": "error",
			"@stylistic/ts/quote-props": ["error", "as-needed"],
			"@stylistic/js/no-multi-spaces": "error",

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
  }
);
