import Test from './Test.svelte';

export default {
  title: 'Test',
  component: Test,
};

/**
 * storybook test
**/
export const Text = () => ({
  Component: Test,
  props: { }
});
