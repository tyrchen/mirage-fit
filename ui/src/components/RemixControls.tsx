import React, { useState, useMemo } from 'react';
import { motion } from 'framer-motion';
import { Sparkles, Loader, Settings, Palette } from 'lucide-react';
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

const RemixControls: React.FC = () => {
  const {
    uploadedPhoto,
    selectedItems,
    loading,
    outputs,
    setLoading,
    setError,
    setOutputs,
    getSelectedItemsForRemix,
  } = useAppStore();

  const [remixOptions, setRemixOptions] = useState({
    style: '',
    quality: 8,
  });

  const [showAdvanced, setShowAdvanced] = useState(false);

  const selectedItemsArray = getSelectedItemsForRemix();
  const canRemix = uploadedPhoto && selectedItemsArray.length > 0;

  const selectedItemsInfo = useMemo(() => {
    return Object.entries(selectedItems)
      .filter(([_, item]) => item !== null)
      .map(([category, item]) => ({ category, item: item! }));
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

      // Add the new output to the beginning of the list
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

  return (
    <div className="bg-white/90 backdrop-blur-sm rounded-xl p-4 shadow-lg border border-white/20 w-full max-w-md">
      {/* Selected Items Summary */}
      <div className="mb-4">
        <h3 className="text-sm font-medium text-gray-800 mb-2">Selected Items</h3>

        {selectedItemsInfo.length > 0 ? (
          <div className="flex flex-wrap gap-2">
            {selectedItemsInfo.map(({ category, item }) => (
              <motion.div
                key={category}
                initial={{ scale: 0 }}
                animate={{ scale: 1 }}
                className="bg-purple-100 text-purple-700 px-2 py-1 rounded-full text-xs font-medium flex items-center space-x-1"
              >
                <div
                  className="w-4 h-4 rounded-full bg-cover bg-center border border-purple-200"
                  style={{ backgroundImage: `url(${item.url})` }}
                />
                <span>{category.split('/')[0]}</span>
              </motion.div>
            ))}
          </div>
        ) : (
          <p className="text-gray-500 text-sm">Select items from the categories above</p>
        )}
      </div>

      {/* Advanced Options */}
      <Popover open={showAdvanced} onOpenChange={setShowAdvanced}>
        <PopoverTrigger asChild>
          <Button variant="outline" size="sm" className="mb-3 w-full">
            <Settings size={14} className="mr-1" />
            Advanced Options
          </Button>
        </PopoverTrigger>
        <PopoverContent className="w-80" align="center">
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

      {/* Remix Button */}
      <Button
        onClick={handleRemix}
        disabled={!canRemix || loading.remixing}
        className="w-full bg-gradient-to-r from-purple-500 to-pink-500 hover:from-purple-600 hover:to-pink-600 text-white font-medium py-2 rounded-lg transition-all duration-200 shadow-md hover:shadow-lg"
      >
        {loading.remixing ? (
          <>
            <Loader className="w-4 h-4 mr-2 animate-spin" />
            Generating Remix...
          </>
        ) : (
          <>
            <Sparkles className="w-4 h-4 mr-2" />
            Generate Remix
          </>
        )}
      </Button>

      {!canRemix && (
        <p className="text-xs text-gray-500 text-center mt-2">
          {!uploadedPhoto
            ? 'Upload a photo and select items to generate a remix'
            : 'Select at least one item to generate a remix'
          }
        </p>
      )}

      {/* Quick Stats */}
      {outputs.length > 0 && (
        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          className="mt-3 pt-3 border-t border-gray-200 text-center"
        >
          <div className="flex items-center justify-center space-x-2 text-xs text-gray-600">
            <Palette size={12} />
            <span>{outputs.length} remix{outputs.length !== 1 ? 'es' : ''} generated</span>
          </div>
        </motion.div>
      )}
    </div>
  );
};

export default RemixControls;
