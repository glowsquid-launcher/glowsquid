<script lang="ts">
	import 'carbon-components-svelte/css/all.css';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Button } from 'carbon-components-svelte';
</script>

<Button on:click={() => invoke('my_custom_command')}>haha yes</Button>
