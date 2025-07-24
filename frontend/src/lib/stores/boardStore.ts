import { writable, derived, get } from "svelte/store";
import type { Writable, Readable } from "svelte/store";

export type TeamType = "red" | "blue" | "neutral" | "assassin";
export type PublicTeamType = TeamType | "unknown";

export interface PublicCard {
    word: string;
    team: PublicTeamType;
}

export interface SpymasterCard {
    word: string;
    team: TeamType;
    revealed: boolean;
}

export type PublicBoard = PublicCard[][];
export type SpymasterBoard = SpymasterCard[][];

export type BoardMode = "public" | "spymaster";

export interface BoardState {
    board: PublicBoard | SpymasterBoard;
    mode: BoardMode;
    loading: boolean;
    error: string | null;
    connected: boolean;
    reconnectAttempts: number;
}

export interface RevealRequest {
    row: number;
    col: number;
}

export interface WsMessage {
    type: "card_revealed" | "new_game";
    data:
        | {
              row: number;
              col: number;
              new_card_state: SpymasterCard;
          }
        | PublicBoard
        | SpymasterBoard;
}

class BoardStore {
    private _store: Writable<BoardState>;
    private _ws: WebSocket | null = null;
    private _reconnectTimer: number | null = null;
    private _maxReconnectAttempts = 5;
    private _reconnectDelay = 1000;
    private _mode: "public" | "spymaster" = "public";

    constructor() {
        this._store = writable<BoardState>({
            board: [],
            mode: "public",
            loading: false,
            error: null,
            connected: false,
            reconnectAttempts: 0
        });
    }

    get store(): Readable<BoardState> {
        return this._store;
    }

    get board(): Readable<PublicBoard | SpymasterBoard> {
        return derived(this._store, ($store) => $store.board);
    }

    get connected(): Readable<boolean> {
        return derived(this._store, ($store) => $store.connected);
    }

    async initialize(mode: BoardMode): Promise<void> {
        this._store.update((state) => ({
            ...state,
            mode,
            loading: true,
            error: null
        }));
        this._mode = mode;

        try {
            const endpoint = mode === "public" ? "/api/board/public" : "/api/board/spymaster";
            const response = await fetch(endpoint);

            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }

            const boardData = await response.json();

            this._store.update((state) => ({
                ...state,
                board: boardData,
                loading: false,
                error: null
            }));

            this._initWebSocket();
        } catch (error) {
            this._store.update((state) => ({
                ...state,
                loading: false,
                error: error instanceof Error ? error.message : "Failed to load board"
            }));
        }
    }

    async revealCard(row: number, col: number): Promise<void> {
        try {
            const response = await fetch("/api/reveal", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({ row, col })
            });

            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }
        } catch (error) {
            this._store.update((state) => ({
                ...state,
                error: error instanceof Error ? error.message : "Failed to reveal card"
            }));
        }
    }

    async newGame(): Promise<void> {
        try {
            this._store.update((state) => ({
                ...state,
                loading: true
            }));

            const response = await fetch("/api/new_game", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: "{}"
            });

            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }
        } catch (error) {
            this._store.update((state) => ({
                ...state,
                error: error instanceof Error ? error.message : "Failed to create a new game"
            }));
        }
    }

    clearError(): void {
        this._store.update((state) => ({ ...state, error: null }));
    }

    private _initWebSocket(): void {
        if (this._ws) {
            this._ws.close();
        }

        try {
            const wsUrl = `/ws/${this._mode}`;

            this._ws = new WebSocket(wsUrl);

            this._ws.onopen = () => {
                console.log("WebSocket connected");
                this._store.update((state) => ({
                    ...state,
                    connected: true,
                    reconnectAttempts: 0
                }));

                if (this._reconnectTimer) {
                    clearTimeout(this._reconnectTimer);
                    this._reconnectTimer = null;
                }
            };

            this._ws.onmessage = (event) => {
                try {
                    const message: WsMessage = JSON.parse(event.data);
                    this._handleWebSocketMessage(message);
                } catch (error) {
                    console.error("Failed to parse WebSocket message:", error);
                }
            };

            this._ws.onclose = () => {
                console.log("WebSocket disconnected");
                this._store.update((state) => ({
                    ...state,
                    connected: false
                }));

                this._attemptReconnect();
            };

            this._ws.onerror = (error) => {
                console.error("WebSocket error:", error);
                this._store.update((state) => ({
                    ...state,
                    error: "WebSocket connection error"
                }));
            };
        } catch (error) {
            console.error("Failed to initialize WebSocket:", error);
            this._store.update((state) => ({
                ...state,
                error: "Failed to initialize WebSocket connection"
            }));
        }
    }

    private _handleWebSocketMessage(message: WsMessage): void {
        switch (message.type) {
            case "card_revealed":
                if ("row" in message.data) {
                    const { row, col, new_card_state } = message.data;
                    this._store.update((state) => {
                        const newBoard = [...state.board];
                        newBoard[row] = [...newBoard[row]];
                        newBoard[row][col] = new_card_state;
                        return { ...state, board: newBoard };
                    });
                }
                break;

            case "new_game":
                this._store.update((state) => ({
                    ...state,
                    board: message.data as PublicBoard | SpymasterBoard,
                    loading: false,
                    error: null
                }));
                break;

            default:
                console.warn("Unknown WebSocket message type:", message.type);
        }
    }

    private _attemptReconnect(): void {
        const currentState = get(this._store);

        if (currentState.reconnectAttempts >= this._maxReconnectAttempts) {
            this._store.update((state) => ({
                ...state,
                error: "Unable to reconnect to server. Please refresh the page."
            }));
            return;
        }

        const delay = this._reconnectDelay * Math.pow(2, currentState.reconnectAttempts);

        this._store.update((state) => ({
            ...state,
            reconnectAttempts: state.reconnectAttempts + 1
        }));

        this._reconnectTimer = setTimeout(() => {
            console.log(
                `Attempting to reconnect... (attempt ${currentState.reconnectAttempts + 1})`
            );
            this._initWebSocket();
        }, delay);
    }

    destroy(): void {
        if (this._ws) {
            this._ws.close();
        }
        if (this._reconnectTimer) {
            clearTimeout(this._reconnectTimer);
        }
    }
}

export const boardStore = new BoardStore();

export const board = boardStore.store;
export const boardData = boardStore.board;
export const connected = boardStore.connected;
