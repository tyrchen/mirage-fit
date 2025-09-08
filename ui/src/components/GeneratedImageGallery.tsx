import React, { useEffect, useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Download, Share2, Eye, Calendar, Grid3X3, List } from 'lucide-react';
import toast from 'react-hot-toast';

import { useAppStore } from '../store/appStore';
import { api } from '../services/api';
import { OutputInfo } from '../types/api';
import LoadingSpinner from './ui/LoadingSpinner';
import { Button } from './ui/button';

interface ImageModalProps {
  image: OutputInfo;
  isOpen: boolean;
  onClose: () => void;
}

const ImageModal: React.FC<ImageModalProps> = ({ image, isOpen, onClose }) => {
  if (!isOpen) return null;

  const handleDownload = () => {
    // Create a temporary link to download the image
    const link = document.createElement('a');
    link.href = image.url;
    link.download = `mirage-fit-remix-${image.id}.jpg`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    toast.success('Download started!');
  };

  const handleShare = async () => {
    if (navigator.share) {
      try {
        await navigator.share({
          title: 'Mirage Fit Remix',
          text: 'Check out this AI-generated fashion remix!',
          url: window.location.href,
        });
      } catch (error) {
        // User cancelled or error occurred
      }
    } else {
      // Fallback: copy URL to clipboard
      navigator.clipboard.writeText(window.location.href);
      toast.success('Link copied to clipboard!');
    }
  };

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      className="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4"
      onClick={onClose}
    >
      <motion.div
        initial={{ scale: 0.9, opacity: 0 }}
        animate={{ scale: 1, opacity: 1 }}
        exit={{ scale: 0.9, opacity: 0 }}
        className="bg-white rounded-xl p-4 max-w-4xl max-h-[90vh] overflow-auto"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex flex-col lg:flex-row gap-4">
          {/* Image */}
          <div className="flex-1">
            <img
              src={image.url}
              alt={`Remix ${image.id}`}
              className="w-full h-auto max-h-96 object-contain rounded-lg"
            />
          </div>

          {/* Info & Actions */}
          <div className="w-full lg:w-80 space-y-4">
            <div>
              <h3 className="text-lg font-semibold text-gray-900 mb-2">Remix Details</h3>

              <div className="space-y-2 text-sm text-gray-600">
                <div className="flex items-center space-x-2">
                  <Calendar size={14} />
                  <span>{new Date(image.created_at).toLocaleDateString()}</span>
                </div>

                <div>
                  <strong>Dimensions:</strong> {image.dimensions[0]} × {image.dimensions[1]}
                </div>

                <div>
                  <strong>Sources:</strong> {image.source_images.length} images
                </div>
              </div>
            </div>

            {/* Actions */}
            <div className="space-y-2">
              <Button onClick={handleDownload} className="w-full" variant="outline">
                <Download size={16} className="mr-2" />
                Download
              </Button>

              <Button onClick={handleShare} className="w-full" variant="outline">
                <Share2 size={16} className="mr-2" />
                Share
              </Button>

              <Button onClick={onClose} className="w-full">
                Close
              </Button>
            </div>
          </div>
        </div>
      </motion.div>
    </motion.div>
  );
};

