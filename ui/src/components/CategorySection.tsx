import { useEffect } from 'react';
import { motion } from 'framer-motion';
import { ChevronLeft, ChevronRight, Upload, Sparkles } from 'lucide-react';
import toast from 'react-hot-toast';

import { useAppStore } from '../store/appStore';
import { api } from '../services/api';
import { ItemCategory, CategoryInfo, ItemInfo } from '../types/api';

import LoadingSpinner from './ui/LoadingSpinner';

interface CategorySectionProps {
  category: ItemCategory;
  categoryData?: CategoryInfo;
  position: string;
  label: string;
}

const positionClasses = {
  'top-center': 'absolute top-0 left-1/2 -translate-x-1/2 -translate-y-16',
  'top-right': 'absolute top-0 right-0 translate-x-16 -translate-y-8',
  'left': 'absolute left-0 top-1/2 -translate-y-1/2 -translate-x-16',
  'right': 'absolute right-0 top-1/2 -translate-y-1/2 translate-x-16',
  'bottom-right': 'absolute bottom-0 right-0 translate-x-12 translate-y-16',
  'bottom-center': 'absolute bottom-0 left-1/2 -translate-x-1/2 translate-y-16',
  'relative': 'relative',
  'grid': 'relative',
  'mobile': 'relative',
};

