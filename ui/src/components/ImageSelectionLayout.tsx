import React from 'react';
import { motion } from 'framer-motion';
import { CategoryInfo } from '../types/api';
import { ItemCategory } from '../types/api';
import PhotoUploadArea from './PhotoUploadArea';
import CategoryCarousel from './CategoryCarousel';
import SelectedItemsSection from './SelectedItemsSection';
import GeneratedRemixes from './GeneratedRemixes';

interface ImageSelectionLayoutProps {
  categories: CategoryInfo[];
}

const ImageSelectionLayout: React.FC<ImageSelectionLayoutProps> = ({ categories }) => {
  // Define the layout for categories around the central upload area
  const categoryConfig = [
    {
      id: ItemCategory.Hat,
      label: '帽子',
      position: 'col-start-2 row-start-1',
      align: 'justify-center'
    },
    {
      id: ItemCategory.Glasses,
      label: '眼镜',
      position: 'col-start-3 row-start-1',
      align: 'justify-center'
    },
    {
      id: ItemCategory.Accessory,
      label: '首饰',
      position: 'col-start-1 row-start-2',
      align: 'justify-end'
    },
    {
      id: ItemCategory.Top,
      label: '上衣',
      position: 'col-start-3 row-start-2',
      align: 'justify-start'
    },
    {
      id: ItemCategory.Shoes,
      label: '鞋',
      position: 'col-start-1 row-start-3',
      align: 'justify-end'
    },
    {
      id: ItemCategory.BottomSkirt,
      label: '裤子/裙子',
      position: 'col-start-3 row-start-3',
      align: 'justify-start'
    }
  ];

  return (
    <div className="max-w-7xl mx-auto">
      {/* Main Image Selection Area */}
      <div className="bg-white/95 backdrop-blur-sm rounded-2xl shadow-xl p-8 mb-8">
        <motion.h2
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center text-2xl font-semibold mb-8 text-gray-800"
        >
          Image Selection
        </motion.h2>

        {/* Desktop Layout - Categories around center */}
        <div className="hidden lg:block">
          <div className="relative max-w-4xl mx-auto">
            {/* Grid Layout */}
            <div className="grid grid-cols-5 gap-4">
              {/* Top Row */}
              <div className="col-span-1"></div>
              <div className="col-span-1 flex justify-center">
                {(() => {
                  const hat = categoryConfig.find(c => c.id === ItemCategory.Hat);
                  const categoryData = categories.find(cat => cat.id === hat?.id);
                  return hat ? (
                    <CategoryCarousel
                      category={hat.id}
                      label={hat.label}
                      categoryData={categoryData}
                    />
                  ) : null;
                })()}
              </div>
              <div className="col-span-1"></div>
              <div className="col-span-1 flex justify-center">
                {(() => {
                  const glasses = categoryConfig.find(c => c.id === ItemCategory.Glasses);
                  const categoryData = categories.find(cat => cat.id === glasses?.id);
                  return glasses ? (
                    <CategoryCarousel
                      category={glasses.id}
                      label={glasses.label}
                      categoryData={categoryData}
                    />
                  ) : null;
                })()}
              </div>
              <div className="col-span-1"></div>

              {/* Middle Row */}
              <div className="col-span-1 flex justify-end items-center">
                {(() => {
                  const accessory = categoryConfig.find(c => c.id === ItemCategory.Accessory);
                  const categoryData = categories.find(cat => cat.id === accessory?.id);
                  return accessory ? (
                    <CategoryCarousel
                      category={accessory.id}
                      label={accessory.label}
                      categoryData={categoryData}
                    />
                  ) : null;
                })()}
              </div>
              <div className="col-span-3 flex justify-center items-center py-8">
                <PhotoUploadArea />
              </div>
              <div className="col-span-1 flex justify-start items-center">
                {(() => {
                  const top = categoryConfig.find(c => c.id === ItemCategory.Top);
                  const categoryData = categories.find(cat => cat.id === top?.id);
                  return top ? (
                    <CategoryCarousel
                      category={top.id}
                      label={top.label}
                      categoryData={categoryData}
                    />
                  ) : null;
                })()}
              </div>

              {/* Bottom Row */}
              <div className="col-span-1"></div>
              <div className="col-span-1 flex justify-center">
                {(() => {
                  const shoes = categoryConfig.find(c => c.id === ItemCategory.Shoes);
                  const categoryData = categories.find(cat => cat.id === shoes?.id);
                  return shoes ? (
                    <CategoryCarousel
                      category={shoes.id}
                      label={shoes.label}
                      categoryData={categoryData}
                    />
                  ) : null;
                })()}
              </div>
              <div className="col-span-1"></div>
              <div className="col-span-1 flex justify-center">
                {(() => {
                  const bottomSkirt = categoryConfig.find(c => c.id === ItemCategory.BottomSkirt);
                  const categoryData = categories.find(cat => cat.id === bottomSkirt?.id);
                  return bottomSkirt ? (
                    <CategoryCarousel
                      category={bottomSkirt.id}
                      label={bottomSkirt.label}
                      categoryData={categoryData}
                    />
                  ) : null;
                })()}
              </div>
              <div className="col-span-1"></div>
            </div>
          </div>
        </div>

        {/* Mobile Layout */}
        <div className="block lg:hidden">
          {/* Central Upload Area */}
          <div className="mb-8 flex justify-center">
            <PhotoUploadArea />
          </div>

          {/* Categories Grid for Mobile */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {categoryConfig.map((config) => {
              const categoryData = categories.find(cat => cat.id === config.id);
              return (
                <div key={config.id} className="flex justify-center">
                  <CategoryCarousel
                    category={config.id}
                    label={config.label}
                    categoryData={categoryData}
                  />
                </div>
              );
            })}
          </div>
        </div>
      </div>

      {/* Selected Items Section */}
      <SelectedItemsSection />

      {/* Generated Remixes Section */}
      <GeneratedRemixes />
    </div>
  );
};

export default ImageSelectionLayout;
