<script lang="ts">
    /**
     * The name of the icon. E.g. home
     */
    export let name: string;
    /**
     * The icon set to use.
     * @default pixelarticons
     */
    export let set = 'pixelarticons'

    /**
     * The width of the icon
     * @default 24
     */
    export let width = 24
    /**
     * The height of the icon
     * @default 24
     */
    export let height = 24
    /**
     * Should the icon be inline with text?
     *
     * @default true
     */
    export let inline = true;
</script>

<iconify-icon icon="{set}:{name}" {inline} {width} {height} />
