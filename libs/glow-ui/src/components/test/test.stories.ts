import { action } from '@storybook/addon-actions';
import Test from './Test.svelte';

export default {
  title: 'Test',
  component: Test,
};

export const Text = () => ({
  Component: Test,
  props: { }
});
