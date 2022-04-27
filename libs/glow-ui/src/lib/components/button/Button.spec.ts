import { ButtonStyle } from '../../types';
import { render, fireEvent } from '@testing-library/svelte';
import Button from './Button.svelte';
import { tick } from 'svelte';

describe('Button', () => {
  it('should render', async () => {
    const { getByTestId, getByText } = render(Button);
    const button = getByTestId('button');
    expect(button).toBeDefined();

    button.onclick = () => {
      button.innerHTML = 'clicked';
    };
    await fireEvent.click(button)

    expect(getByText('clicked')).toBeDefined();
    expect(button.classList.contains('default')).toBeTruthy();
  });

  it('should change style based on prop', () => {
    const { getByTestId, component } = render(Button);
    const button = getByTestId('button')
    expect(button).toBeDefined();

    const testStyle = async (style: ButtonStyle) => {
      component.$set({ variant: style });
      await tick()
      return button.classList.contains(style)
    }

    expect(testStyle(ButtonStyle.Primary)).toBeTruthy();
    expect(testStyle(ButtonStyle.Secondary)).toBeTruthy();
    expect(testStyle(ButtonStyle.Success)).toBeTruthy()
    expect(testStyle(ButtonStyle.Info)).toBeTruthy();
    expect(testStyle(ButtonStyle.Warning)).toBeTruthy();
    expect(testStyle(ButtonStyle.Danger)).toBeTruthy();
    expect(testStyle(ButtonStyle.Link)).toBeTruthy();
  })
});
