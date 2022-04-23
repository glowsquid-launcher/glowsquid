const WindiCSS = require("vite-plugin-windicss").default;

module.exports = {
  stories: [],
  addons: ['@storybook/addon-essentials'],
  core: {
    builder: "@storybook/builder-vite",
  },
  async viteFinal(config, { configType }) {
    config.plugins = config.plugins ?? [];
    config.plugins.push(
      WindiCSS()
    );
    return config;
  },
};
