import React from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { ChevronLeft, ChevronRight } from 'lucide-react';

import { ItemInfo } from '../types/api';

interface ImageGalleryProps {
  items: ItemInfo[];
  currentIndex: number;
  selectedItem?: ItemInfo | null;
  onPrevious: () => void;
  onNext: () => void;
  onSelect?: (item: ItemInfo) => void;
  className?: string;
}

const ImageGallery: React.FC<ImageGalleryProps> = ({
  items,
  currentIndex,
  selectedItem,
  onPrevious,
  onNext,
  onSelect,
  className = '',
}) => {
  if (items.length === 0) {
    return (
      <div className={`flex items-center justify-center bg-gray-100 rounded-lg ${className}`}>
        <div className="text-center py-8">
          <div className="w-16 h-16 bg-gray-200 rounded-lg mx-auto mb-3 flex items-center justify-center">
            <svg className="w-8 h-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
          </div>
          <p className="text-gray-500 text-sm">No images available</p>
        </div>
      </div>
    );
  }

  const currentItem = items[currentIndex];
  const canNavigate = items.length > 1;

  return (
    <div className={`relative overflow-hidden ${className}`}>
      {/* Main Image Display */}
      <div className="relative aspect-square bg-gray-100 rounded-lg overflow-hidden">
        <AnimatePresence mode="wait">
          {currentItem && (
            <motion.div
              key={currentItem.id}
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -20 }}
              transition={{ duration: 0.2 }}
              className="absolute inset-0"
              onClick={() => onSelect?.(currentItem)}
            >
              <img
                src={currentItem.url}
                alt={currentItem.filename || 'Gallery item'}
                className={`w-full h-full object-cover transition-all duration-200 ${
                  onSelect ? 'cursor-pointer hover:scale-105' : ''
                } ${
                  selectedItem?.id === currentItem.id ? 'ring-4 ring-purple-500' : ''
                }`}
                loading="lazy"
              />

              {/* Selection Overlay */}
              {selectedItem?.id === currentItem.id && (
                <motion.div
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  className="absolute inset-0 bg-purple-500/20 flex items-center justify-center"
                >
                  <div className="bg-purple-500 text-white rounded-full p-3">
                    <svg className="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
                      <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                    </svg>
                  </div>
                </motion.div>
              )}
            </motion.div>
          )}
        </AnimatePresence>

        {/* Navigation Buttons */}
        {canNavigate && (
          <>
            <motion.button
              whileHover={{ scale: 1.1 }}
              whileTap={{ scale: 0.95 }}
              onClick={onPrevious}
              className="absolute left-2 top-1/2 -translate-y-1/2 bg-white/80 hover:bg-white text-gray-700 rounded-full p-2 shadow-lg transition-all duration-200 backdrop-blur-sm"
            >
              <ChevronLeft size={20} />
            </motion.button>

            <motion.button
              whileHover={{ scale: 1.1 }}
              whileTap={{ scale: 0.95 }}
              onClick={onNext}
              className="absolute right-2 top-1/2 -translate-y-1/2 bg-white/80 hover:bg-white text-gray-700 rounded-full p-2 shadow-lg transition-all duration-200 backdrop-blur-sm"
            >
              <ChevronRight size={20} />
            </motion.button>
          </>
        )}

        {/* Progress Indicators */}
        {canNavigate && (
          <div className="absolute bottom-2 left-1/2 -translate-x-1/2 flex space-x-1">
            {items.map((_, index) => (
              <div
                key={index}
                className={`w-2 h-2 rounded-full transition-all duration-200 ${
                  index === currentIndex ? 'bg-white' : 'bg-white/50'
                }`}
              />
            ))}
          </div>
        )}
      </div>

      {/* Image Info */}
      {currentItem && (
        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          className="mt-2 text-center"
        >
          <div className="text-xs text-gray-500">
            {currentIndex + 1} of {items.length}
          </div>
          {currentItem.prompt && (
            <div className="text-xs text-gray-600 mt-1 truncate" title={currentItem.prompt}>
              {currentItem.prompt}
            </div>
          )}
        </motion.div>
      )}
    </div>
  );
};

export default ImageGallery;
