import { render } from '@testing-library/svelte'
import { writable, type Writable } from 'svelte/store'
import Dropdown from './Dropdown.svelte'

describe('Dropdown', () => {
  it('should render', () => {
    // TODO: test slots
    const store: Writable<string | null> = writable(null)

    const { getByTestId } = render(Dropdown, {
      props: {
        selected: store,
        options: [1, 2, 3],
      },
    })
    const button = getByTestId('dropdown')
    expect(button).toBeDefined()
  })
})
