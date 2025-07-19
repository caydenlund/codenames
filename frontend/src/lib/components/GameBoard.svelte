<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { boardStore, board, connected } from "$lib/stores/boardStore";
    import type { BoardMode, PublicCard, SpymasterCard } from "$lib/stores/boardStore";

    export let mode: BoardMode = "public";
    export let allowClicks: boolean = true;

    // Reactive declarations
    $: boardState = $board;
    $: isConnected = $connected;
    $: isLoading = boardState.loading;
    $: error = boardState.error;
    $: boardData = boardState.board;

    onMount(async () => {
        await boardStore.initialize(mode);
    });

    onDestroy(() => {
        boardStore.destroy();
    });

    // Handle card clicks
    function handleCardClick(row: number, col: number) {
        if (!allowClicks || isLoading || !isConnected) return;

        const card = boardData[row]?.[col];
        if (!card) return;

        // For public mode, don't allow clicking already revealed cards
        if (mode === "public" && card.team !== "unknown") return;

        // For spymaster mode, don't allow clicking already revealed cards
        if (mode === "spymaster" && (card as SpymasterCard).revealed) return;

        boardStore.revealCard(row, col);
    }

    // Get card styling based on team and mode
    function getCardClass(card: PublicCard | SpymasterCard): string {
        const baseClasses =
            "relative flex items-center justify-center min-h-[80px] p-4 rounded-lg border-2 font-medium text-center cursor-pointer transition-all duration-200 hover:scale-105 hover:shadow-lg";

        if (mode === "spymaster") {
            const spymasterCard = card as SpymasterCard;
            const isRevealed = spymasterCard.revealed;

            // Spymaster can see all teams, with different styling for revealed cards
            switch (spymasterCard.team) {
                case "red":
                    return `${baseClasses} ${isRevealed ? "bg-red-600 text-white border-red-700" : "bg-red-100 text-red-800 border-red-300"} ${isRevealed ? "opacity-75" : ""}`;
                case "blue":
                    return `${baseClasses} ${isRevealed ? "bg-blue-600 text-white border-blue-700" : "bg-blue-100 text-blue-800 border-blue-300"} ${isRevealed ? "opacity-75" : ""}`;
                case "neutral":
                    return `${baseClasses} ${isRevealed ? "bg-gray-600 text-white border-gray-700" : "bg-gray-100 text-gray-800 border-gray-300"} ${isRevealed ? "opacity-75" : ""}`;
                case "assassin":
                    return `${baseClasses} ${isRevealed ? "bg-black text-white border-gray-900" : "bg-gray-900 text-white border-black"}`;
                default:
                    return `${baseClasses} bg-white text-gray-800 border-gray-300`;
            }
        } else {
            const publicCard = card as PublicCard;

            // Public view - only show revealed teams
            switch (publicCard.team) {
                case "red":
                    return `${baseClasses} bg-red-500 text-white border-red-600`;
                case "blue":
                    return `${baseClasses} bg-blue-500 text-white border-blue-600`;
                case "neutral":
                    return `${baseClasses} bg-gray-500 text-white border-gray-600`;
                case "assassin":
                    return `${baseClasses} bg-black text-white border-gray-900`;
                case "unknown":
                default:
                    return `${baseClasses} bg-white text-gray-800 border-gray-300 hover:bg-gray-50`;
            }
        }
    }

    // Check if a card is clickable
    function isCardClickable(card: PublicCard | SpymasterCard): boolean {
        if (!allowClicks || isLoading || !isConnected) return false;

        if (mode === "public") {
            return (card as PublicCard).team === "unknown";
        } else {
            return !(card as SpymasterCard).revealed;
        }
    }

    // Clear error when user clicks the error message
    function clearError() {
        boardStore.clearError();
    }
</script>

