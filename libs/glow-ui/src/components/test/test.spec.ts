import Test from './Test.svelte'
import { render } from '@testing-library/svelte'

it('it works', async () => {
  const { getByText } = render(Test)

  expect(getByText('Hello component!'));
})
