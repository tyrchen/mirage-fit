import React, { useState, useMemo } from 'react';
import { motion } from 'framer-motion';
import { Sparkles, Settings, Loader, X } from 'lucide-react';
import toast from 'react-hot-toast';
import { useAppStore } from '../store/appStore';
import { api } from '../services/api';
import { Button } from './ui/button';
import { Input } from './ui/input';
import { Label } from './ui/label';
import { Slider } from './ui/slider';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from './ui/popover';
import { ItemCategory } from '../types/api';

const SelectedItemsSection: React.FC = () => {
  const {
    uploadedPhoto,
    selectedItems,
    loading,
    outputs,
    setLoading,
    setError,
    setOutputs,
    getSelectedItemsForRemix,
    setSelectedItem,
  } = useAppStore();

  const [remixOptions, setRemixOptions] = useState({
    style: '',
    quality: 8,
  });

  const [showAdvanced, setShowAdvanced] = useState(false);

  const getCategoryLabel = (category: ItemCategory): string => {
    const labels: Record<ItemCategory, string> = {
      [ItemCategory.Hat]: 'hat',
      [ItemCategory.Glasses]: 'glasses',
      [ItemCategory.Top]: 'top',
      [ItemCategory.Accessory]: 'accessory',
      [ItemCategory.BottomSkirt]: 'bottom',
      [ItemCategory.Shoes]: 'shoes',
      [ItemCategory.Socks]: 'socks',
      [ItemCategory.Gloves]: 'gloves',
      [ItemCategory.Scarf]: 'scarf',
      [ItemCategory.Bag]: 'bag',
      [ItemCategory.Other]: 'other',
    };
    return labels[category] || category;
  };

  const selectedItemsArray = getSelectedItemsForRemix();
  const canRemix = uploadedPhoto && selectedItemsArray.length > 0;

  const selectedItemsInfo = useMemo(() => {
    return Object.entries(selectedItems)
      .filter(([_, item]) => item !== null)
      .map(([category, item]) => ({
        category: category as ItemCategory,
        item: item!,
        label: getCategoryLabel(category as ItemCategory)
      }));
  }, [selectedItems]);

  const handleRemix = async () => {
    if (!uploadedPhoto) {
      toast.error('Please upload a photo first');
      return;
    }

    if (selectedItemsArray.length === 0) {
      toast.error('Please select at least one item');
      return;
    }

    setLoading('remixing', true);

    try {
      const remixRequest = {
        base_image: uploadedPhoto.hash,
        items: selectedItemsArray,
        style: remixOptions.style || undefined,
        quality: remixOptions.quality,
      };

      const response = await api.generateRemix(remixRequest);

      const updatedOutputs = [
        {
          id: response.id,
          hash: response.hash,
          dimensions: response.dimensions,
          created_at: new Date().toISOString(),
          source_images: response.source_images,
          url: response.url,
        },
        ...outputs,
      ];

      setOutputs(updatedOutputs);
      toast.success('Remix generated successfully!');
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Remix failed';
      toast.error(message);
      setError({
        message: `Remix failed: ${message}`,
        type: 'api',
      });
    } finally {
      setLoading('remixing', false);
    }
  };

  const handleRemoveItem = (category: ItemCategory) => {
    setSelectedItem(category, null);
    toast.success('Item removed');
  };

  return (
    <div className="bg-white/95 backdrop-blur-sm rounded-2xl shadow-xl p-6 mb-8">
      {/* Header */}
      <h3 className="text-lg font-semibold text-gray-800 mb-4">Selected Items</h3>

      {/* Selected Items Display */}
      <div className="mb-6">
        {selectedItemsInfo.length > 0 ? (
          <div className="flex flex-wrap gap-3">
            {selectedItemsInfo.map(({ category, item, label }) => (
              <motion.div
                key={category}
                initial={{ scale: 0, opacity: 0 }}
                animate={{ scale: 1, opacity: 1 }}
                exit={{ scale: 0, opacity: 0 }}
                className="bg-purple-50 border border-purple-200 rounded-lg p-2 flex items-center space-x-2"
              >
                <div
                  className="w-10 h-10 rounded-lg bg-cover bg-center"
                  style={{ backgroundImage: `url(${item.url})` }}
                />
                <div className="flex flex-col">
                  <span className="text-xs font-medium text-purple-700">{label}</span>
                  <span className="text-xs text-gray-500">{item.filename || 'Generated'}</span>
                </div>
                <button
                  onClick={() => handleRemoveItem(category)}
                  className="ml-2 p-1 hover:bg-purple-100 rounded-full transition-colors"
                >
                  <X size={14} className="text-gray-500 hover:text-red-500" />
                </button>
              </motion.div>
            ))}
          </div>
        ) : (
          <div className="py-8 text-center bg-gray-50 rounded-lg border-2 border-dashed border-gray-200">
            <p className="text-gray-500">No items selected</p>
            <p className="text-sm text-gray-400 mt-1">Select items from the categories above to generate a remix</p>
          </div>
        )}
      </div>

      {/* Remix Controls */}
      <div className="flex items-center space-x-3">
        {/* Advanced Options */}
        <Popover open={showAdvanced} onOpenChange={setShowAdvanced}>
          <PopoverTrigger asChild>
            <Button variant="outline" size="sm" className="flex items-center space-x-1">
              <Settings size={16} />
              <span>Advanced Options</span>
            </Button>
          </PopoverTrigger>
          <PopoverContent className="w-80" align="start">
            <div className="space-y-4">
              <div>
                <Label htmlFor="style" className="text-sm font-medium">
                  Style Prompt
                </Label>
                <Input
                  id="style"
                  placeholder="e.g., vintage, modern, casual..."
                  value={remixOptions.style}
                  onChange={(e) =>
                    setRemixOptions(prev => ({ ...prev, style: e.target.value }))
                  }
                  className="mt-1"
                />
                <p className="text-xs text-gray-500 mt-1">
                  Describe the overall style you want
                </p>
              </div>

              <div>
                <Label className="text-sm font-medium">
                  Quality: {remixOptions.quality}
                </Label>
                <Slider
                  value={[remixOptions.quality]}
                  onValueChange={([value]) =>
                    setRemixOptions(prev => ({ ...prev, quality: value }))
                  }
                  min={1}
                  max={10}
                  step={1}
                  className="mt-2"
                />
                <div className="flex justify-between text-xs text-gray-500 mt-1">
                  <span>Fast</span>
                  <span>High Quality</span>
                </div>
              </div>
            </div>
          </PopoverContent>
        </Popover>

        {/* Generate Remix Button */}
        <Button
          onClick={handleRemix}
          disabled={!canRemix || loading.remixing}
          className="flex-1 bg-gradient-to-r from-purple-500 to-pink-500 hover:from-purple-600 hover:to-pink-600 text-white font-medium"
        >
          {loading.remixing ? (
            <>
              <Loader className="w-4 h-4 mr-2 animate-spin" />
              <span>Generating Remix...</span>
            </>
          ) : (
            <>
              <Sparkles className="w-4 h-4 mr-2" />
              <span>Generate Remix</span>
            </>
          )}
        </Button>
      </div>

      {/* Helper Text */}
      {!canRemix && (
        <p className="text-xs text-gray-500 text-center mt-3">
          {!uploadedPhoto
            ? 'Upload a photo first to generate a remix'
            : 'Select at least one item to generate a remix'
          }
        </p>
      )}
    </div>
  );
};

export default SelectedItemsSection;
