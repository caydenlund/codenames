type CardType = "red" | "blue" | "neutral" | "assassin" | "hidden";

interface Card {
    word: string;
    type: CardType;
}

type Grid = [
    [Card, Card, Card, Card, Card],
    [Card, Card, Card, Card, Card],
    [Card, Card, Card, Card, Card],
    [Card, Card, Card, Card, Card],
    [Card, Card, Card, Card, Card]
];

// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
    namespace App {
        // interface Error {}
        // interface Locals {}
        // interface PageData {}
        interface PageState {
            board: Grid;
        }
        // interface Platform {}
    }

    interface ImportMetaEnv {
        VITE_BACKEND: string;
    }
}

export {};
