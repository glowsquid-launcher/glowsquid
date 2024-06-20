<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { Message } from '@repo/ui';

	let name = '';
	let greetMsg = '';

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		greetMsg = await invoke('greet', { name });
	}
</script>

<div class="container">
	<h1>Welcome to Glowsquid!</h1>

	<form class="row" on:submit|preventDefault={greet}>
		<input id="greet-input" placeholder="Enter a name..." bind:value={name} />
		<button type="submit">Greet</button>
	</form>

	<p>{greetMsg}</p>

	<Message sponsor={name} />
</div>

<style>
	.container {
		padding-top: 10vh;
		display: flex;
		flex-direction: column;
		justify-content: center;
		text-align: center;
        gap: var(--size-lg);
	}

	.row {
		display: flex;
		justify-content: center;
	}

	h1 {
		text-align: center;
	}

	input,
	button {
		border-radius: 8px;
		border: 1px solid transparent;
		padding: 0.6em 1.2em;
		font-size: 1em;
		font-weight: 500;
		font-family: inherit;
		color: #0f0f0f;
		background-color: #ffffff;
		transition: border-color 0.25s;
		box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
	}


	input,
	button {
		outline: none;
	}

	#greet-input {
		margin-right: 5px;
	}
</style>
