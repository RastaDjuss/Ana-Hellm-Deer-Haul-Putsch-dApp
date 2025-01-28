import { defineFlatConfig } from 'eslint-define-config';

export default defineFlatConfig([
  {
    extends: ['@antfu/eslint-config'],
    settings: {
      unocss: true, // Activer Unocss, si configurable via `settings`
    },
    rules: {
      'vue/no-extra-parens': 'off',
    },
  },
  {
    files: ['src/**/*.ts'],
    rules: {
      'no-console': 'off',
    },
  },
]);