import { writable } from 'svelte/store';
// 视图状态 store（缩放和平移）
const viewStoreInternal = writable({
    scale: 1,
    offsetX: 0,
    offsetY: 0,
});
export const viewStore = {
    subscribe: viewStoreInternal.subscribe,
    setScale: (scale) => {
        viewStoreInternal.update((state) => ({
            ...state,
            scale,
        }));
    },
    setOffset: (offsetX, offsetY) => {
        viewStoreInternal.update((state) => ({
            ...state,
            offsetX,
            offsetY,
        }));
    },
    resetZoom: () => {
        viewStoreInternal.set({
            scale: 1,
            offsetX: 0,
            offsetY: 0,
        });
    },
};
// 世界数据 store
const worldStoreInternal = writable({
    world: null,
    loading: false,
    error: null,
});
export const worldStore = {
    subscribe: worldStoreInternal.subscribe,
    setWorld: (world) => {
        worldStoreInternal.update((state) => ({
            ...state,
            world,
            loading: false,
            error: null,
        }));
    },
    setLoading: (loading) => {
        worldStoreInternal.update((state) => ({
            ...state,
            loading,
        }));
    },
    setError: (error) => {
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
const highlightStoreInternal = writable({
    selectedTileId: null,
    selectedTileName: null,
    highlightAll: false,
    foundPositions: [],
});
export const highlightStore = {
    subscribe: highlightStoreInternal.subscribe,
    setSelectedTile: (tileId, tileName) => {
        highlightStoreInternal.update((state) => ({
            ...state,
            selectedTileId: tileId,
            selectedTileName: tileName,
            highlightAll: false,
            foundPositions: [],
        }));
    },
    setHighlightAll: (highlightAll) => {
        highlightStoreInternal.update((state) => ({
            ...state,
            highlightAll,
        }));
    },
    setFoundPositions: (positions) => {
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
const npcStoreInternal = writable({
    selectedNpcId: null,
    selectedNpcName: null,
    npcPosition: null,
});
export const npcStore = {
    subscribe: npcStoreInternal.subscribe,
    setSelectedNpc: (npcId, npcName, position) => {
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
//# sourceMappingURL=stores.js.map