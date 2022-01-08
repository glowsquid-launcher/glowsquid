<script lang="ts">
	import {
	Accordion,
	AccordionItem,
	Button,
	NumberInput,
	TextInput
	} from 'carbon-components-svelte'
	import { ArrowLeft32,Play32 } from 'carbon-icons-svelte'

	// have all of this passed in somehow
	export let id = 0
	export let name = 'test'
	export let memory = {
		min: 1000,
		max: 5000
	}

	function launch() {
		console.log(`launching instance with id:  ${id}`)
	}
</script>

<div class="flex flex-col">
	<span class="mb-5">
		<Button
			class="inline-block mt-2 ml-6"
			icon={ArrowLeft32}
			iconDescription="Back"
			on:click={() => window.history.back()}
		/>
		<h1 class="text-5xl ml-6 inline-block mr-5">{name}</h1>
		<Button
			class="inline-block mt-2 float-right mr-6"
			icon={Play32}
			iconDescription="Launch"
			on:click={launch}>Launch</Button
		>
	</span>
	<Accordion>
		<AccordionItem title={'Main Settings'} open>
			<!-- Can't have an empty instance name! -->
			<TextInput
				invalid={name == ''}
				invalidText="Instance must have a name."
				labelText="Instance name"
				bind:value={name}
			/>

			<!-- dedicated wam -->
			<div class="mt-3 mb-5">
				<NumberInput
					label="Minimum RAM"
					invalidText="You cannot have less than 1GB of RAM (We dont even reccomend 1GB)"
					step={100}
					bind:value={memory.min}
					min={1000}
				/>
			</div>
			<div>
				<NumberInput
					label="Maximum RAM"
					invalidText="Max RAM must be more than minimum RAM"
					step={100}
					bind:value={memory.max}
					min={memory.min}
				/>
			</div>
		</AccordionItem>

		<AccordionItem title={'Advanced Settings'}>
			<Button kind="tertiary" class="mt-3 mr-3">Export Instance</Button>
			<Button kind="danger-tertiary" class="mt-3">Delete Instance</Button>
		</AccordionItem>
	</Accordion>
</div>
