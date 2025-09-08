import React, { useCallback } from 'react';
import { motion } from 'framer-motion';
import { Upload, User, Sparkles, Loader } from 'lucide-react';
import { useDropzone } from 'react-dropzone';
import toast from 'react-hot-toast';

import { useAppStore } from '../store/appStore';
import { api } from '../services/api';
import RemixControls from './RemixControls';

const CentralCharacter: React.FC = () => {
  const {
    uploadedPhoto,
    loading,
    setUploadedPhoto,
    setLoading,
    setError,
  } = useAppStore();

  const onDrop = useCallback(async (acceptedFiles: File[]) => {
    const file = acceptedFiles[0];
    if (!file) return;

    // Validate file type
    if (!file.type.startsWith('image/')) {
      toast.error('Please upload an image file');
      return;
    }

    // Validate file size (20MB limit)
    if (file.size > 20 * 1024 * 1024) {
      toast.error('Image must be less than 20MB');
      return;
    }

    setLoading('uploading', true);

    try {
      const response = await api.uploadPhoto(file);
      setUploadedPhoto(response);
      toast.success('Photo uploaded successfully!');
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Upload failed';
      toast.error(message);
      setError({
        message: `Upload failed: ${message}`,
        type: 'api',
      });
    } finally {
      setLoading('uploading', false);
    }
  }, [setLoading, setUploadedPhoto, setError]);

  const {
    getRootProps,
    getInputProps,
    isDragActive,
    isDragReject,
  } = useDropzone({
    onDrop,
    accept: {
      'image/*': ['.jpeg', '.jpg', '.png', '.webp'],
    },
    maxFiles: 1,
    disabled: loading.uploading,
  });

  return (
    <div className="flex flex-col items-center space-y-8 p-8">
      {/* Character/Photo Display */}
      <motion.div
        initial={{ opacity: 0, scale: 0.9 }}
        animate={{ opacity: 1, scale: 1 }}
        className="relative"
      >
        <div
          {...getRootProps()}
          className={`relative w-72 h-72 rounded-full border-4 border-dashed transition-all duration-300 cursor-pointer overflow-hidden shadow-xl ${
            isDragActive && !isDragReject
              ? 'border-purple-500 bg-purple-50 scale-105'
              : isDragReject
              ? 'border-red-500 bg-red-50'
              : uploadedPhoto
              ? 'border-green-500 bg-green-50'
              : 'border-gray-300 bg-white/80 hover:border-purple-400 hover:bg-purple-50/50'
          }`}
        >
          <input {...getInputProps()} />

          {loading.uploading ? (
            <div className="absolute inset-0 flex flex-col items-center justify-center">
              <Loader className="w-12 h-12 text-purple-600 animate-spin mb-2" />
              <p className="text-sm text-gray-600">Uploading...</p>
            </div>
          ) : uploadedPhoto && uploadedPhoto.url ? (
            <>
              <img
                src={uploadedPhoto.url}
                alt="Uploaded photo"
                className="w-full h-full object-cover"
                onError={() => {
                  console.error('Failed to load uploaded photo:', uploadedPhoto.url);
                  // You could set an error state here if needed
                }}
                onLoad={() => {
                  console.log('Successfully loaded photo:', uploadedPhoto.url);
                }}
              />
              <div className="absolute inset-0 bg-black/20 opacity-0 hover:opacity-100 transition-opacity duration-200 flex items-center justify-center">
                <div className="bg-white/90 backdrop-blur-sm rounded-lg p-3 text-center shadow-lg">
                  <Upload className="w-6 h-6 mx-auto mb-1 text-gray-600" />
                  <p className="text-xs text-gray-600">Click or drop to replace</p>
                </div>
              </div>
            </>
          ) : (
            <div className="absolute inset-0 flex flex-col items-center justify-center">
              {isDragActive ? (
                isDragReject ? (
                  <>
                    <div className="w-16 h-16 bg-red-100 rounded-full flex items-center justify-center mb-3">
                      <svg className="w-8 h-8 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                      </svg>
                    </div>
                    <p className="text-sm text-red-600 font-medium">Invalid file type</p>
                    <p className="text-xs text-red-500">Please drop an image file</p>
                  </>
                ) : (
                  <>
                    <motion.div
                      animate={{ scale: [1, 1.1, 1] }}
                      transition={{ repeat: Infinity, duration: 1.5 }}
                      className="w-16 h-16 bg-purple-100 rounded-full flex items-center justify-center mb-3"
                    >
                      <Upload className="w-8 h-8 text-purple-600" />
                    </motion.div>
                    <p className="text-sm text-purple-600 font-medium">Drop your photo here</p>
                  </>
                )
              ) : (
                <>
                  <div className="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mb-3">
                    <User className="w-8 h-8 text-gray-400" />
                  </div>
                  <p className="text-sm text-gray-600 font-medium mb-1">Upload your photo</p>
                  <p className="text-xs text-gray-500 text-center">
                    Click or drag an image here
                    <br />
                    <span className="text-xs">JPG, PNG, WebP (max 20MB)</span>
                  </p>
                </>
              )}
            </div>
          )}
        </div>

        {/* Upload Status Indicator */}
        {uploadedPhoto && (
          <motion.div
            initial={{ scale: 0 }}
            animate={{ scale: 1 }}
            className="absolute -bottom-2 -right-2 bg-green-500 text-white rounded-full p-2 shadow-lg"
          >
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
            </svg>
          </motion.div>
        )}
      </motion.div>

      {/* Remix Controls */}
      {uploadedPhoto && (
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.3 }}
        >
          <RemixControls />
        </motion.div>
      )}

      {/* Instructions */}
      {!uploadedPhoto && (
        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className="text-center max-w-sm"
        >
          <p className="text-gray-600 text-sm mb-2">
            Start by uploading your photo to see the magic happen
          </p>
          <div className="flex items-center justify-center space-x-2 text-xs text-gray-500">
            <Sparkles className="w-4 h-4" />
            <span>AI will remix your photo with selected fashion items</span>
          </div>
        </motion.div>
      )}
    </div>
  );
};

export default CentralCharacter;
