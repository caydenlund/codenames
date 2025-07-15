// src/lib/stores/boardStore.ts

import { writable, derived, get } from 'svelte/store';
import type { Writable, Readable } from 'svelte/store';

// Types matching your backend API
export type TeamType = 'red' | 'blue' | 'neutral' | 'assassin';
export type PublicTeamType = TeamType | 'unknown';

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

export type BoardMode = 'public' | 'spymaster';

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

// WebSocket message types
export interface WSMessage {
  type: 'board_update' | 'card_revealed' | 'game_reset';
  data: any;
}

class BoardStore {
  private _store: Writable<BoardState>;
  private _ws: WebSocket | null = null;
  private _reconnectTimer: number | null = null;
  private _maxReconnectAttempts = 5;
  private _reconnectDelay = 1000; // Start with 1 second
  private _currentMode: BoardMode = 'public';

  constructor() {
    this._store = writable<BoardState>({
      board: [],
      mode: 'public',
      loading: false,
      error: null,
      connected: false,
      reconnectAttempts: 0
    });
  }

  // Public store interface
  get store(): Readable<BoardState> {
    return this._store;
  }

  // Derived store for just the board data (convenience)
  get board(): Readable<PublicBoard | SpymasterBoard> {
    return derived(this._store, ($store) => $store.board);
  }

  // Derived store for connection status
  get connected(): Readable<boolean> {
    return derived(this._store, ($store) => $store.connected);
  }

  // Initialize the store with a specific mode
  async initialize(mode: BoardMode): Promise<void> {
    this._currentMode = mode;
    this._store.update(state => ({ 
      ...state, 
      mode, 
      loading: true, 
      error: null 
    }));

    try {
      // Fetch initial board state
      const endpoint = mode === 'public' ? '/api/board/public' : '/api/board/spymaster';
      const response = await fetch(endpoint);
      
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
      }
      
      const boardData = await response.json();
      
      this._store.update(state => ({
        ...state,
        board: boardData,
        loading: false,
        error: null
      }));

      // Initialize WebSocket connection
      this._initWebSocket();
      
    } catch (error) {
      this._store.update(state => ({
        ...state,
        loading: false,
        error: error instanceof Error ? error.message : 'Failed to load board'
      }));
    }
  }

  // Reveal a card (send to backend)
  async revealCard(row: number, col: number): Promise<void> {
    try {
      const response = await fetch('/api/reveal', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ row, col })
      });

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
      }

      // Don't update local state here - let the WebSocket handle it
      // This ensures consistency across all connected clients
      
    } catch (error) {
      this._store.update(state => ({
        ...state,
        error: error instanceof Error ? error.message : 'Failed to reveal card'
      }));
    }
  }

  // Clear any error state
  clearError(): void {
    this._store.update(state => ({ ...state, error: null }));
  }

  // WebSocket initialization
  private _initWebSocket(): void {
    if (this._ws) {
      this._ws.close();
    }

    try {
      // Use wss:// for https, ws:// for http
      const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
      const wsUrl = `${protocol}//${window.location.host}/ws`;
      
      this._ws = new WebSocket(wsUrl);
      
      this._ws.onopen = () => {
        console.log('WebSocket connected');
        this._store.update(state => ({
          ...state,
          connected: true,
          reconnectAttempts: 0
        }));
        
        // Clear any existing reconnect timer
        if (this._reconnectTimer) {
          clearTimeout(this._reconnectTimer);
          this._reconnectTimer = null;
        }
      };

      this._ws.onmessage = (event) => {
        try {
          const message: WSMessage = JSON.parse(event.data);
          this._handleWebSocketMessage(message);
        } catch (error) {
          console.error('Failed to parse WebSocket message:', error);
        }
      };

      this._ws.onclose = () => {
        console.log('WebSocket disconnected');
        this._store.update(state => ({
          ...state,
          connected: false
        }));
        
        // Attempt to reconnect
        this._attemptReconnect();
      };

      this._ws.onerror = (error) => {
        console.error('WebSocket error:', error);
        this._store.update(state => ({
          ...state,
          error: 'WebSocket connection error'
        }));
      };
      
    } catch (error) {
      console.error('Failed to initialize WebSocket:', error);
      this._store.update(state => ({
        ...state,
        error: 'Failed to initialize WebSocket connection'
      }));
    }
  }

  // Handle incoming WebSocket messages
  private _handleWebSocketMessage(message: WSMessage): void {
    const currentState = get(this._store);
    
    switch (message.type) {
      case 'board_update':
        // Full board update
        this._store.update(state => ({
          ...state,
          board: message.data,
          error: null
        }));
        break;
        
      case 'card_revealed':
        // Single card update - more efficient than full board update
        const { row, col, newCardState } = message.data;
        this._store.update(state => {
          const newBoard = [...state.board];
          newBoard[row] = [...newBoard[row]];
          newBoard[row][col] = newCardState;
          return { ...state, board: newBoard };
        });
        break;
        
      case 'game_reset':
        // Game has been reset, refetch board state
        this.initialize(currentState.mode);
        break;
        
      default:
        console.warn('Unknown WebSocket message type:', message.type);
    }
  }

  // Reconnection logic with exponential backoff
  private _attemptReconnect(): void {
    const currentState = get(this._store);
    
    if (currentState.reconnectAttempts >= this._maxReconnectAttempts) {
      this._store.update(state => ({
        ...state,
        error: 'Unable to reconnect to server. Please refresh the page.'
      }));
      return;
    }

    const delay = this._reconnectDelay * Math.pow(2, currentState.reconnectAttempts);
    
    this._store.update(state => ({
      ...state,
      reconnectAttempts: state.reconnectAttempts + 1
    }));

    this._reconnectTimer = setTimeout(() => {
      console.log(`Attempting to reconnect... (attempt ${currentState.reconnectAttempts + 1})`);
      this._initWebSocket();
    }, delay);
  }

  // Cleanup method
  destroy(): void {
    if (this._ws) {
      this._ws.close();
    }
    if (this._reconnectTimer) {
      clearTimeout(this._reconnectTimer);
    }
  }
}

// Export a singleton instance
export const boardStore = new BoardStore();

// Export the store interface for components
export const board = boardStore.store;
export const boardData = boardStore.board;
export const connected = boardStore.connected;
