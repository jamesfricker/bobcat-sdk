import { create } from 'zustand';
import { GameDetail, PendingIntent } from '../types';

interface GameStore {
  currentGame: GameDetail | null;
  pendingIntent: PendingIntent | null;
  isConnected: boolean;
  userAddress: string | null;
  setCurrentGame: (game: GameDetail | null) => void;
  setPendingIntent: (intent: PendingIntent | null) => void;
  setConnection: (connected: boolean, address: string | null) => void;
  updateTimer: (deadline: string) => void;
}

export const useGameStore = create<GameStore>((set) => ({
  currentGame: null,
  pendingIntent: null,
  isConnected: false,
  userAddress: null,
  setCurrentGame: (game) => set({ currentGame: game }),
  setPendingIntent: (intent) => set({ pendingIntent: intent }),
  setConnection: (connected, address) => set({ isConnected: connected, userAddress: address }),
  updateTimer: (deadline) => set((state) => 
    state.currentGame 
      ? { currentGame: { ...state.currentGame, deadline } }
      : {}
  )
}));
