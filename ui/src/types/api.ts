/**
 * TypeScript types for Mirage Fit API
 */

// Item Categories based on backend ItemCategory enum
export enum ItemCategory {
  Hat = "hat",
  Glasses = "glasses",
  Shoes = "shoes",
  Top = "top",
  BottomSkirt = "bottom",
  Socks = "socks",
  Gloves = "gloves",
  Scarf = "scarf",
  Bag = "bag",
  Accessory = "accessory",
  Other = "other"
}

// API Response Types
export interface HealthResponse {
  status: string;
  version: string;
  gemini_api_available: boolean;
}

export interface CategoryInfo {
  name: string;
  count: number;
  id: ItemCategory;
}

export interface CategoriesResponse {
  categories: CategoryInfo[];
}

export interface ItemInfo {
  id: string;
  filename: string | null;
  hash: string;
  dimensions: [number, number];
  created_at: string;
  prompt: string | null;
  url: string;
}

export interface ItemsResponse {
  category: ItemCategory;
  items: ItemInfo[];
}

export interface GenerateItemRequest {
  prompt?: string;
  style?: string;
  color?: string;
}

export interface GenerateItemResponse {
  item: ItemInfo;
  message: string;
}

export interface UploadResponse {
  id: string;
  hash: string;
  filename: string | null;
  dimensions: [number, number];
  size: number;
  url: string;
  message: string;
}

export interface RemixRequest {
  base_image: string; // hash
  items: [ItemCategory, string][]; // category, hash pairs
  style?: string;
  quality?: number;
}

export interface RemixResponse {
  id: string;
  hash: string;
  dimensions: [number, number];
  url: string;
  source_images: string[];
  message: string;
}

export interface OutputInfo {
  id: string;
  hash: string;
  dimensions: [number, number];
  created_at: string;
  source_images: string[];
  url: string;
}

export interface OutputsResponse {
  outputs: OutputInfo[];
}

export interface ErrorResponse {
  error: {
    message: string;
    code: number;
  };
}

// UI State Types
export interface SelectedItems {
  [key: string]: ItemInfo | null; // category -> selected item
}

export interface LoadingState {
  uploading: boolean;
  generating: boolean;
  remixing: boolean;
  loadingCategories: boolean;
  loadingItems: { [category: string]: boolean };
}

export interface AppError {
  message: string;
  type: 'api' | 'network' | 'validation';
}

// Gallery Navigation
export interface GalleryState {
  currentIndex: { [category: string]: number };
}