const GeneratedImageGallery: React.FC = () => {
  const { outputs, setOutputs, setError } = useAppStore();
  const [loading, setLoading] = useState(false);
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid');
  const [selectedImage, setSelectedImage] = useState<OutputInfo | null>(null);

  // Load outputs on component mount
  useEffect(() => {
    const loadOutputs = async () => {
      if (outputs.length > 0) return; // Already loaded

      setLoading(true);
      try {
        const response = await api.getOutputs();
        setOutputs(response.outputs);
      } catch (error) {
        setError({
          message: error instanceof Error ? error.message : 'Failed to load generated images',
          type: 'api',
        });
      } finally {
        setLoading(false);
      }
    };

    loadOutputs();
  }, [outputs.length, setOutputs, setError]);

  if (loading) {
    return (
      <div className="bg-white/90 backdrop-blur-sm rounded-xl p-8 shadow-lg border border-white/20">
        <div className="text-center">
          <LoadingSpinner size="lg" className="text-purple-600 mx-auto mb-4" />
          <p className="text-gray-600">Loading generated images...</p>
        </div>
      </div>
    );
  }

  if (outputs.length === 0) {
    return (
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="bg-white/90 backdrop-blur-sm rounded-xl p-8 shadow-lg border border-white/20 text-center"
      >
        <div className="w-16 h-16 bg-gradient-to-br from-purple-100 to-pink-100 rounded-full mx-auto mb-4 flex items-center justify-center">
          <Eye className="w-8 h-8 text-purple-500" />
        </div>
        <h3 className="text-lg font-semibold text-gray-900 mb-2">No Generated Images Yet</h3>
        <p className="text-gray-600 mb-4">
          Upload a photo and select some fashion items to create your first remix!
        </p>
      </motion.div>
    );
  }

  return (
    <>
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="bg-white/90 backdrop-blur-sm rounded-xl p-6 shadow-lg border border-white/20"
      >
        {/* Header */}
        <div className="flex items-center justify-between mb-6">
          <div>
            <h2 className="text-xl font-bold text-gray-900 mb-1">
              Generated Remixes
            </h2>
            <p className="text-sm text-gray-600">
              {outputs.length} image{outputs.length !== 1 ? 's' : ''} generated
            </p>
          </div>

          {/* View Mode Toggle */}
          <div className="flex items-center space-x-1 bg-gray-100 rounded-lg p-1">
            <button
              onClick={() => setViewMode('grid')}
              className={`p-2 rounded-md transition-colors ${
                viewMode === 'grid'
                  ? 'bg-white text-gray-900 shadow-sm'
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              <Grid3X3 size={16} />
            </button>
            <button
              onClick={() => setViewMode('list')}
              className={`p-2 rounded-md transition-colors ${
                viewMode === 'list'
                  ? 'bg-white text-gray-900 shadow-sm'
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              <List size={16} />
            </button>
          </div>
        </div>

        {/* Images Grid */}
        <div className={viewMode === 'grid'
          ? "grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4"
          : "space-y-4"
        }>
          {outputs.map((image, index) => (
            <motion.div
              key={image.id}
              initial={{ opacity: 0, scale: 0.9 }}
              animate={{ opacity: 1, scale: 1 }}
              transition={{ delay: index * 0.05 }}
              className={viewMode === 'grid'
                ? "group cursor-pointer"
                : "flex items-center space-x-4 p-4 rounded-lg hover:bg-gray-50 cursor-pointer"
              }
              onClick={() => setSelectedImage(image)}
            >
              {viewMode === 'grid' ? (
                <div className="relative aspect-square bg-gray-100 rounded-lg overflow-hidden">
                  <img
                    src={image.url}
                    alt={`Remix ${image.id}`}
                    className="w-full h-full object-cover transition-transform duration-200 group-hover:scale-105"
                    loading="lazy"
                  />

                  {/* Overlay */}
                  <div className="absolute inset-0 bg-black/0 group-hover:bg-black/20 transition-all duration-200 flex items-center justify-center">
                    <motion.div
                      initial={{ opacity: 0, scale: 0.8 }}
                      whileHover={{ opacity: 1, scale: 1 }}
                      className="bg-white text-gray-900 rounded-full p-2 shadow-lg opacity-0 group-hover:opacity-100 transition-opacity"
                    >
                      <Eye size={16} />
                    </motion.div>
                  </div>

                  {/* Date Badge */}
                  <div className="absolute top-2 left-2 bg-white/90 backdrop-blur-sm text-xs text-gray-600 px-2 py-1 rounded-full">
                    {new Date(image.created_at).toLocaleDateString()}
                  </div>
                </div>
              ) : (
                <>
                  <div className="w-16 h-16 bg-gray-100 rounded-lg overflow-hidden flex-shrink-0">
                    <img
                      src={image.url}
                      alt={`Remix ${image.id}`}
                      className="w-full h-full object-cover"
                      loading="lazy"
                    />
                  </div>

                  <div className="flex-1 min-w-0">
                    <div className="text-sm font-medium text-gray-900 truncate">
                      Remix {image.id.slice(-8)}
                    </div>
                    <div className="text-xs text-gray-500">
                      {new Date(image.created_at).toLocaleDateString()} •{' '}
                      {image.dimensions[0]} × {image.dimensions[1]} •{' '}
                      {image.source_images.length} sources
                    </div>
                  </div>

                  <Eye size={16} className="text-gray-400" />
                </>
              )}
            </motion.div>
          ))}
        </div>
      </motion.div>

      {/* Image Modal */}
      <AnimatePresence>
        {selectedImage && (
          <ImageModal
            image={selectedImage}
            isOpen={true}
            onClose={() => setSelectedImage(null)}
          />
        )}
      </AnimatePresence>
    </>
  );
};

export default GeneratedImageGallery;
