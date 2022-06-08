<script lang="ts">
  export let username: string
  export let id: string

  export let imageDimentions = {
    width: 32,
    height: 32,
  }
  let clazz = ''
  export { clazz as class }
</script>

<div
  class="flex flex-row items-center gap-2 text-center text-white text-lg rounded-lg {clazz} "
>
  <img
    class="rounded-lg"
    src="https://crafatar.com/avatars/{id}"
    alt={username}
    {...imageDimentions}
  />
  <span>{username}</span>
</div>
