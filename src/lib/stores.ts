import { writable } from 'svelte/store';
import type { World } from './types';

// 世界数据 store
const worldStoreInternal = writable<{
  world: World | null;
  loading: boolean;
  error: string | null;
}>({
  world: null,
  loading: false,
  error: null,
});

export const worldStore = {
  subscribe: worldStoreInternal.subscribe,
  setWorld: (world: World) => {
    worldStoreInternal.update((state) => ({
      ...state,
      world,
      loading: false,
      error: null,
    }));
  },
  setLoading: (loading: boolean) => {
    worldStoreInternal.update((state) => ({
      ...state,
      loading,
    }));
  },
  setError: (error: string) => {
    worldStoreInternal.update((state) => ({
      ...state,
      error,
      loading: false,
    }));
  },
  clear: () => {
    worldStoreInternal.set({
      world: null,
      loading: false,
      error: null,
    });
    highlightStore.clear();
    npcStore.clear();
  },
};

// 高亮方块 store
const highlightStoreInternal = writable<{
  selectedTileId: number | null;
  selectedTileName: string | null;
  highlightAll: boolean;
  foundPositions: Array<{ x: number; y: number }>;
}>({
  selectedTileId: null,
  selectedTileName: null,
  highlightAll: false,
  foundPositions: [],
});

export const highlightStore = {
  subscribe: highlightStoreInternal.subscribe,
  setSelectedTile: (tileId: number | null, tileName: string | null) => {
    highlightStoreInternal.update((state) => ({
      ...state,
      selectedTileId: tileId,
      selectedTileName: tileName,
      highlightAll: false,
      foundPositions: [],
    }));
  },
  setHighlightAll: (highlightAll: boolean) => {
    highlightStoreInternal.update((state) => ({
      ...state,
      highlightAll,
    }));
  },
  setFoundPositions: (positions: Array<{ x: number; y: number }>) => {
    highlightStoreInternal.update((state) => ({
      ...state,
      foundPositions: positions,
    }));
  },
  clear: () => {
    highlightStoreInternal.set({
      selectedTileId: null,
      selectedTileName: null,
      highlightAll: false,
      foundPositions: [],
    });
  },
};

// NPC 选择 store
const npcStoreInternal = writable<{
  selectedNpcId: number | null;
  selectedNpcName: string | null;
  npcPosition: { x: number; y: number } | null;
}>({
  selectedNpcId: null,
  selectedNpcName: null,
  npcPosition: null,
});

export const npcStore = {
  subscribe: npcStoreInternal.subscribe,
  setSelectedNpc: (npcId: number | null, npcName: string | null, position: { x: number; y: number } | null) => {
    npcStoreInternal.update((state) => ({
      ...state,
      selectedNpcId: npcId,
      selectedNpcName: npcName,
      npcPosition: position,
    }));
  },
  clear: () => {
    npcStoreInternal.set({
      selectedNpcId: null,
      selectedNpcName: null,
      npcPosition: null,
    });
  },
};