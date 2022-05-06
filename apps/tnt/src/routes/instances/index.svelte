<script lang="ts">import { animationTime } from '$lib/animations'
import InstanceCard from '$lib/components/InstanceCard.svelte'
import { slide } from 'svelte/transition'

// get ALL instances
const instanceList = [{ id: 0 }, { id: 1 }, { id: 2 }, { id: 3 }, { id: 4 }, { id: 5 }, { id: 6 }]
</script>

<div class="flex flex-col">
	<section class="w-4/5 self-center">
		<div class="grid grid-cols-[repeat(auto-fill, 366px)] lg:grid-cols-4 gap-6">
			{#each instanceList as instance, i}
				<div transition:slide={{ delay: i * (animationTime / instanceList.length) }}>
					<InstanceCard title="test" version="1.18.1" id={instance.id.toString()} />
				</div>
			{/each}
		</div>
	</section>
</div>
