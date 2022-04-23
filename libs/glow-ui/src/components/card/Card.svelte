<script lang="ts">
  import type { ButtonType } from '../..';
  interface Button {
    icon?: string;
    label?: string;
    type?: ButtonType;
    action?: () => void | Promise<void> | PromiseLike<void>;
  }

  export let title: string;
  export let body: string;
  export let buttons: Button[];
</script>
