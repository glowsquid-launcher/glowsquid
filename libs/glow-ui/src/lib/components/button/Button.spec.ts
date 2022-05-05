import { ButtonVariant } from '../../types'
import { render, fireEvent } from '@testing-library/svelte'
import Button from './Button.svelte'
import { tick } from 'svelte'

describe('Button', () => {
  it('should render', async () => {
    const { getByTestId, getByText } = render(Button)
    const button = getByTestId('button')
    expect(button).toBeDefined()

    button.onclick = () => {
      button.innerHTML = 'clicked'
    }
    await fireEvent.click(button)

    expect(getByText('clicked')).toBeDefined()
    expect(button.classList.contains('primary')).toBeTruthy()
  })

  it('should change style based on prop', () => {
    const { getByTestId, component } = render(Button)
    const button = getByTestId('button')
    expect(button).toBeDefined()

    const setStyleAndConfirm = async (style: ButtonVariant) => {
      component.$set({ variant: style })
      await tick()
      return button.classList.contains(style)
    }

    expect(setStyleAndConfirm(ButtonVariant.Primary)).toBeTruthy()
    expect(setStyleAndConfirm(ButtonVariant.Secondary)).toBeTruthy()
    expect(setStyleAndConfirm(ButtonVariant.Success)).toBeTruthy()
    expect(setStyleAndConfirm(ButtonVariant.Info)).toBeTruthy()
    expect(setStyleAndConfirm(ButtonVariant.Warning)).toBeTruthy()
    expect(setStyleAndConfirm(ButtonVariant.Danger)).toBeTruthy()
    expect(setStyleAndConfirm(ButtonVariant.Link)).toBeTruthy()
  })
})
