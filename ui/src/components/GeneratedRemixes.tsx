import React from 'react';
import { motion } from 'framer-motion';
import { Download, Trash2, Grid3X3, List } from 'lucide-react';
import { useAppStore } from '../store/appStore';
import toast from 'react-hot-toast';

const GeneratedRemixes: React.FC = () => {
  const { outputs, setOutputs } = useAppStore();
  const [viewMode, setViewMode] = React.useState<'grid' | 'list'>('grid');

  const handleDownload = (url: string, id: string) => {
    const link = document.createElement('a');
    link.href = url;
    link.download = `remix-${id}.png`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    toast.success('Image downloaded');
  };

  const handleDelete = (id: string) => {
    setOutputs(outputs.filter(output => output.id !== id));
    toast.success('Remix deleted');
  };

  if (outputs.length === 0) {
    return null;
  }

  return (
    <div className="bg-white/95 backdrop-blur-sm rounded-2xl shadow-xl p-6">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div>
          <h3 className="text-lg font-semibold text-gray-800">Generated Remixes</h3>
          <p className="text-sm text-gray-500 mt-1">{outputs.length} images generated</p>
        </div>

        {/* View Mode Toggle */}
        <div className="flex items-center space-x-1 bg-gray-100 rounded-lg p-1">
          <button
            onClick={() => setViewMode('grid')}
            className={`p-1.5 rounded transition-colors ${
              viewMode === 'grid'
                ? 'bg-white text-purple-600 shadow-sm'
                : 'text-gray-500 hover:text-gray-700'
            }`}
          >
            <Grid3X3 size={16} />
          </button>
          <button
            onClick={() => setViewMode('list')}
            className={`p-1.5 rounded transition-colors ${
              viewMode === 'list'
                ? 'bg-white text-purple-600 shadow-sm'
                : 'text-gray-500 hover:text-gray-700'
            }`}
          >
            <List size={16} />
          </button>
        </div>
      </div>

      {/* Gallery */}
      <div className={
        viewMode === 'grid'
          ? 'grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4'
          : 'space-y-4'
      }>
        {outputs.map((output, index) => (
          <motion.div
            key={output.id}
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ delay: index * 0.05 }}
            className={
              viewMode === 'grid'
                ? 'group relative bg-gray-100 rounded-xl overflow-hidden shadow-md hover:shadow-xl transition-all'
                : 'flex items-start space-x-4 bg-gray-50 rounded-xl p-4 hover:bg-gray-100 transition-colors'
            }
          >
            {viewMode === 'grid' ? (
              <>
                {/* Grid View */}
                <div className="aspect-square relative">
                  <img
                    src={output.url}
                    alt={`Remix ${output.id}`}
                    className="w-full h-full object-cover"
                  />

                  {/* Overlay with actions */}
                  <div className="absolute inset-0 bg-gradient-to-t from-black/60 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-200">
                    <div className="absolute bottom-0 left-0 right-0 p-3 flex items-center justify-between">
                      <span className="text-xs text-white/80">
                        {new Date(output.created_at).toLocaleDateString()}
                      </span>
                      <div className="flex items-center space-x-2">
                        <button
                          onClick={() => handleDownload(output.url, output.id)}
                          className="p-1.5 bg-white/20 backdrop-blur-sm rounded-lg text-white hover:bg-white/30 transition-colors"
                        >
                          <Download size={14} />
                        </button>
                        <button
                          onClick={() => handleDelete(output.id)}
                          className="p-1.5 bg-white/20 backdrop-blur-sm rounded-lg text-white hover:bg-red-500/50 transition-colors"
                        >
                          <Trash2 size={14} />
                        </button>
                      </div>
                    </div>
                  </div>
                </div>

                {/* Invalid Date tag */}
                {index === 0 && (
                  <div className="absolute top-2 left-2 bg-purple-500 text-white text-xs px-2 py-1 rounded-full">
                    Latest
                  </div>
                )}
              </>
            ) : (
              <>
                {/* List View */}
                <img
                  src={output.url}
                  alt={`Remix ${output.id}`}
                  className="w-24 h-24 rounded-lg object-cover"
                />

                <div className="flex-1">
                  <div className="flex items-start justify-between">
                    <div>
                      <h4 className="font-medium text-gray-800">Remix {index + 1}</h4>
                      <p className="text-sm text-gray-500 mt-1">
                        Created {new Date(output.created_at).toLocaleString()}
                      </p>
                      <p className="text-xs text-gray-400 mt-2">
                        {output.dimensions[0]} × {output.dimensions[1]}
                      </p>
                    </div>

                    <div className="flex items-center space-x-2">
                      <button
                        onClick={() => handleDownload(output.url, output.id)}
                        className="p-2 text-gray-500 hover:text-purple-600 hover:bg-purple-50 rounded-lg transition-colors"
                      >
                        <Download size={16} />
                      </button>
                      <button
                        onClick={() => handleDelete(output.id)}
                        className="p-2 text-gray-500 hover:text-red-500 hover:bg-red-50 rounded-lg transition-colors"
                      >
                        <Trash2 size={16} />
                      </button>
                    </div>
                  </div>
                </div>
              </>
            )}
          </motion.div>
        ))}
      </div>
    </div>
  );
};

export default GeneratedRemixes;
