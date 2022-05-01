/* eslint-disable no-unused-vars */
import '@sveltejs/kit'

declare namespace svelte.JSX {
  import { AttributifyAttributes } from '@unocss/preset-attributify'
  type HTMLAttributes<T> = AttributifyAttributes;
}