const CategorySection: React.FC<CategorySectionProps> = ({
  category,
  categoryData,
  position,
  label,
}) => {
  const {
    categoryItems,
    galleryState,
    loading,
    selectedItems,
    setCategoryItems,
    setItemsLoading,
    setError,
    setGalleryIndex,
    nextImage,
    prevImage,
    setSelectedItem,
  } = useAppStore();

  const items = categoryItems[category] || [];
  const currentIndex = galleryState.currentIndex[category] || 0;
  const isLoading = loading.loadingItems[category] || false;
  const selectedItem = selectedItems[category];
  const currentItem = items[currentIndex];

  // Load items for this category
  useEffect(() => {
    const loadItems = async () => {
      if (items.length > 0) return; // Already loaded

      setItemsLoading(category, true);
      try {
        const response = await api.getCategoryItems(category);
        setCategoryItems(category, response.items);

        // Auto-select first item if available
        if (response.items.length > 0) {
          setSelectedItem(category, response.items[0]);
        }
      } catch (error) {
        setError({
          message: `Failed to load ${label} items: ${error instanceof Error ? error.message : 'Unknown error'}`,
          type: 'api',
        });
      } finally {
        setItemsLoading(category, false);
      }
    };

    loadItems();
  }, [category, items.length, label, setCategoryItems, setItemsLoading, setError, setSelectedItem]);

  // Generate new item
  const handleGenerateItem = async () => {
    try {
      const response = await api.generateItem(category, {
        prompt: `Generate a stylish ${label}`,
      });

      const updatedItems = [response.item, ...items];
      setCategoryItems(category, updatedItems);
      setGalleryIndex(category, 0);
      setSelectedItem(category, response.item);

      toast.success(`Generated new ${label}!`);
    } catch (error) {
      toast.error(`Failed to generate ${label}: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  };

  const handleItemSelect = (item: ItemInfo) => {
    setSelectedItem(category, item);
    toast.success(`Selected ${label}: ${item.filename || 'Generated item'}`);
  };

  const positionClass = positionClasses[position as keyof typeof positionClasses] || '';

  return (
    <motion.div
      initial={{ opacity: 0, scale: 0.8 }}
      animate={{ opacity: 1, scale: 1 }}
      transition={{ delay: 0.2 }}
      className={`${positionClass} w-72 h-56 bg-white/95 backdrop-blur-sm rounded-xl shadow-xl border border-white/30 overflow-hidden z-10 hover:shadow-2xl transition-all duration-300`}
    >
      <div className="p-4 h-full flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between mb-3">
          <h3 className="font-semibold text-gray-800 text-base">{label}</h3>
          <span className="text-xs text-gray-500 bg-gray-100 px-2 py-1 rounded-full">
            {categoryData?.count || items.length} items
          </span>
        </div>

        {/* Content */}
        {isLoading ? (
          <div className="flex-1 flex items-center justify-center">
            <div className="text-center">
              <LoadingSpinner className="text-purple-600 mx-auto mb-2" />
              <p className="text-xs text-gray-500">Loading...</p>
            </div>
          </div>
        ) : items.length > 0 ? (
          <div className="flex-1 flex flex-col">
            {/* Image Display */}
            <div className="relative flex-1 bg-gray-100 rounded-lg overflow-hidden mb-2">
              {currentItem && (
                <img
                  src={currentItem.url}
                  alt={currentItem.filename || `${label} item`}
                  className={`w-full h-full object-cover transition-all duration-300 ${
                    selectedItem?.id === currentItem.id ? 'ring-2 ring-purple-500' : ''
                  }`}
                />
              )}

              {/* Selection Indicator */}
              {selectedItem?.id === currentItem?.id && (
                <motion.div
                  initial={{ scale: 0 }}
                  animate={{ scale: 1 }}
                  className="absolute top-2 right-2 bg-purple-500 text-white rounded-full p-1"
                >
                  <svg className="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                    <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                  </svg>
                </motion.div>
              )}
            </div>

            {/* Navigation & Actions */}
            <div className="flex items-center justify-between mt-2">
              {/* Navigation */}
              <div className="flex items-center space-x-2 bg-gray-100 rounded-full p-1">
                <button
                  onClick={() => prevImage(category)}
                  disabled={items.length <= 1}
                  className="p-2 text-gray-600 hover:text-gray-800 hover:bg-white rounded-full disabled:opacity-50 disabled:cursor-not-allowed transition-all"
                >
                  <ChevronLeft size={16} />
                </button>

                <span className="text-xs text-gray-600 px-2 font-medium min-w-[3rem] text-center">
                  {items.length > 0 ? `${currentIndex + 1}/${items.length}` : '0/0'}
                </span>

                <button
                  onClick={() => nextImage(category)}
                  disabled={items.length <= 1}
                  className="p-2 text-gray-600 hover:text-gray-800 hover:bg-white rounded-full disabled:opacity-50 disabled:cursor-not-allowed transition-all"
                >
                  <ChevronRight size={16} />
                </button>
              </div>

              {/* Actions */}
              <div className="flex items-center space-x-2">
                <button
                  onClick={() => currentItem && handleItemSelect(currentItem)}
                  disabled={!currentItem}
                  className="p-2 bg-purple-100 text-purple-600 hover:bg-purple-200 hover:text-purple-700 rounded-full disabled:opacity-50 disabled:cursor-not-allowed transition-all"
                  title="Select this item"
                >
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                  </svg>
                </button>

                <button
                  onClick={handleGenerateItem}
                  className="p-2 bg-blue-100 text-blue-600 hover:bg-blue-200 hover:text-blue-700 rounded-full transition-all"
                  title="Generate new item"
                >
                  <Sparkles size={16} />
                </button>
              </div>
            </div>
          </div>
        ) : (
          <div className="flex-1 flex flex-col items-center justify-center text-center">
            <div className="bg-gray-50 rounded-xl p-6 mb-4 w-full border-2 border-dashed border-gray-200">
              <Upload className="w-10 h-10 text-gray-400 mx-auto mb-3" />
              <p className="text-sm text-gray-600 font-medium">No items yet</p>
              <p className="text-xs text-gray-500 mt-1">Generate your first item</p>
            </div>

            <button
              onClick={handleGenerateItem}
              className="w-full bg-gradient-to-r from-purple-500 to-pink-500 text-white text-sm py-3 px-4 rounded-xl hover:from-purple-600 hover:to-pink-600 transition-all duration-200 flex items-center justify-center space-x-2 font-medium shadow-lg"
            >
              <Sparkles size={16} />
              <span>Generate {label}</span>
            </button>
          </div>
        )}
      </div>
    </motion.div>
  );
};

export default CategorySection;
