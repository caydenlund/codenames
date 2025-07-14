<script lang="ts">
    import { onMount } from "svelte";
    import { page } from "$app/state";
    import { pushState } from "$app/navigation";
    import { slide } from "svelte/transition";

    import { backendUrl } from "$lib/util/backendUrl";

    let getBoardPromise: Promise<any> | null = $state(null);

    async function getBoard() {
        const url = backendUrl("/api/board");
        const response = await fetch(url);
        if (!response.ok) throw new Error(`Get board request failed: ${response.statusText}`);
        return await response.json();
    }

    onMount(() => {
        getBoardPromise = getBoard();
    })
</script>

<h1>board</h1>

{#if page.state.board}
    <p>{page.state.board}</p>
{/if}
