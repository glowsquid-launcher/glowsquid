<script lang="ts">
	import { addInstanceModalActive } from '$lib/stores'
	import {
	ComposedModal,FormGroup,
	ModalBody,
	ModalFooter,
	ModalHeader,TextInput
	} from 'carbon-components-svelte'

	let name
	let version
	let loader
	let loaderVersion
	let description
	const submit = () => addInstanceModalActive.set(false)
</script>

<!-- when submitting this is set to false, so this is for debugger statements -->
{@debug $addInstanceModalActive}

<ComposedModal open={$addInstanceModalActive} on:submit={submit}>
	<ModalHeader label="instances" title="Add instance" />
	<ModalBody hasForm>
		<FormGroup legendText="Basic information">
			<TextInput id="name" placeholder="Name" bind:value={name} />
			<TextInput id="description" placeholder="Description" bind:value={description} />
		</FormGroup>
	</ModalBody>
	<ModalFooter primaryButtonText="Proceed" />
</ComposedModal>
