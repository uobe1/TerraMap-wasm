import { writable } from 'svelte/store';
import type { World } from './types';

// 世界数据 store
export const worldStore = writable<{
  world: World | null;
  loading: boolean;
  error: string | null;
}>({
  world: null,
  loading: false,
  error: null,
});

// 世界 store 的辅助方法
worldStore.setWorld = (world: World) => {
  worldStore.update((state) => ({
    ...state,
    world,
    loading: false,
    error: null,
  }));
};

worldStore.setLoading = (loading: boolean) => {
  worldStore.update((state) => ({
    ...state,
    loading,
  }));
};

worldStore.setError = (error: string) => {
  worldStore.update((state) => ({
    ...state,
    error,
    loading: false,
  }));
};

worldStore.clear = () => {
  worldStore.set({
    world: null,
    loading: false,
    error: null,
  });
};