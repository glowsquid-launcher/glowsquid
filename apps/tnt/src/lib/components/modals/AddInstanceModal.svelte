<script lang="ts">
  import { addInstanceModalActive } from '$lib/stores';
  import {
    ComposedModal,
    FormGroup,
    ModalBody,
    ModalFooter,
    ModalHeader,
    TextInput,
  } from 'carbon-components-svelte';

  let name: string;
  let version: string;
  let loader: string;
  let loaderVersion: string;
  let description: string;
  const submit = () => addInstanceModalActive.set(false);
</script>

<ComposedModal
  open={$addInstanceModalActive}
  on:close={() => addInstanceModalActive.set(false)}
  on:submit={submit}
>
  <ModalHeader label="Instances" title="Add instance" />
  <ModalBody hasForm>
    <FormGroup legendText="Basic information">
      <TextInput id="name" placeholder="Name" bind:value={name} />
      <TextInput
        id="description"
        placeholder="Description"
        bind:value={description}
      />
    </FormGroup>
  </ModalBody>
  <ModalFooter primaryButtonText="Proceed" />
</ComposedModal>
