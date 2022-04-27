import { render } from '@testing-library/svelte';
import Button from './Button.svelte';

describe('Button', () => {
  it('should render', () => {
    const { getByTestId, getByText } = render(Button);
    const button = getByTestId('button');
    expect(button).toBeDefined();

    button.onclick = () => {
      button.innerHTML = 'clicked';
    };

    button.click();
    expect(getByText('clicked')).toBeDefined();
  });
});
