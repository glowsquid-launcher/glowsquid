<script lang="ts">
  import {
    Dialog,
    DialogOverlay,
    DialogTitle,
    Transition,
    TransitionChild,
  } from '@rgossiaux/svelte-headlessui'
  import { createEventDispatcher } from 'svelte'
  import { ColorVariant } from '$lib/types'

  /**
   * Is the modal opened?
   */
  export let isOpened: boolean
  export let variant: ColorVariant = ColorVariant.Primary

  function getHeaderColor(variant: ColorVariant) {
    switch (variant) {
      case ColorVariant.Primary:
        return ColorVariant.Secondary
      case ColorVariant.Secondary:
        return ColorVariant.Primary
      default:
        return variant
    }
  }

  function getBgCol(variant: ColorVariant) {
    switch (variant) {
      case ColorVariant.Secondary:
        return ColorVariant.Secondary
      default:
        return ColorVariant.Primary
    }
  }

  $: headerCol = getHeaderColor(variant)
  $: bgCol = getBgCol(variant)

  let dispatch = createEventDispatcher<{
    close: void
  }>()
</script>

<Transition show={isOpened}>
  <Dialog on:close={() => dispatch('close')}>
    <div class="fixed inset-0 z-10 overflow-y-auto">
      <div
        class="align-center min-h-screen flex items-end justify-center px-4 pb-20 pt-4 text-center sm:block sm:p-0"
      >
        <TransitionChild
          enter="ease-out duration-300"
          enterFrom="opacity-0"
          enterTo="opacity-75"
          leave="ease-in duration-200"
          leaveFrom="opacity-75"
          leaveTo="opacity-0"
          entered="opacity-75"
        >
          <DialogOverlay class="fixed inset-0 bg-gray-500 transition-opacity" />
        </TransitionChild>

        <TransitionChild
          enter="ease-out transform duration-300"
          enterFrom="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
          enterTo="opacity-100 translate-y-0 sm:scale-100"
          leave="ease-in transform duration-200"
          leaveFrom="opacity-100 translate-y-0 sm:scale-100"
          leaveTo="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
        >
          <!-- This element is to trick the browser into centering the modal contents. -->
          <span
            class="hidden sm:inline-block sm:align-middle sm:h-screen"
            aria-hidden="true"
          >
            &#8203;
          </span>

          <div
            class="inline-block transform overflow-hidden rounded-lg bg-{bgCol} align-bottom text-left text-white text-lg shadow-xl transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full"
          >
            <div class="mb-2 rounded bg-{headerCol} px-6 py-4">
              <DialogTitle
                as="h3"
                class="font-medium leading-6 text-3xl text-left"
              >
                <slot name="title" />
              </DialogTitle>
            </div>
            <div class="pa-4">
              <slot />
              <hr class="mx-auto my-4 divide-grey-200" />
              <div class="flex justify-end rounded">
                <slot name="buttons" />
              </div>
            </div>
          </div>
        </TransitionChild>
      </div>
    </div>
  </Dialog>
</Transition>
