<script lang="ts">
	import { animationTime } from '$lib/animations'
	import { Button, Tile } from 'carbon-components-svelte'
	import { Play32, Settings32 } from 'carbon-icons-svelte'
	import { fly } from 'svelte/transition'

	export let title
	export let version
	export let id

	function launch() {
		console.log(`Launching instance ${id}`)
	}
</script>

<Tile class="w-full h-32 flex flex-row">
	<header class="justify-self-start self-center mr-auto h-min">
		<h1 transition:fly={{ duration: animationTime, x: -50 }} class="text-size-3rem text-left pb-2">
			{title}
		</h1>
		<h2 transition:fly={{ duration: animationTime, x: -50, delay: 100 }} class="text-size-1rem">
			{version}
		</h2>
	</header>

	<div transition:fly={{ duration: animationTime, y: -100 }} class="h-full">
		<Button icon={Play32} iconDescription="Launch" on:click={launch} class="h-full" />
	</div>
	<div transition:fly={{ duration: animationTime, y: -100, delay: 100 }} class="h-full">
		<Button icon={Settings32} iconDescription="Settings" href={`/instances/${id}`} class="h-full" />
	</div>
</Tile>
