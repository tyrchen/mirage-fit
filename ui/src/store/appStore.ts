/**
 * Application state management using Zustand
 */

import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import {
  ItemCategory,
  ItemInfo,
  CategoryInfo,
  SelectedItems,
  LoadingState,
  AppError,
  GalleryState,
  UploadResponse,
  OutputInfo,
} from '../types/api';

interface AppState {
  // Data
  categories: CategoryInfo[];
  categoryItems: Record<string, ItemInfo[]>;
  selectedItems: SelectedItems;
  uploadedPhoto: UploadResponse | null;
  outputs: OutputInfo[];

  // UI State
  loading: LoadingState;
  error: AppError | null;
  galleryState: GalleryState;

  // Actions
  setCategories: (categories: CategoryInfo[]) => void;
  setCategoryItems: (category: ItemCategory, items: ItemInfo[]) => void;
  setSelectedItem: (category: ItemCategory, item: ItemInfo | null) => void;
  setUploadedPhoto: (photo: UploadResponse | null) => void;
  setOutputs: (outputs: OutputInfo[]) => void;

  // Loading actions
  setLoading: (key: keyof LoadingState, value: boolean) => void;
  setItemsLoading: (category: string, loading: boolean) => void;

  // Error handling
  setError: (error: AppError | null) => void;
  clearError: () => void;

  // Gallery navigation
  setGalleryIndex: (category: string, index: number) => void;
  nextImage: (category: string) => void;
  prevImage: (category: string) => void;

  // Utils
  reset: () => void;
  getSelectedItemsForRemix: () => [ItemCategory, string][];
}

const initialState = {
  categories: [],
  categoryItems: {},
  selectedItems: {},
  uploadedPhoto: null,
  outputs: [],
  loading: {
    uploading: false,
    generating: false,
    remixing: false,
    loadingCategories: false,
    loadingItems: {},
  },
  error: null,
  galleryState: {
    currentIndex: {},
  },
};

export const useAppStore = create<AppState>()(
  devtools(
    (set, get) => ({
      ...initialState,

      setCategories: (categories) =>
        set({ categories }, false, 'setCategories'),

      setCategoryItems: (category, items) =>
        set(
          (state) => ({
            categoryItems: {
              ...state.categoryItems,
              [category]: items,
            },
          }),
          false,
          'setCategoryItems'
        ),

      setSelectedItem: (category, item) =>
        set(
          (state) => ({
            selectedItems: {
              ...state.selectedItems,
              [category]: item,
            },
          }),
          false,
          'setSelectedItem'
        ),

      setUploadedPhoto: (photo) =>
        set({ uploadedPhoto: photo }, false, 'setUploadedPhoto'),

      setOutputs: (outputs) =>
        set({ outputs }, false, 'setOutputs'),

      setLoading: (key, value) =>
        set(
          (state) => ({
            loading: {
              ...state.loading,
              [key]: value,
            },
          }),
          false,
          'setLoading'
        ),

      setItemsLoading: (category, loading) =>
        set(
          (state) => ({
            loading: {
              ...state.loading,
              loadingItems: {
                ...state.loading.loadingItems,
                [category]: loading,
              },
            },
          }),
          false,
          'setItemsLoading'
        ),

      setError: (error) =>
        set({ error }, false, 'setError'),

      clearError: () =>
        set({ error: null }, false, 'clearError'),

      setGalleryIndex: (category, index) =>
        set(
          (state) => ({
            galleryState: {
              ...state.galleryState,
              currentIndex: {
                ...state.galleryState.currentIndex,
                [category]: index,
              },
            },
          }),
          false,
          'setGalleryIndex'
        ),

      nextImage: (category) => {
        const state = get();
        const items = state.categoryItems[category] || [];
        const currentIndex = state.galleryState.currentIndex[category] || 0;
        const nextIndex = (currentIndex + 1) % Math.max(items.length, 1);

        set(
          (state) => ({
            galleryState: {
              ...state.galleryState,
              currentIndex: {
                ...state.galleryState.currentIndex,
                [category]: nextIndex,
              },
            },
          }),
          false,
          'nextImage'
        );
      },

      prevImage: (category) => {
        const state = get();
        const items = state.categoryItems[category] || [];
        const currentIndex = state.galleryState.currentIndex[category] || 0;
        const prevIndex = currentIndex === 0 ? Math.max(items.length - 1, 0) : currentIndex - 1;

        set(
          (state) => ({
            galleryState: {
              ...state.galleryState,
              currentIndex: {
                ...state.galleryState.currentIndex,
                [category]: prevIndex,
              },
            },
          }),
          false,
          'prevImage'
        );
      },

      getSelectedItemsForRemix: () => {
        const state = get();
        const result: [ItemCategory, string][] = [];

        Object.entries(state.selectedItems).forEach(([category, item]) => {
          if (item) {
            result.push([category as ItemCategory, item.hash]);
          }
        });

        return result;
      },

      reset: () =>
        set(initialState, false, 'reset'),
    }),
    {
      name: 'mirage-fit-store',
    }
  )
);
