<script lang="ts">
    import { onMount } from "svelte";
    import GameBoard from "$lib/components/GameBoard.svelte";

    let title = "Codenames - Spymaster";

    onMount(() => {
        document.title = "Codenames - Spymaster";
    });
</script>

<svelte:head>
    <title>{title}</title>
    <meta name="description" content="Play Codenames - Spymaster board view" />
</svelte:head>

<main class="min-h-screen">
    <GameBoard mode="spymaster" />
</main>
