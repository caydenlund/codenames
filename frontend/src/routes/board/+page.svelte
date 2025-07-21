<!-- src/routes/board/+page.svelte -->
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

<main class="min-h-screen bg-gray-50 px-4 py-8">
    <div class="mx-auto max-w-6xl">
        <header class="mb-8 text-center">
            <h1 class="mb-2 text-4xl font-bold text-gray-900">Codenames</h1>
            <p class="text-lg text-gray-600">Team Board View</p>
        </header>

        <nav class="mb-8 flex justify-center gap-4">
            <a
                href="/board"
                class="rounded-lg bg-blue-500 px-4 py-2 font-medium text-white shadow-sm"
            >
                Team View
            </a>
            <a
                href="/spymaster"
                class="rounded-lg bg-purple-500 px-4 py-2 font-medium text-white shadow-sm transition-colors hover:bg-purple-600"
            >
                Spymaster View
            </a>
        </nav>

        <div class="rounded-xl bg-white p-6 shadow-sm lg:p-8">
            <GameBoard mode="public" />
        </div>
    </div>
</main>
