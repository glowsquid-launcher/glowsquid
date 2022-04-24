const rootMain = require('../../../.storybook/main');

module.exports = {
  ...rootMain,
  "stories": [
    ...rootMain.stories,
    "../src/**/*.stories.mdx",
    "../src/**/*.stories.@(js|jsx|ts|tsx|svelte)"
  ],
  addons: [...rootMain.addons],
  "framework": "@storybook/svelte",
  "svelteOptions": {
    "preprocess": require("../svelte.config.cjs").preprocess
  }
}
