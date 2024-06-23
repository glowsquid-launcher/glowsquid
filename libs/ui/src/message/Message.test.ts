/// <reference types="@testing-library/jest-dom" />

import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/svelte/svelte5'
import Message from './Message.svelte'

describe('message component', () => {
    it('can load the component', () => {
        render(Message, { sponsor: 'absolutely no one' })

        const greeting = screen.getByText(/This app/u)

        expect(greeting).toBeInTheDocument()
        expect(greeting.innerHTML).toBe('This app is sponsored by absolutely no one!')
    })
})
