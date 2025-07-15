<script lang="ts">
    import { onMount } from "svelte";

    // Types matching your backend
    type Team = "red" | "blue" | "neutral" | "assassin";

    interface Card {
        word: string;
        type: Team;
    }

    let board: Card[][] = [];
    let loading = true;
    let error: string | null = null;

    onMount(async () => {
        try {
            const response = await fetch("http://localhost:8080/api/board/public");
            if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
            board = await response.json();
        } catch (err) {
            error = err instanceof Error ? err.message : "Unknown error occurred";
        } finally {
            loading = false;
        }
    });

    // Team color mapping
    const teamColors: Record<Team, string> = {
        red: "bg-red-500 text-white",
        blue: "bg-blue-500 text-white",
        neutral: "bg-amber-200 text-gray-800",
        assassin: "bg-gray-900 text-white"
    };

    function getCardClass(card: Card): string {
        return (
            teamColors[card.team] +
            " " +
            (card.revealed ? "opacity-50" : "hover:scale-105 shadow-md")
        );
    }
</script>

<div class="p-4">
    <h1 class="mb-4 text-2xl font-bold">Board Debug View</h1>

    {#if loading}
        <div class="py-8 text-center">Loading board data...</div>
    {:else if error}
        <div class="rounded border border-red-400 p-4 text-red-500">
            Error: {error}
            <div class="mt-2 text-sm">
                Make sure your Actix backend is running at localhost:8080
            </div>
        </div>
    {:else}
        <div class="mx-auto grid max-w-2xl grid-cols-5 gap-2">
            {#each board as row, rowIndex}
                {#each row as card, colIndex}
                    <div
                        class="{getCardClass(
                            card
                        )} flex h-24 cursor-pointer items-center justify-center rounded border p-2 text-center transition-all"
                    >
                        <div>
                            <div class="font-bold">{card.word}</div>
                            <div class="mt-1 text-xs">{card.type}</div>
                        </div>
                    </div>
                {/each}
            {/each}
        </div>

        <div class="mx-auto mt-6 max-w-2xl rounded bg-gray-100 p-4">
            <h2 class="mb-2 font-bold">Debug Info:</h2>
            <pre class="text-xs">{JSON.stringify(board, null, 2)}</pre>
        </div>
    {/if}
</div>
