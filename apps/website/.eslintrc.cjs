require('@rushstack/eslint-patch/modern-module-resolution')

module.exports = {
  extends: '@glowsquid/eslint-config',
  parserOptions: { tsconfigRootDir: __dirname }
}
