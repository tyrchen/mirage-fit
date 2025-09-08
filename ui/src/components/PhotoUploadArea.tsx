import React, { useCallback } from 'react';
import { motion } from 'framer-motion';
import { Upload, User, Loader } from 'lucide-react';
import { useDropzone } from 'react-dropzone';
import toast from 'react-hot-toast';
import { useAppStore } from '../store/appStore';
import { api } from '../services/api';

const PhotoUploadArea: React.FC = () => {
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

    if (!file.type.startsWith('image/')) {
      toast.error('Please upload an image file');
      return;
    }

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
    <motion.div
      initial={{ opacity: 0, scale: 0.9 }}
      animate={{ opacity: 1, scale: 1 }}
      className="relative"
    >
      <div
        {...getRootProps()}
        className={`relative w-64 h-64 rounded-2xl border-2 border-dashed transition-all duration-300 cursor-pointer overflow-hidden ${
          isDragActive && !isDragReject
            ? 'border-purple-500 bg-purple-50 scale-105'
            : isDragReject
            ? 'border-red-500 bg-red-50'
            : uploadedPhoto
            ? 'border-purple-500 bg-purple-50/50'
            : 'border-gray-300 bg-white hover:border-purple-400 hover:bg-purple-50/50'
        }`}
      >
        <input {...getInputProps()} />

        {loading.uploading ? (
          <div className="absolute inset-0 flex flex-col items-center justify-center">
            <Loader className="w-10 h-10 text-purple-600 animate-spin mb-2" />
            <p className="text-sm text-gray-600">Uploading...</p>
          </div>
        ) : uploadedPhoto && uploadedPhoto.url ? (
          <>
            <img
              src={uploadedPhoto.url}
              alt="Uploaded photo"
              className="w-full h-full object-cover"
            />
            <div className="absolute inset-0 bg-gradient-to-t from-black/40 to-transparent opacity-0 hover:opacity-100 transition-opacity duration-200 flex items-end justify-center pb-4">
              <div className="bg-white/90 backdrop-blur-sm rounded-lg px-4 py-2 text-center">
                <Upload className="w-5 h-5 mx-auto mb-1 text-gray-600" />
                <p className="text-xs text-gray-600">Click to replace</p>
              </div>
            </div>
          </>
        ) : (
          <div className="absolute inset-0 flex flex-col items-center justify-center">
            {isDragActive ? (
              isDragReject ? (
                <>
                  <div className="w-14 h-14 bg-red-100 rounded-full flex items-center justify-center mb-3">
                    <svg className="w-7 h-7 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </div>
                  <p className="text-sm text-red-600 font-medium">Invalid file type</p>
                </>
              ) : (
                <>
                  <motion.div
                    animate={{ scale: [1, 1.1, 1] }}
                    transition={{ repeat: Infinity, duration: 1.5 }}
                    className="w-14 h-14 bg-purple-100 rounded-full flex items-center justify-center mb-3"
                  >
                    <Upload className="w-7 h-7 text-purple-600" />
                  </motion.div>
                  <p className="text-sm text-purple-600 font-medium">Drop photo here</p>
                </>
              )
            ) : (
              <>
                <div className="w-14 h-14 bg-gray-100 rounded-full flex items-center justify-center mb-3">
                  <User className="w-8 h-8 text-gray-400" />
                </div>
                <p className="text-sm text-gray-700 font-medium mb-1">Upload photo</p>
                <p className="text-xs text-gray-500 text-center px-4">
                  Click or drag to upload
                </p>
              </>
            )}
          </div>
        )}
      </div>

      {uploadedPhoto && (
        <motion.div
          initial={{ scale: 0 }}
          animate={{ scale: 1 }}
          className="absolute -bottom-2 -right-2 bg-green-500 text-white rounded-full p-1.5 shadow-lg"
        >
          <svg className="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
            <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
          </svg>
        </motion.div>
      )}
    </motion.div>
  );
};

export default PhotoUploadArea;
