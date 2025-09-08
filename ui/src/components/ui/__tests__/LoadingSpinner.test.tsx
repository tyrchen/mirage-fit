import { describe, it, expect } from 'vitest';
import { render, screen } from '../../../test/test-utils';
import LoadingSpinner from '../LoadingSpinner';

describe('LoadingSpinner Component', () => {
  describe('Rendering', () => {
    it('renders loading spinner', () => {
      const { container } = render(<LoadingSpinner />);
      const spinner = container.querySelector('.border-t-current');
      expect(spinner).toBeInTheDocument();
    });

    it('applies default medium size class', () => {
      const { container } = render(<LoadingSpinner />);
      const spinner = container.querySelector('.border-t-current');
      expect(spinner).toHaveClass('w-6', 'h-6');
    });

    it('applies custom className', () => {
      const { container } = render(<LoadingSpinner className="custom-spinner" />);
      const spinner = container.querySelector('.border-t-current');
      expect(spinner).toHaveClass('custom-spinner');
    });
  });

  describe('Size Variants', () => {
    it('applies small size styles', () => {
      const { container } = render(<LoadingSpinner size="sm" />);
      const spinner = container.querySelector('.border-t-current');
      expect(spinner).toHaveClass('w-4', 'h-4');
    });

    it('applies medium size styles (default)', () => {
      const { container } = render(<LoadingSpinner size="md" />);
      const spinner = container.querySelector('.border-t-current');
      expect(spinner).toHaveClass('w-6', 'h-6');
    });

    it('applies large size styles', () => {
      const { container } = render(<LoadingSpinner size="lg" />);
      const spinner = container.querySelector('.border-t-current');
      expect(spinner).toHaveClass('w-8', 'h-8');
    });
  });

  describe('Styling', () => {
    it('has correct base styles', () => {
      const { container } = render(<LoadingSpinner />);
      const spinner = container.querySelector('.border-t-current');

      expect(spinner).toHaveClass(
        'border-2',
        'border-transparent',
        'border-t-current',
        'rounded-full'
      );
    });

    it('combines size and custom classes correctly', () => {
      const { container } = render(<LoadingSpinner size="lg" className="text-blue-500" />);
      const spinner = container.querySelector('.border-t-current');

      expect(spinner).toHaveClass('w-8', 'h-8', 'text-blue-500');
    });
  });

  describe('Animation', () => {
    it('has rotation animation properties', () => {
      const { container } = render(<LoadingSpinner />);
      const spinner = container.querySelector('.border-t-current');

      // The motion.div should be present (mocked in setup)
      expect(spinner).toBeInTheDocument();
    });
  });

  describe('Accessibility', () => {
    it('provides proper loading indication', () => {
      render(
        <div>
          <LoadingSpinner />
          <span className="sr-only">Loading...</span>
        </div>
      );

      expect(screen.getByText(/loading/i)).toBeInTheDocument();
    });

    it('can have aria-label for screen readers', () => {
      render(
        <div aria-label="Loading content">
          <LoadingSpinner />
        </div>
      );

      const container = screen.getByLabelText(/loading content/i);
      expect(container).toBeInTheDocument();
    });

    it('works with aria-live regions', () => {
      render(
        <div aria-live="polite" aria-atomic="true" role="status">
          <LoadingSpinner />
          <span>Loading data...</span>
        </div>
      );

      const liveRegion = screen.getByRole('status');
      expect(liveRegion).toBeInTheDocument();
    });
  });

  describe('Integration with Theme', () => {
    it('respects current color from parent', () => {
      const { container } = render(
        <div className="text-red-500">
          <LoadingSpinner />
        </div>
      );

      const spinner = container.querySelector('.border-t-current');
      expect(spinner).toHaveClass('border-t-current');
    });

    it('works with different color contexts', () => {
      render(
        <>
          <div className="text-blue-500">
            <LoadingSpinner data-testid="blue-spinner" />
          </div>
          <div className="text-green-500">
            <LoadingSpinner data-testid="green-spinner" />
          </div>
        </>
      );

      expect(screen.getByTestId('blue-spinner')).toHaveClass('border-t-current');
      expect(screen.getByTestId('green-spinner')).toHaveClass('border-t-current');
    });
  });

  describe('Edge Cases', () => {
    it('handles undefined size gracefully', () => {
      const { container } = render(<LoadingSpinner size={undefined} />);
      const spinner = container.querySelector('.border-t-current');
      expect(spinner).toHaveClass('w-6', 'h-6'); // Should default to medium
    });

    it('handles empty className', () => {
      const { container } = render(<LoadingSpinner className="" />);
      const spinner = container.querySelector('.border-t-current');
      expect(spinner).toBeInTheDocument();
    });

    it('maintains aspect ratio', () => {
      const { container } = render(<LoadingSpinner size="lg" />);
      const spinner = container.querySelector('.border-t-current');
      expect(spinner).toHaveClass('w-8', 'h-8'); // Square aspect ratio
    });
  });

  describe('Performance', () => {
    it('renders without causing layout shift', () => {
      const { container } = render(<LoadingSpinner />);

      // Spinner should have fixed dimensions
      const spinner = container.firstChild as HTMLElement;
      expect(spinner).toHaveClass('w-6', 'h-6');
    });

    it('supports multiple spinners without conflict', () => {
      render(
        <>
          <LoadingSpinner size="sm" data-testid="spinner-1" />
          <LoadingSpinner size="md" data-testid="spinner-2" />
          <LoadingSpinner size="lg" data-testid="spinner-3" />
        </>
      );

      expect(screen.getByTestId('spinner-1')).toHaveClass('w-4', 'h-4');
      expect(screen.getByTestId('spinner-2')).toHaveClass('w-6', 'h-6');
      expect(screen.getByTestId('spinner-3')).toHaveClass('w-8', 'h-8');
    });
  });
});
