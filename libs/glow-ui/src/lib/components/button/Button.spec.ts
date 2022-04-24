import { render } from '@testing-library/svelte';
import Button from './Button.svelte';

describe('Button', () => {
  it('should render', () => {
    const { getByText } = render(Button);
    expect(getByText('e')).toBeDefined();
  });
});
