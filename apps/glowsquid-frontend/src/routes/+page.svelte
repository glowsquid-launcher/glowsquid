<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import { Input, Message, Button } from '@repo/ui'

    let name = $state('')
    let greetMsg = $state('')

    async function greet(e: Event) {
        e.preventDefault()

        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        greetMsg = await invoke('greet', { name })
    }
</script>

<div class="container">
    <h1>Welcome to Glowsquid!</h1>

    <form class="row" onsubmit={greet}>
        <Input placeholder="Enter a name..." bind:value={name} />
        <Button type="submit">Greet</Button>
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

        gap: var(--size-sm);
    }

    h1 {
        text-align: center;
    }
</style>
