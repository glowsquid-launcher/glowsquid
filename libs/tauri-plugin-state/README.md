# Tauri Plugin state

## A Stupid simple state management plugin for Tauri

Let's say you want to manage state between your frontend and backend, but you want to keep it in sync and _completely_ managed from rust.

This plugin is for you.

## Frontend Framework guides

By default, we use a simple `Map` object to store state on the frontend.
If you need reactivity, frameworks have their own reactive Map implementations that you can use.

- Svelte: `import { Map } from 'svelte/reactivity';`
- Vue: You can wrap a map in a `reactive` object. `setupState(reactive(new Map()));`
- SolidJS: You can use <https://primitives.solidjs.community/package/map> to create a reactive map. `import { ReactiveMap } from "@solid-primitives/map";`
- React: N/A. No idea how to do this in React. Make a PR if you know how.

## Manually watching for state changes

All state changes are broadcasted to the frontend via Tauri's native event API. You can listen for these changes by listening to the `plugin:state-change` event.

```js
import { listen } from '@tauri-apps/api/event'

listen<{ key: string, value: unknown }>('plugin:state-change', (event) => {
    const { key, value } = event.payload
    console.log(`State change for key ${key} with value ${value}`)
})
```

How you type `value` is up to you. We recommend using a discriminated union to type the value.

## Panic guide

This plugin considers not being able to sync state as an invariant violation and will panic if it can't sync state. We always want to be in sync to make sure nothing goes wrong.
