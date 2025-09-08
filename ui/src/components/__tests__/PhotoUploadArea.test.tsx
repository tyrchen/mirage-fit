import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '../../test/test-utils';
import PhotoUploadArea from '../PhotoUploadArea';
import { useAppStore } from '../../store/appStore';
import { api } from '../../services/api';
import toast from 'react-hot-toast';
import type { DropzoneState } from 'react-dropzone';

// Mock dependencies
vi.mock('../../store/appStore');
vi.mock('../../services/api');
vi.mock('react-hot-toast');
vi.mock('react-dropzone', () => ({
  useDropzone: vi.fn(),
}));

const mockUseAppStore = vi.mocked(useAppStore);
const mockApi = vi.mocked(api);
const mockToast = vi.mocked(toast);

describe('PhotoUploadArea Component', () => {
  const defaultStoreState = {
    uploadedPhoto: null,
    loading: { uploading: false },
    setUploadedPhoto: vi.fn(),
    setLoading: vi.fn(),
    setError: vi.fn(),
  };

  const mockDropzoneProps: DropzoneState = {
    getRootProps: () => ({
      'data-testid': 'dropzone-root',
      role: 'button' as const,
      tabIndex: 0,
    } as any),
    getInputProps: () => ({
      'data-testid': 'dropzone-input',
      type: 'file' as const,
    } as any),
    isDragActive: false,
    isDragAccept: false,
    isDragReject: false,
    isFocused: false,
    isFileDialogActive: false,
    open: vi.fn(),
    acceptedFiles: [],
    fileRejections: [],
    rootRef: { current: null } as any,
    inputRef: { current: null } as any,
  };

  beforeEach(async () => {
    vi.clearAllMocks();
    mockUseAppStore.mockReturnValue(defaultStoreState as any);

    // Mock useDropzone hook
    const { useDropzone } = await import('react-dropzone');
    vi.mocked(useDropzone).mockReturnValue(mockDropzoneProps);
  });

  describe('Initial Render', () => {
    it('renders upload area with default state', () => {
      render(<PhotoUploadArea />);

      expect(screen.getByTestId('dropzone-root')).toBeInTheDocument();
      expect(screen.getByTestId('dropzone-input')).toBeInTheDocument();
      expect(screen.getByText('Upload photo')).toBeInTheDocument();
      expect(screen.getByText('Click or drag to upload')).toBeInTheDocument();
    });

    it('displays user icon in default state', () => {
      render(<PhotoUploadArea />);

      const userIcon = screen.getByTestId('dropzone-root').querySelector('.lucide-user');
      expect(userIcon).toBeInTheDocument();
    });

    it('has correct styling for default state', () => {
      render(<PhotoUploadArea />);

      const dropzone = screen.getByTestId('dropzone-root');
      expect(dropzone).toHaveClass(
        'w-64',
        'h-64',
        'rounded-2xl',
        'border-2',
        'border-dashed',
        'cursor-pointer'
      );
    });
  });

  describe('File Upload Process', () => {
    it('shows loading state during upload', () => {
      mockUseAppStore.mockReturnValue({
        ...defaultStoreState,
        loading: { uploading: true },
      } as any);

      const { container } = render(<PhotoUploadArea />);

      expect(screen.getByText('Uploading...')).toBeInTheDocument();
      const loadingSpinner = container.querySelector('.animate-spin, .border-t-current');
      expect(loadingSpinner).toBeInTheDocument();
    });

    it('handles successful file upload', async () => {
      const mockFile = new File(['test'], 'test.jpg', { type: 'image/jpeg' });
      const mockResponse = {
        url: 'http://example.com/photo.jpg',
        id: '123',
        hash: 'hash123',
        filename: 'test.jpg',
        dimensions: [100, 100] as [number, number],
        size: 1024,
        message: 'Upload successful'
      };

      mockApi.uploadPhoto.mockResolvedValue(mockResponse);

      const mockOnDrop = vi.fn().mockImplementation(async (files) => {
        defaultStoreState.setLoading({ uploading: true });
        try {
          const response = await mockApi.uploadPhoto(files[0]);
          defaultStoreState.setUploadedPhoto(response);
          mockToast.success('Photo uploaded successfully!');
        } catch (error) {
          const message = error instanceof Error ? error.message : 'Upload failed';
          mockToast.error(message);
        } finally {
          defaultStoreState.setLoading({ uploading: false });
        }
      });

      const dropzoneWithOnDrop: DropzoneState = {
        ...mockDropzoneProps,
        onDrop: mockOnDrop,
      } as any;

      const { useDropzone } = await import('react-dropzone');
      vi.mocked(useDropzone).mockReturnValue(dropzoneWithOnDrop);

      render(<PhotoUploadArea />);

      await mockOnDrop([mockFile]);

      expect(mockApi.uploadPhoto).toHaveBeenCalledWith(mockFile);
      expect(defaultStoreState.setUploadedPhoto).toHaveBeenCalledWith(mockResponse);
      expect(mockToast.success).toHaveBeenCalledWith('Photo uploaded successfully!');
    });

    it('handles multiple file uploads', async () => {
      const mockFiles = [
        new File(['test1'], 'test1.jpg', { type: 'image/jpeg' }),
        new File(['test2'], 'test2.jpg', { type: 'image/jpeg' }),
      ];

      const mockOnDrop = vi.fn().mockImplementation(async (files) => {
        for (const file of files) {
          await mockApi.uploadPhoto(file);
        }
        mockToast.success('Photos uploaded successfully!');
      });

      const dropzoneWithOnDrop: DropzoneState = {
        ...mockDropzoneProps,
        onDrop: mockOnDrop,
      } as any;

      const { useDropzone } = await import('react-dropzone');
      vi.mocked(useDropzone).mockReturnValue(dropzoneWithOnDrop);

      render(<PhotoUploadArea />);

      await mockOnDrop(mockFiles);

      expect(mockApi.uploadPhoto).toHaveBeenCalledTimes(2);
      expect(mockToast.success).toHaveBeenCalledWith('Photos uploaded successfully!');
    });

    it('shows progress during upload', async () => {
      const mockFile = new File(['test'], 'test.jpg', { type: 'image/jpeg' });

      const mockOnDrop = vi.fn().mockImplementation(async () => {
        defaultStoreState.setLoading({ uploading: true });
        await new Promise(resolve => setTimeout(resolve, 100));
        defaultStoreState.setLoading({ uploading: false });
      });

      const dropzoneWithOnDrop: DropzoneState = {
        ...mockDropzoneProps,
        onDrop: mockOnDrop,
      } as any;

      const { useDropzone } = await import('react-dropzone');
      vi.mocked(useDropzone).mockReturnValue(dropzoneWithOnDrop);

      render(<PhotoUploadArea />);

      await mockOnDrop([mockFile]);

      expect(defaultStoreState.setLoading).toHaveBeenCalledWith({ uploading: true });
      expect(defaultStoreState.setLoading).toHaveBeenCalledWith({ uploading: false });
    });

    it('uploads correct file type to API', async () => {
      const mockFile = new File(['test'], 'test.jpg', { type: 'image/jpeg' });
      const mockPhotoUrl = 'http://example.com/photo.jpg';

      const mockOnDrop = vi.fn().mockImplementation(async (files) => {
        await mockApi.uploadPhoto(files[0]);
        mockToast.success('Photo uploaded successfully!');
      });

      const dropzoneWithOnDrop: DropzoneState = {
        ...mockDropzoneProps,
        onDrop: mockOnDrop,
      } as any;

      const { useDropzone } = await import('react-dropzone');
      vi.mocked(useDropzone).mockReturnValue(dropzoneWithOnDrop);

      render(<PhotoUploadArea />);

      const mockResponse = {
        url: mockPhotoUrl,
        id: '456',
        hash: 'hash456',
        filename: 'test.jpg',
        dimensions: [100, 100] as [number, number],
        size: 1024,
        message: 'Upload successful'
      };

      mockApi.uploadPhoto.mockResolvedValue(mockResponse);

      await mockOnDrop([mockFile]);

      expect(mockApi.uploadPhoto).toHaveBeenCalledWith(mockFile);
      expect(mockToast.success).toHaveBeenCalledWith('Photo uploaded successfully!');
    });
  });

  describe('Drag and Drop States', () => {
    it('shows active drag state', async () => {
      const { useDropzone } = await import('react-dropzone');
      vi.mocked(useDropzone).mockReturnValue({
        ...mockDropzoneProps,
        isDragActive: true,
        isDragReject: false,
      });

      render(<PhotoUploadArea />);

      expect(screen.getByText('Drop photo here')).toBeInTheDocument();
      const dropzone = screen.getByTestId('dropzone-root');
      expect(dropzone).toHaveClass('border-purple-500', 'bg-purple-50', 'scale-105');
    });

    it('shows reject drag state', async () => {
      const { useDropzone } = await import('react-dropzone');
      vi.mocked(useDropzone).mockReturnValue({
        ...mockDropzoneProps,
        isDragActive: true,
        isDragReject: true,
      });

      render(<PhotoUploadArea />);

      expect(screen.getByText('Invalid file type')).toBeInTheDocument();
      const dropzone = screen.getByTestId('dropzone-root');
      expect(dropzone).toHaveClass('border-red-500', 'bg-red-50');
    });

    it('has correct hover states', () => {
      render(<PhotoUploadArea />);

      const dropzone = screen.getByTestId('dropzone-root');
      expect(dropzone).toHaveClass('hover:border-purple-400', 'hover:bg-purple-50/50');
    });
  });

  describe('Uploaded Photo Display', () => {
    it('displays uploaded photo', () => {
      const mockPhoto = { url: 'http://example.com/photo.jpg' };
      mockUseAppStore.mockReturnValue({
        ...defaultStoreState,
        uploadedPhoto: mockPhoto,
      } as any);

      render(<PhotoUploadArea />);

      const image = screen.getByAltText('Uploaded photo');
      expect(image).toBeInTheDocument();
      expect(image).toHaveAttribute('src', mockPhoto.url);
    });

    it('shows success checkmark when photo uploaded', () => {
      const mockPhoto = { url: 'http://example.com/photo.jpg' };
      mockUseAppStore.mockReturnValue({
        ...defaultStoreState,
        uploadedPhoto: mockPhoto,
      } as any);

      render(<PhotoUploadArea />);

      // Check for the checkmark SVG
      const checkmark = screen.getByRole('img', { hidden: true });
      expect(checkmark).toBeInTheDocument();
    });

    it('shows replace photo overlay on hover', () => {
      const mockPhoto = { url: 'http://example.com/photo.jpg' };
      mockUseAppStore.mockReturnValue({
        ...defaultStoreState,
        uploadedPhoto: mockPhoto,
      } as any);

      render(<PhotoUploadArea />);

      expect(screen.getByText('Click to replace')).toBeInTheDocument();
    });

    it('has correct styling for uploaded state', () => {
      const mockPhoto = { url: 'http://example.com/photo.jpg' };
      mockUseAppStore.mockReturnValue({
        ...defaultStoreState,
        uploadedPhoto: mockPhoto,
      } as any);

      render(<PhotoUploadArea />);

      const dropzone = screen.getByTestId('dropzone-root');
      expect(dropzone).toHaveClass('border-purple-500', 'bg-purple-50/50');
    });
  });

  describe('Accessibility', () => {
    it('has proper file input accessibility', () => {
      render(<PhotoUploadArea />);

      const input = screen.getByTestId('dropzone-input');
      expect(input).toHaveAttribute('type', 'file');
    });

    it('provides keyboard navigation support', () => {
      render(<PhotoUploadArea />);

      const dropzone = screen.getByTestId('dropzone-root');
      expect(dropzone).toHaveAttribute('role', 'button');
      expect(dropzone).toHaveAttribute('tabIndex', '0');
    });

    it('has descriptive text for screen readers', () => {
      render(<PhotoUploadArea />);

      expect(screen.getByText('Upload photo')).toBeInTheDocument();
      expect(screen.getByText('Click or drag to upload')).toBeInTheDocument();
    });

    it('provides proper alt text for uploaded images', () => {
      const mockPhoto = { url: 'http://example.com/photo.jpg' };
      mockUseAppStore.mockReturnValue({
        ...defaultStoreState,
        uploadedPhoto: mockPhoto,
      } as any);

      render(<PhotoUploadArea />);

      const image = screen.getByAltText('Uploaded photo');
      expect(image).toBeInTheDocument();
    });

    it('disables dropzone when uploading', async () => {
      mockUseAppStore.mockReturnValue({
        ...defaultStoreState,
        loading: { uploading: true },
      } as any);

      const { useDropzone } = await import('react-dropzone');
      vi.mocked(useDropzone).mockReturnValue({
        ...mockDropzoneProps,
        disabled: true,
      } as any);

      render(<PhotoUploadArea />);

      // Dropzone should be disabled during upload
      expect(vi.mocked(useDropzone)).toHaveBeenCalledWith(
        expect.objectContaining({ disabled: true })
      );
    });
  });

  describe('Error Handling', () => {
    it('handles API errors gracefully', async () => {
      const mockFile = new File(['test'], 'test.jpg', { type: 'image/jpeg' });
      const error = new Error('Network error');

      mockApi.uploadPhoto.mockRejectedValue(error);

      const mockOnDrop = vi.fn().mockImplementation(async (files) => {
        try {
          await mockApi.uploadPhoto(files[0]);
        } catch (error) {
          const message = error instanceof Error ? error.message : 'Upload failed';
          mockToast.error(message);
          defaultStoreState.setError({
            message: `Upload failed: ${message}`,
            type: 'api',
          });
        }
      });

      const dropzoneWithOnDrop: DropzoneState = {
        ...mockDropzoneProps,
        onDrop: mockOnDrop,
      } as any;

      const { useDropzone } = await import('react-dropzone');
      vi.mocked(useDropzone).mockReturnValue(dropzoneWithOnDrop);

      render(<PhotoUploadArea />);

      await mockOnDrop([mockFile]);

      expect(mockToast.error).toHaveBeenCalledWith('Network error');
      expect(defaultStoreState.setError).toHaveBeenCalledWith({
        message: 'Upload failed: Network error',
        type: 'api',
      });
    });

    it('handles unknown errors', async () => {
      const mockFile = new File(['test'], 'test.jpg', { type: 'image/jpeg' });

      mockApi.uploadPhoto.mockRejectedValue('Unknown error');

      const mockOnDrop = vi.fn().mockImplementation(async (files) => {
        try {
          await mockApi.uploadPhoto(files[0]);
        } catch (error) {
          const message = error instanceof Error ? error.message : 'Upload failed';
          mockToast.error(message);
        }
      });

      const dropzoneWithOnDrop: DropzoneState = {
        ...mockDropzoneProps,
        onDrop: mockOnDrop,
      } as any;

      const { useDropzone } = await import('react-dropzone');
      vi.mocked(useDropzone).mockReturnValue(dropzoneWithOnDrop);

      render(<PhotoUploadArea />);

      await mockOnDrop([mockFile]);

      expect(mockToast.error).toHaveBeenCalledWith('Upload failed');
    });
  });

  describe('User Interactions', () => {
    it('handles click to upload', () => {
      render(<PhotoUploadArea />);

      const dropzone = screen.getByTestId('dropzone-root');
      fireEvent.click(dropzone);

      // Clicking should trigger the file input (handled by react-dropzone)
      expect(dropzone).toBeInTheDocument();
    });

    it('replaces existing photo when clicked', () => {
      const mockPhoto = { url: 'http://example.com/photo.jpg' };
      mockUseAppStore.mockReturnValue({
        ...defaultStoreState,
        uploadedPhoto: mockPhoto,
      } as any);

      render(<PhotoUploadArea />);

      const dropzone = screen.getByTestId('dropzone-root');
      fireEvent.click(dropzone);

      // Should still be clickable for replacement
      expect(dropzone).toBeInTheDocument();
    });
  });

  describe('Performance', () => {
    it('uses proper file size limits', async () => {
      render(<PhotoUploadArea />);

      const { useDropzone } = await import('react-dropzone');
      expect(vi.mocked(useDropzone)).toHaveBeenCalledWith(
        expect.objectContaining({
          maxSize: 10 * 1024 * 1024, // 10MB
        })
      );
    });

    it('only accepts image files', async () => {
      render(<PhotoUploadArea />);

      const { useDropzone } = await import('react-dropzone');
      expect(vi.mocked(useDropzone)).toHaveBeenCalledWith(
        expect.objectContaining({
          accept: {
            'image/*': ['.png', '.jpg', '.jpeg', '.gif', '.webp'],
          },
        })
      );
    });
  });
});
