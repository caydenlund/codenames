<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import {
        boardStore,
        type BoardMode,
        type PublicCard,
        type SpymasterCard
    } from "$lib/stores/boardStore";

    export let mode: BoardMode = "public";

    let storeState = boardStore.store;
    let unsubscribeStore: () => void;
    let previousConnectedState = false;
    let hasConnectedOnce = false;

    onMount(() => {
        // Subscribe to store updates
        unsubscribeStore = storeState.subscribe((state) => {
            // Track if we've ever been connected
            if (state.connected && !hasConnectedOnce) {
                hasConnectedOnce = true;
            }

            // Auto-dismiss errors when reconnected
            if (state.connected && !previousConnectedState && state.error) {
                boardStore.clearError();
            }
            previousConnectedState = state.connected;
        });

        // Initialize the board
        boardStore.initialize(mode);
    });

    onDestroy(() => {
        if (unsubscribeStore) unsubscribeStore();
        boardStore.destroy();
    });

    function handleCardClick(row: number, col: number) {
        if (mode === "spymaster") {
            boardStore.revealCard(row, col);
        }
    }

    function getCardClasses(card: PublicCard | SpymasterCard): string {
        let classes = [
            "w-full",
            "sm:h-25",
            "lg:h-30",
            "rounded-lg",
            "transition-all",
            "duration-200",
            "flex",
            "items-center",
            "justify-center",
            "text-center",
            "p-2",
            "sm:text-xl",
            "lg:text-2xl",
            "shadow-sm"
        ];

        // Team-based styling
        if (mode === "spymaster" && "revealed" in card && card.revealed) {
            classes.push("italic");
            if (card.team === "red") {
                classes.push("bg-red-300", "text-gray-100");
            } else if (card.team === "blue") {
                classes.push("bg-blue-300", "text-gray-100");
            } else if (card.team === "assassin") {
                classes.push("bg-gray-600", "text-gray-100");
            } else if (card.team === "neutral") {
                classes.push("bg-amber-50", "text-gray-400");
            } else if (card.team === "unknown") {
                classes.push("bg-gray-100", "text-gray-800");
            }
        } else {
            classes.push("font-medium", "border-2");
            if (card.team === "red") {
                classes.push("bg-red-500", "text-white", "border-red-700");
            } else if (card.team === "blue") {
                classes.push("bg-blue-500", "text-white", "border-blue-700");
            } else if (card.team === "assassin") {
                classes.push("bg-black", "text-white", "border-gray-700");
            } else if (card.team === "neutral") {
                classes.push("bg-amber-100", "text-black", "border-amber-300");
            } else if (card.team === "unknown") {
                classes.push("bg-gray-100", "text-gray-800", "border-gray-300");
            }
        }

        // Interactive styling for spymaster mode
        if (mode === "spymaster") {
            classes.push("cursor-pointer", "hover:shadow-md", "hover:scale-105", "active:scale-95");
        }

        return classes.join(" ");
    }
</script>

<div class="mx-auto p-5 lg:p-10">
    <div class="flex items-center justify-between lg:mb-4">
        <a href="/" class="text-2xl text-gray-600 transition-all duration-200 hover:text-black"
            >Back</a
        >
        <div class="text-gray-600 capitalize">
            {mode} View
        </div>
        <div class="flex items-center space-x-2">
            <div
                class={`h-3 w-3 rounded-full ${
                    $storeState.connected
                        ? "bg-green-500"
                        : hasConnectedOnce
                          ? "bg-red-500"
                          : "bg-yellow-500"
                }`}
            ></div>
            <span class="font-medium text-gray-700">
                {$storeState.connected
                    ? "Connected"
                    : hasConnectedOnce
                      ? "Disconnected"
                      : "Connecting..."}
            </span>
        </div>
    </div>

    {#if $storeState.error}
        <div class="mb-4 rounded-lg border border-red-300 bg-red-100 p-3 text-red-700">
            {$storeState.error}
            <button
                class="ml-2 text-red-600 underline hover:text-red-800"
                on:click={() => boardStore.clearError()}
            >
                Dismiss
            </button>
        </div>
    {/if}

    <!-- Loading State -->
    {#if $storeState.loading}
        <div class="flex h-96 items-center justify-center">
            <div class="h-12 w-12 animate-spin rounded-full border-b-2 border-blue-500"></div>
        </div>
    {:else if $storeState.board && Array.isArray($storeState.board) && $storeState.board.length > 0}
        <!-- Game Board Grid -->
        <div class="grid grid-cols-2 gap-3 rounded-xl bg-white p-6 sm:grid-cols-5">
            {#each $storeState.board as row, rowIndex (rowIndex)}
                {#each row as card, colIndex (colIndex)}
                    <button
                        class={getCardClasses(card)}
                        on:click={() => handleCardClick(rowIndex, colIndex)}
                        disabled={mode !== "spymaster"}
                    >
                        <span class="leading-tight break-words">
                            {card.word}
                        </span>
                    </button>
                {/each}
            {/each}
        </div>
        {#if mode === "spymaster"}
            <div class="w-full p-5">
                <button
                    class="w-full cursor-pointer rounded-xl bg-red-600 p-5 text-center text-xl font-bold text-white transition-all hover:scale-101 active:scale-99"
                    on:click={() => {
                        boardStore.newGame();
                    }}
                >
                    New Game
                </button>
            </div>
        {/if}
    {:else}
        <!-- Empty State -->
        <div class="flex h-96 items-center justify-center rounded-xl bg-gray-50">
            <div class="text-center">
                <div class="mb-2 text-lg text-gray-400">No board data available</div>
                <button
                    class="rounded-lg bg-blue-500 px-4 py-2 text-white hover:bg-blue-600"
                    on:click={() => boardStore.initialize(mode)}
                >
                    Retry
                </button>
            </div>
        </div>
    {/if}
</div>

<style>
    button:disabled {
        cursor: default;
    }

    button.relative {
        position: relative;
    }
</style>
