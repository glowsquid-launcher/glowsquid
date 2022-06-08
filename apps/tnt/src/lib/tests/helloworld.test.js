/* eslint-disable no-undef */
describe('test', () => {
  it('should have a title', async () => {
    const header = await $('header > div > h1')
    const text = await header.getText()
    expect(text).toMatch('Glowsquid')
  })
})

export { }
