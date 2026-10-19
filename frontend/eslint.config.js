import js from '@eslint/js'
import prettier from 'eslint-config-prettier'
import svelte from 'eslint-plugin-svelte'
import globals from 'globals'
import ts from 'typescript-eslint'
import svelteConfig from './svelte.config.js'

export default ts.config(
  js.configs.recommended,
  ...ts.configs.recommended,
  ...svelte.configs.recommended,
  prettier,
  ...svelte.configs.prettier,
  { languageOptions: { globals: { ...globals.browser, ...globals.node } } },
  {
    files: ['**/*.svelte', '**/*.svelte.ts', '**/*.svelte.js'],
    languageOptions: {
      parserOptions: { projectService: true, extraFileExtensions: ['.svelte'], parser: ts.parser, svelteConfig },
    },
  },
  { ignores: ['dist/'] },
  // only lib/services talks to the API: everything else reads $lib/stores/data and emits events on $lib/events/bus
  {
    files: ['src/routes/**', 'src/lib/components/**', 'src/lib/widgets/**', 'src/lib/stores/**'],
    rules: {
      '@typescript-eslint/no-restricted-imports': [
        'error',
        {
          patterns: [
            {
              group: ['$lib/services/*', '**/services/*'],
              allowTypeImports: true,
              message: 'Read from $lib/stores/data and emit on $lib/events/bus; only lib/services calls the API.',
            },
          ],
        },
      ],
    },
  },
  // not SvelteKit: there is no $app/paths resolve() for hrefs
  { rules: { 'svelte/no-navigation-without-resolve': 'off', 'svelte/valid-prop-names-in-kit-pages': 'off' } },
)
