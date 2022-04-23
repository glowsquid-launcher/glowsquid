import Card from './Card.svelte';
import { render } from '@testing-library/svelte';

it('it works', () => {
  const { getByText } = render(Card);

  expect(getByText('Hello component!'));
});
