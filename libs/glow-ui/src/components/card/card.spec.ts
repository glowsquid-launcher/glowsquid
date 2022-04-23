import Card from './Card.svelte'
import { render } from '@testing-library/svelte'

it('it works', async () => {
  const { getByText } = render(Card)

  expect(getByText('Hello component!'));
})
