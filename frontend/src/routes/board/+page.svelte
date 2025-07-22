<script lang="ts">
    import { onMount } from "svelte";
    import GameBoard from "$lib/components/GameBoard.svelte";

    let title = "Codenames - Board";

    onMount(() => {
        document.title = "Codenames - Board";
    });
</script>

<svelte:head>
    <title>{title}</title>
    <meta name="description" content="Play Codenames - Team board view" />
</svelte:head>

<main class="min-h-screen">
    <GameBoard mode="public" />
</main>
