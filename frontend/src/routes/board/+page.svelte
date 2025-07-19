<!-- src/routes/board/+page.svelte -->
<script lang="ts">
    import { onMount } from "svelte";
    import { page } from "$app/stores";
    import GameBoard from "$lib/components/GameBoard.svelte";

    // Page metadata
    let title = "Codenames - Board";

    // You could add query parameters for additional features
    $: searchParams = $page.url.searchParams;
    $: gameId = searchParams.get("game") || "default";
    $: spectatorMode = searchParams.get("spectator") === "true";

    onMount(() => {
        // Update page title
        document.title = title;
    });
</script>

<!-- SEO Head -->
<svelte:head>
    <title>{title}</title>
    <meta name="description" content="Play Codenames - Team board view" />
</svelte:head>

<main class="min-h-screen bg-gray-50 px-4 py-8">
    <div class="mx-auto max-w-6xl">
        <!-- Header -->
        <header class="mb-8 text-center">
            <h1 class="mb-2 text-4xl font-bold text-gray-900">Codenames</h1>
            <p class="text-lg text-gray-600">Team Board View</p>

            <!-- Game info -->
            {#if gameId !== "default"}
                <div class="mt-4">
                    <span
                        class="inline-block rounded-full bg-blue-100 px-3 py-1 text-sm font-medium text-blue-800"
                    >
                        Game: {gameId}
                    </span>
                </div>
            {/if}
        </header>

        <!-- Navigation -->
        <nav class="mb-8 flex justify-center gap-4">
            <a
                href="/board"
                class="rounded-lg bg-blue-500 px-4 py-2 font-medium text-white shadow-sm"
                class:opacity-75={spectatorMode}
            >
                Team View
            </a>
            <a
                href="/spymaster{gameId !== 'default' ? `?game=${gameId}` : ''}"
                class="rounded-lg bg-purple-500 px-4 py-2 font-medium text-white shadow-sm transition-colors hover:bg-purple-600"
            >
                Spymaster View
            </a>
            <a
                href="/board?spectator=true{gameId !== 'default' ? `&game=${gameId}` : ''}"
                class="rounded-lg bg-gray-500 px-4 py-2 font-medium text-white shadow-sm transition-colors hover:bg-gray-600"
                class:bg-gray-700={spectatorMode}
            >
                Spectator
            </a>
        </nav>

        <!-- Board Component -->
        <div class="rounded-xl bg-white p-6 shadow-sm lg:p-8">
            <GameBoard mode="public" allowClicks={!spectatorMode} />
        </div>

        <!-- Instructions -->
        <div class="mx-auto mt-8 max-w-2xl">
            <details class="rounded-lg bg-white shadow-sm">
                <summary
                    class="cursor-pointer rounded-lg px-6 py-4 text-lg font-medium text-gray-900 hover:bg-gray-50"
                >
                    How to Play
                </summary>
                <div class="space-y-3 px-6 pb-6 text-gray-600">
                    <p>
                        <strong>Team View:</strong> You can see all the words on the board. Cards that
                        haven't been revealed yet appear in white. Click on a card to reveal it.
                    </p>
                    <p>
                        <strong>Goal:</strong> Your spymaster will give you clues to help you identify
                        which words belong to your team. Avoid the other team's words, neutral words,
                        and especially the assassin word!
                    </p>
                    <p>
                        <strong>Card Colors:</strong>
                    </p>
                    <ul class="ml-6 space-y-1">
                        <li>
                            <span class="mr-2 inline-block h-3 w-3 rounded bg-red-500"></span>Red
                            team words
                        </li>
                        <li>
                            <span class="mr-2 inline-block h-3 w-3 rounded bg-blue-500"></span>Blue
                            team words
                        </li>
                        <li>
                            <span class="mr-2 inline-block h-3 w-3 rounded bg-gray-500"
                            ></span>Neutral words
                        </li>
                        <li>
                            <span class="mr-2 inline-block h-3 w-3 rounded bg-black"></span>Assassin
                            word (avoid!)
                        </li>
                    </ul>
                    <p class="text-sm text-gray-500">
                        The connection indicator shows if you're receiving live updates from other
                        players.
                    </p>
                </div>
            </details>
        </div>
    </div>
</main>

<!-- Footer -->
<footer class="mt-16 pb-8 text-center text-sm text-gray-500">
    <p>
        Codenames • Digital version •
        <a href="/spymaster" class="text-blue-600 hover:text-blue-700">Spymaster View</a>
    </p>
</footer>
