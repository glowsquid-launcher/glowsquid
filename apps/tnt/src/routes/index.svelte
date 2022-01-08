<script lang="ts">
	import { animationTime } from '$lib/animations'
	import QuickLaunchCard from '$lib/components/QuickLaunchCard.svelte'
	import { addInstanceModalActive } from '$lib/stores'
	import { PieChart } from '@carbon/charts-svelte'
	import { Button } from 'carbon-components-svelte'
	import { Add32, Fire32, FolderOpen32, Tools32 } from 'carbon-icons-svelte'
	import { fade, fly, slide } from 'svelte/transition'

	// get the last 4 used instances
	let quickLaunchList = [0, 1, 2, 3]
	let instanceTimeData = [
		{
			group: 'test',
			value: 10
		},
		{
			group: 'AOF3',
			value: 20
		}
	]

	let opts = {
		title: 'Instance Times',
		resizable: true,
		legend: {
			alignment: 'center'
		},
		pie: {
			alignment: 'center'
		},
		height: '600px'
	}

	const importCurseforge = () => {}

	const importModrinth = () => {}

	const importZip = () => {}

	const createNewInstance = () => {
		addInstanceModalActive.set(true)
	}
</script>

<div class="flex flex-col content-center">
	<div class="w-4/5 self-center">
		<section>
			<h2 transition:fade={{ duration: 300 }}>Quick launch</h2>
			<div class="grid grid-cols-[repeat(auto-fill, 366px)] lg:grid-cols-4 gap-6">
				{#each quickLaunchList as _, i}
					<div
						transition:slide={{
							delay: i * (animationTime / quickLaunchList.length) - 200
						}}
					>
						<QuickLaunchCard
							title="test"
							version="1.18.1"
							id="instance-id"
							on:launch={console.log}
						/>
					</div>
				{/each}
			</div>
		</section>

		<hr
			class="mt-5 mb-5"
			in:fly={{ duration: animationTime, x: -1000 }}
			out:fly={{ duration: animationTime, x: 1000 }}
		/>

		<section>
			<h2 transition:fade={{ duration: animationTime }} class="mb-6">Add New Instance</h2>

			<div class="flex gap-2">
				<div transition:slide={{ duration: animationTime }}>
					<Button
						class="mt-1"
						icon={Fire32}
						iconDescription="Curseforge"
						on:click={importCurseforge}>Add from Curseforge</Button
					>
				</div>

				<div transition:slide={{ duration: animationTime, delay: 100 }}>
					<Button class="mt-1" icon={Tools32} iconDescription="Modrinth" on:click={importModrinth}
						>Add from Modrinth</Button
					>
				</div>

				<div transition:slide={{ duration: animationTime, delay: 200 }}>
					<Button class="mt-1" icon={FolderOpen32} iconDescription="ZIP" on:click={importZip}
						>Add from ZIP</Button
					>
				</div>

				<div transition:slide={{ duration: animationTime, delay: 300 }}>
					<Button class="mt-1" icon={Add32} iconDescription="New" on:click={createNewInstance}
						>New Instance</Button
					>
				</div>
			</div>
		</section>

		<hr
			class="mt-5 mb-5"
			in:fly={{ duration: animationTime, x: -1000 }}
			out:fly={{ duration: animationTime, x: 1000 }}
		/>

		<section transition:fade={{ duration: animationTime }}>
			<h2 class="mb-6">Playtime stats</h2>
			<PieChart data={instanceTimeData} options={opts} />
		</section>
	</div>
</div>
