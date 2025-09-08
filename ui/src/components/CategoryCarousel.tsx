import React, { useEffect, useState } from 'react';
import { motion } from 'framer-motion';
import { ChevronLeft, ChevronRight, Plus, Upload, Check } from 'lucide-react';
import toast from 'react-hot-toast';
import { useAppStore } from '../store/appStore';
import { api } from '../services/api';
import { ItemCategory, CategoryInfo, ItemInfo } from '../types/api';
import LoadingSpinner from './ui/LoadingSpinner';

interface CategoryCarouselProps {
  category: ItemCategory;
  label: string;
  categoryData?: CategoryInfo;
}

const CategoryCarousel: React.FC<CategoryCarouselProps> = ({
  category,
  label,
  categoryData,
}) => {
  const {
    categoryItems,
    loading,
    selectedItems,
    setCategoryItems,
    setItemsLoading,
    setError,
    setSelectedItem,
  } = useAppStore();

  const [currentIndex, setCurrentIndex] = useState(0);
  const items = categoryItems[category] || [];
  const isLoading = loading.loadingItems[category] || false;
  const selectedItem = selectedItems[category];

  // Load items for this category
  useEffect(() => {
    const loadItems = async () => {
      if (items.length > 0) return;

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
          message: `Failed to load ${label} items`,
          type: 'api',
        });
      } finally {
        setItemsLoading(category, false);
      }
    };

    loadItems();
  }, [category, items.length, label, setCategoryItems, setItemsLoading, setError, setSelectedItem]);

  const handlePrevious = () => {
    if (items.length <= 1) return;
    setCurrentIndex((prev) => (prev === 0 ? items.length - 1 : prev - 1));
  };

  const handleNext = () => {
    if (items.length <= 1) return;
    setCurrentIndex((prev) => (prev === items.length - 1 ? 0 : prev + 1));
  };

  const handleGenerateItem = async () => {
    try {
      setItemsLoading(category, true);
      const response = await api.generateItem(category, {
        prompt: `Generate a stylish ${label}`,
      });

      const updatedItems = [response.item, ...items];
      setCategoryItems(category, updatedItems);
      setCurrentIndex(0);
      setSelectedItem(category, response.item);

      toast.success(`Generated new ${label}!`);
    } catch (error) {
      toast.error(`Failed to generate ${label}`);
    } finally {
      setItemsLoading(category, false);
    }
  };

  const handleSelectItem = (item: ItemInfo) => {
    setSelectedItem(category, item);
    toast.success(`Selected ${label}`);
  };

  const currentItem = items[currentIndex];

  return (
    <motion.div
      initial={{ opacity: 0, scale: 0.9 }}
      animate={{ opacity: 1, scale: 1 }}
      transition={{ delay: 0.1 }}
      className="bg-white rounded-xl shadow-lg p-4 w-48"
    >
      {/* Header */}
      <div className="flex items-center justify-between mb-3">
        <h3 className="text-sm font-semibold text-gray-800">{label}</h3>
        <span className="text-xs text-gray-500 bg-gray-100 px-2 py-0.5 rounded-full">
          {items.length} items
        </span>
      </div>

      {/* Carousel Content */}
      {isLoading ? (
        <div className="h-32 flex items-center justify-center">
          <LoadingSpinner className="text-purple-600" />
        </div>
      ) : (
        <div className="relative">
          {/* Image Carousel */}
          <div className="flex items-center space-x-1 mb-3">
            {/* Previous Button */}
            <button
              onClick={handlePrevious}
              disabled={items.length <= 1}
              className="p-1 text-gray-500 hover:text-gray-700 disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
            >
              <ChevronLeft size={16} />
            </button>

            {/* Image Items */}
            <div className="flex-1 flex space-x-1 overflow-hidden">
              {items.length > 0 ? (
                <>
                  {/* Show current and next 2 items */}
                  {[0, 1, 2].map((offset) => {
                    const index = (currentIndex + offset) % items.length;
                    const item = items[index];
                    if (!item) return null;

                    return (
                      <div
                        key={`${item.id}-${offset}`}
                        onClick={() => handleSelectItem(item)}
                        className={`relative flex-shrink-0 w-12 h-12 rounded-lg overflow-hidden cursor-pointer transition-all ${
                          selectedItem?.id === item.id
                            ? 'ring-2 ring-purple-500 scale-105'
                            : 'hover:scale-105'
                        } ${offset === 0 ? 'opacity-100' : 'opacity-60'}`}
                      >
                        <img
                          src={item.url}
                          alt={`${label} ${index + 1}`}
                          className="w-full h-full object-cover"
                        />
                        {selectedItem?.id === item.id && (
                          <div className="absolute inset-0 bg-purple-500/20 flex items-center justify-center">
                            <Check size={12} className="text-white" />
                          </div>
                        )}
                      </div>
                    );
                  })}
                </>
              ) : (
                <div className="flex-1 flex space-x-1">
                  {/* Empty placeholders */}
                  {[1, 2, 3].map((i) => (
                    <div
                      key={i}
                      className="w-12 h-12 rounded-lg bg-gray-100 border-2 border-dashed border-gray-300 flex items-center justify-center"
                    >
                      <span className="text-xs text-gray-400">Img{i}</span>
                    </div>
                  ))}
                </div>
              )}
            </div>

            {/* Next Button */}
            <button
              onClick={handleNext}
              disabled={items.length <= 1}
              className="p-1 text-gray-500 hover:text-gray-700 disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
            >
              <ChevronRight size={16} />
            </button>
          </div>

          {/* Upload/Create Button */}
          <button
            onClick={handleGenerateItem}
            className="w-full py-2 px-3 bg-gray-50 hover:bg-gray-100 border border-gray-200 rounded-lg text-xs text-gray-600 flex items-center justify-center space-x-1 transition-colors"
          >
            <Upload size={12} />
            <span>Upload or create</span>
          </button>
        </div>
      )}
    </motion.div>
  );
};

export default CategoryCarousel;
