<script lang="ts">
    import { onMount } from "svelte";
    import GameBoard from "$lib/components/GameBoard.svelte";
    import { boardStore } from "$lib/stores/boardStore";
    import type { Card } from "$lib/types";

    let board: Card[][] = [];

    onMount(() => {
        boardStore.connect(false); // false = not a spymaster
        const unsubscribe = boardStore.subscribe((value: Card[][]) => {
            board = value;
        });
        return unsubscribe;
    });

    const reveal = (row: number, col: number) => {
        boardStore.reveal(row, col);
    };
</script>

<GameBoard {board} isSpymaster={false} onReveal={reveal} />
