/**
 * API Service Layer for Mirage Fit
 * Handles all communication with the Rust backend
 */

import {
  ItemCategory,
  HealthResponse,
  CategoriesResponse,
  ItemsResponse,
  GenerateItemRequest,
  GenerateItemResponse,
  UploadResponse,
  RemixRequest,
  RemixResponse,
  OutputsResponse,
  ErrorResponse,
} from '../types/api';

const API_BASE = 'http://127.0.0.1:3000/api';

class ApiError extends Error {
  constructor(public message: string, public status?: number) {
    super(message);
    this.name = 'ApiError';
  }
}

/**
 * Generic API request handler with error handling
 */
async function apiRequest<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<T> {
  try {
    const response = await fetch(`${API_BASE}${endpoint}`, {
      headers: {
        'Content-Type': 'application/json',
        ...options.headers,
      },
      ...options,
    });

    if (!response.ok) {
      let errorMessage = `HTTP ${response.status}`;

      try {
        const errorBody: ErrorResponse = await response.json();
        errorMessage = errorBody.error.message || errorMessage;
      } catch {
        errorMessage = response.statusText || errorMessage;
      }

      throw new ApiError(errorMessage, response.status);
    }

    const data: T = await response.json();
    return data;
  } catch (error) {
    if (error instanceof ApiError) {
      throw error;
    }

    // Network or other errors
    throw new ApiError(
      error instanceof Error ? error.message : 'Network request failed'
    );
  }
}

/**
 * Upload file with multipart form data
 */
async function uploadFile(
  endpoint: string,
  file: File,
  additionalFields?: Record<string, string>
): Promise<any> {
  try {
    const formData = new FormData();
    formData.append('image', file);

    if (additionalFields) {
      Object.entries(additionalFields).forEach(([key, value]) => {
        formData.append(key, value);
      });
    }

    const response = await fetch(`${API_BASE}${endpoint}`, {
      method: 'POST',
      body: formData,
    });

    if (!response.ok) {
      let errorMessage = `HTTP ${response.status}`;

      try {
        const errorBody: ErrorResponse = await response.json();
        errorMessage = errorBody.error.message || errorMessage;
      } catch {
        errorMessage = response.statusText || errorMessage;
      }

      throw new ApiError(errorMessage, response.status);
    }

    return await response.json();
  } catch (error) {
    if (error instanceof ApiError) {
      throw error;
    }

    throw new ApiError(
      error instanceof Error ? error.message : 'Upload failed'
    );
  }
}

export const api = {
  /**
   * Health check endpoint
   */
  async healthCheck(): Promise<HealthResponse> {
    return apiRequest<HealthResponse>('/health');
  },

  /**
   * Get all available item categories
   */
  async getCategories(): Promise<CategoriesResponse> {
    return apiRequest<CategoriesResponse>('/categories');
  },

  /**
   * Get items for a specific category
   */
  async getCategoryItems(category: ItemCategory): Promise<ItemsResponse> {
    return apiRequest<ItemsResponse>(`/items/${encodeURIComponent(category)}`);
  },

  /**
   * Generate a new item for a category
   */
  async generateItem(
    category: ItemCategory,
    request: GenerateItemRequest
  ): Promise<GenerateItemResponse> {
    return apiRequest<GenerateItemResponse>(`/items/${encodeURIComponent(category)}`, {
      method: 'POST',
      body: JSON.stringify(request),
    });
  },

  /**
   * Upload a user photo
   */
  async uploadPhoto(file: File): Promise<UploadResponse> {
    return uploadFile('/upload', file);
  },

  /**
   * Generate a remix image
   */
  async generateRemix(request: RemixRequest): Promise<RemixResponse> {
    return apiRequest<RemixResponse>('/remix', {
      method: 'POST',
      body: JSON.stringify(request),
    });
  },

  /**
   * Get list of generated output images
   */
  async getOutputs(): Promise<OutputsResponse> {
    return apiRequest<OutputsResponse>('/outputs');
  },

  /**
   * Get image URL for display
   */
  getImageUrl(type: 'input' | 'items' | 'output', path: string): string {
    return `${API_BASE}/images/${type}/${path}`;
  },
};

export { ApiError };
