import { useEffect } from 'react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Toaster } from 'react-hot-toast';
import { motion } from 'framer-motion';

import { useAppStore } from './store/appStore';
import { api } from './services/api';

import ImageSelectionLayout from './components/ImageSelectionLayout';
import ErrorBoundary from './components/ErrorBoundary';
import LoadingSpinner from './components/ui/LoadingSpinner';

// import { ItemCategory } from './types/api'; // Commented out - not used in this file

// Create a client
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      retry: 2,
      staleTime: 5 * 60 * 1000, // 5 minutes
    },
  },
});


function MirageFitApp() {
  const {
    categories,
    loading,
    error,
    setCategories,
    setLoading,
    setError,
    clearError,
  } = useAppStore();

  // Load categories on app start
  useEffect(() => {
    const loadCategories = async () => {
      setLoading('loadingCategories', true);
      try {
        const response = await api.getCategories();
        setCategories(response.categories);
      } catch (error) {
        setError({
          message: error instanceof Error ? error.message : 'Failed to load categories',
          type: 'api',
        });
      } finally {
        setLoading('loadingCategories', false);
      }
    };

    loadCategories();
  }, [setCategories, setLoading, setError]);

  if (loading.loadingCategories) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-purple-50 via-blue-50 to-pink-50">
        <div className="text-center">
          <LoadingSpinner size="lg" />
          <p className="mt-4 text-gray-600 text-lg">Loading Mirage Fit...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-50 via-blue-50 to-pink-50 overflow-hidden">
      {/* Header */}
      <header className="relative z-10 bg-white/80 backdrop-blur-md border-b border-white/20 shadow-sm">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <motion.h1
              initial={{ opacity: 0, y: -20 }}
              animate={{ opacity: 1, y: 0 }}
              className="text-2xl font-bold bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent"
            >
              Mirage Fit
            </motion.h1>
            <div className="text-sm text-gray-600">
              AI-Powered Fashion Remix
            </div>
          </div>
        </div>
      </header>

      {/* Error Display */}
      {error && (
        <motion.div
          initial={{ opacity: 0, y: -50 }}
          animate={{ opacity: 1, y: 0 }}
          className="relative z-20 bg-red-50 border-l-4 border-red-500 p-4 m-4 rounded-r-lg"
        >
          <div className="flex items-center justify-between">
            <div className="text-red-700">
              <strong>Error:</strong> {error.message}
            </div>
            <button
              onClick={clearError}
              className="text-red-500 hover:text-red-700 font-bold text-xl"
            >
              ×
            </button>
          </div>
        </motion.div>
      )}

      {/* Main Layout */}
      <main className="container mx-auto px-4 py-6">
        <ImageSelectionLayout categories={categories} />
      </main>

      {/* Toast notifications */}
      <Toaster
        position="top-right"
        toastOptions={{
          duration: 4000,
          style: {
            background: 'rgba(255, 255, 255, 0.9)',
            backdropFilter: 'blur(10px)',
            border: '1px solid rgba(255, 255, 255, 0.2)',
          },
        }}
      />
    </div>
  );
}

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <ErrorBoundary>
        <MirageFitApp />
      </ErrorBoundary>
    </QueryClientProvider>
  );
}

export default App;
