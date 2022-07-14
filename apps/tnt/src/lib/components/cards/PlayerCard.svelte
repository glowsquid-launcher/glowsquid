<script lang="ts">
  export let username: string
  export let avatar: string

  // eslint-disable-next-line unused-export-let
  export let id: string = ''
  export let meta = {}
  export let raw_id = ''

  export let imageDimentions = {
    width: 32,
    height: 32,
  }
  let clazz = ''
  export { clazz as class }
</script>

<div
  class="flex flex-row items-center gap-2 text-center text-white text-lg rounded-lg {clazz}"
>
  <img class="rounded-lg" src={avatar} alt={username} {...imageDimentions} />
  <span>{username}</span>
</div>