<!-- Connection status indicator -->
<div class="mb-4 flex items-center gap-2">
    <div class="flex items-center gap-2">
        <div
            class="h-3 w-3 rounded-full {isConnected ? 'bg-green-500' : 'bg-red-500'}"
            class:animate-pulse={!isConnected}
        ></div>
        <span class="text-sm font-medium {isConnected ? 'text-green-700' : 'text-red-700'}">
            {isConnected ? "Connected" : "Disconnected"}
        </span>
    </div>

    <!-- Mode indicator -->
    <div class="ml-auto">
        <span
            class="rounded-full px-2 py-1 text-xs font-medium {mode === 'spymaster'
                ? 'bg-purple-100 text-purple-800'
                : 'bg-blue-100 text-blue-800'}"
        >
            {mode === "spymaster" ? "Spymaster View" : "Public View"}
        </span>
    </div>
</div>

<!-- Error message -->
{#if error}
    <div
        class="mb-4 cursor-pointer rounded-lg border border-red-200 bg-red-50 p-4"
        on:click={clearError}
        on:keydown={(e) => e.key === "Enter" && clearError()}
        role="button"
        tabindex="0"
    >
        <div class="flex items-center">
            <div class="flex-shrink-0">
                <svg class="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
                    <path
                        fill-rule="evenodd"
                        d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                        clip-rule="evenodd"
                    />
                </svg>
            </div>
            <div class="ml-3">
                <p class="text-sm text-red-800">{error}</p>
                <p class="mt-1 text-xs text-red-600">Click to dismiss</p>
            </div>
        </div>
    </div>
{/if}

<!-- Loading state -->
{#if isLoading}
    <div class="flex items-center justify-center py-12">
        <div class="h-12 w-12 animate-spin rounded-full border-b-2 border-blue-500"></div>
        <span class="ml-3 text-lg text-gray-600">Loading board...</span>
    </div>
{:else if boardData.length === 0}
    <!-- Empty state -->
    <div class="flex items-center justify-center py-12">
        <div class="text-center">
            <p class="text-lg text-gray-500">No board data available</p>
            <button
                class="mt-2 rounded bg-blue-500 px-4 py-2 text-white hover:bg-blue-600"
                on:click={() => boardStore.initialize(mode)}
            >
                Retry
            </button>
        </div>
    </div>
{:else}
    <!-- Board grid -->
    <div class="mx-auto grid max-w-4xl grid-cols-5 gap-3">
        {#each boardData as row, rowIndex}
            {#each row as card, colIndex}
                <div
                    class={getCardClass(card)}
                    class:cursor-not-allowed={!isCardClickable(card)}
                    class:cursor-pointer={isCardClickable(card)}
                    on:click={() => handleCardClick(rowIndex, colIndex)}
                    on:keydown={(e) => e.key === "Enter" && handleCardClick(rowIndex, colIndex)}
                    role="button"
                    tabindex={isCardClickable(card) ? 0 : -1}
                    aria-label="Card {card.word}, {mode === 'spymaster'
                        ? `${card.team} team, ${(card as SpymasterCard).revealed ? 'revealed' : 'not revealed'}`
                        : `${(card as PublicCard).team === 'unknown' ? 'not revealed' : `revealed as ${(card as PublicCard).team}`}`}"
                >
                    <!-- Card word -->
                    <span class="text-sm font-semibold sm:text-base">
                        {card.word}
                    </span>

                    <!-- Revealed indicator for spymaster mode -->
                    {#if mode === "spymaster" && (card as SpymasterCard).revealed}
                        <div class="absolute top-1 right-1">
                            <svg
                                class="h-4 w-4 text-white opacity-75"
                                fill="currentColor"
                                viewBox="0 0 20 20"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </div>
                    {/if}

                    <!-- Team indicator dot for spymaster mode on unrevealed cards -->
                    {#if mode === "spymaster" && !(card as SpymasterCard).revealed}
                        <div
                            class="absolute top-2 left-2 h-2 w-2 rounded-full {(
                                card as SpymasterCard
                            ).team === 'red'
                                ? 'bg-red-500'
                                : (card as SpymasterCard).team === 'blue'
                                  ? 'bg-blue-500'
                                  : (card as SpymasterCard).team === 'neutral'
                                    ? 'bg-gray-500'
                                    : 'bg-black'}"
                        ></div>
                    {/if}
                </div>
            {/each}
        {/each}
    </div>
{/if}
