<script lang="ts">
    export let board: { word: string; team: string; revealed?: boolean }[][];
    export let isSpymaster: boolean;
    export let onReveal: (row: number, col: number) => void;

    const color = (team: string) => {
        if (team == "red") {
            return "bg-red-500";
        }
        if (team == "blue") {
            return "bg-blue-500";
        }
        if (team == "neutral") {
            return "bg-green-500";
        }
        if (team == "assassin") {
            return "bg-gray-500";
        }
        return "bg-yellow-500";
    };
</script>

<div class="grid grid-cols-5 gap-2">
    {#each board as row, rowIndex}
        {#each row as card, colIndex}
            <button
                class="rounded p-4 text-center shadow {color(card.team)}"
                on:click={() => onReveal(rowIndex, colIndex)}
                disabled={card.revealed || isSpymaster}
            >
                {card.word}
            </button>
        {/each}
    {/each}
</div>
