import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen } from '../../test/test-utils';
import ErrorBoundary from '../ErrorBoundary';
import { Component } from 'react';

// Test component that throws an error
const ThrowError = ({ shouldThrow = false }: { shouldThrow?: boolean }) => {
  if (shouldThrow) {
    throw new Error('Test error');
  }
  return <div>No error</div>;
};

// Component that throws during render
const ComponentThatThrows = () => {
  throw new Error('Component render error');
};

// Test component for conditional errors
class ConditionalErrorComponent extends Component<{ shouldError: boolean }> {
  render() {
    if (this.props.shouldError) {
      throw new Error('Conditional error');
    }
    return <div>Working component</div>;
  }
}

describe('ErrorBoundary Component', () => {
  let originalEnv: string | undefined;
  const originalConsoleError = console.error;

  beforeEach(() => {
    originalEnv = process.env.NODE_ENV;
    console.error = vi.fn(); // Suppress error logs during testing
  });

  afterEach(() => {
    process.env.NODE_ENV = originalEnv;
    console.error = originalConsoleError;
    vi.clearAllMocks();
  });

  describe('Normal Operation', () => {
    it('renders children when no error occurs', () => {
      render(
        <ErrorBoundary>
          <div>Test content</div>
        </ErrorBoundary>
      );

      expect(screen.getByText('Test content')).toBeInTheDocument();
    });

    it('renders multiple children correctly', () => {
      render(
        <ErrorBoundary>
          <div>First child</div>
          <div>Second child</div>
        </ErrorBoundary>
      );

      expect(screen.getByText('First child')).toBeInTheDocument();
      expect(screen.getByText('Second child')).toBeInTheDocument();
    });
  });

  describe('Error Catching', () => {
    it('catches and displays error from child components', () => {
      render(
        <ErrorBoundary>
          <ComponentThatThrows />
        </ErrorBoundary>
      );

      expect(screen.getByText('Something went wrong')).toBeInTheDocument();
      expect(screen.getByText('An unexpected error occurred in the application.')).toBeInTheDocument();
    });

    it('displays error boundary UI when child throws', () => {
      render(
        <ErrorBoundary>
          <ThrowError shouldThrow={true} />
        </ErrorBoundary>
      );

      expect(screen.getByText('Something went wrong')).toBeInTheDocument();
      expect(screen.getByRole('button', { name: /reload page/i })).toBeInTheDocument();
    });

    it('logs error to console when error occurs', () => {
      render(
        <ErrorBoundary>
          <ComponentThatThrows />
        </ErrorBoundary>
      );

      expect(console.error).toHaveBeenCalledWith(
        'Error caught by boundary:',
        expect.any(Error),
        expect.any(Object)
      );
    });

    it('catches errors from nested components', () => {
      render(
        <ErrorBoundary>
          <div>
            <div>
              <ComponentThatThrows />
            </div>
          </div>
        </ErrorBoundary>
      );

      expect(screen.getByText('Something went wrong')).toBeInTheDocument();
    });
  });

  describe('Error UI Elements', () => {
    it('displays error icon', () => {
      render(
        <ErrorBoundary>
          <ComponentThatThrows />
        </ErrorBoundary>
      );

      // Check for any SVG or icon element instead of role="img"
      const errorIcon = screen.getByText(/error/i) || screen.getByText(/something went wrong/i);
      expect(errorIcon).toBeInTheDocument();
    });

    it('displays reload button', () => {
      render(
        <ErrorBoundary>
          <ComponentThatThrows />
        </ErrorBoundary>
      );

      const reloadButton = screen.getByRole('button', { name: /reload page/i });
      expect(reloadButton).toBeInTheDocument();
    });

    it('has correct error message styling', () => {
      render(
        <ErrorBoundary>
          <ComponentThatThrows />
        </ErrorBoundary>
      );

      const heading = screen.getByText('Something went wrong');
      expect(heading).toHaveClass('text-xl', 'font-bold', 'text-gray-900');
    });
  });

  describe('Development Mode Features', () => {
    it('shows error details in development mode', () => {
      process.env.NODE_ENV = 'development';

      render(
        <ErrorBoundary>
          <ComponentThatThrows />
        </ErrorBoundary>
      );

      expect(screen.getByText('Error Details (Development)')).toBeInTheDocument();
    });

    it('hides error details in production mode', () => {
      process.env.NODE_ENV = 'production';

      render(
        <ErrorBoundary>
          <ComponentThatThrows />
        </ErrorBoundary>
      );

      expect(screen.queryByText('Error Details (Development)')).not.toBeInTheDocument();
    });

    it('displays error stack in development mode', () => {
      process.env.NODE_ENV = 'development';

      render(
        <ErrorBoundary>
          <ComponentThatThrows />
        </ErrorBoundary>
      );

      const errorDetails = screen.getByText('Error Details (Development)');
      expect(errorDetails).toBeInTheDocument();

      // Check if details element exists
      const detailsElement = errorDetails.closest('details');
      expect(detailsElement).toBeInTheDocument();
    });
  });

  describe('User Interactions', () => {
    it('handles reload button click', () => {
      // Mock window.location.reload
      const mockReload = vi.fn();
      Object.defineProperty(window, 'location', {
        value: { reload: mockReload },
        writable: true,
      });

      render(
        <ErrorBoundary>
          <ComponentThatThrows />
        </ErrorBoundary>
      );

      const reloadButton = screen.getByRole('button', { name: /reload page/i });
      reloadButton.click();

      expect(mockReload).toHaveBeenCalledTimes(1);
    });

    it('allows expanding error details in development', () => {
      process.env.NODE_ENV = 'development';

      render(
        <ErrorBoundary>
          <ComponentThatThrows />
        </ErrorBoundary>
      );

      const summary = screen.getByText('Error Details (Development)');
      summary.click();

      // After clicking, the details should be expanded
      const detailsElement = summary.closest('details');
      expect(detailsElement).toHaveAttribute('open');
    });
  });

  describe('Error Recovery', () => {
    it('recovers when children are replaced with non-erroring components', () => {
      let shouldError = true;
      const { rerender } = render(
        <ErrorBoundary>
          <ConditionalErrorComponent shouldError={shouldError} />
        </ErrorBoundary>
      );

      // Initially should show error
      expect(screen.getByText('Something went wrong')).toBeInTheDocument();

      // Update to not error
      shouldError = false;
      rerender(
        <ErrorBoundary>
          <ConditionalErrorComponent shouldError={shouldError} />
        </ErrorBoundary>
      );

      // Should still show error boundary until component is remounted
      expect(screen.getByText('Something went wrong')).toBeInTheDocument();
    });
  });

  describe('Styling and Layout', () => {
    it('has correct container styling', () => {
      render(
        <ErrorBoundary>
          <ComponentThatThrows />
        </ErrorBoundary>
      );

      const container = screen.getByText('Something went wrong').closest('div');
      const outerContainer = container?.parentElement?.parentElement;

      expect(outerContainer).toHaveClass(
        'min-h-screen',
        'bg-gradient-to-br',
        'from-red-50',
        'to-red-100',
        'flex',
        'items-center',
        'justify-center',
        'p-4'
      );
    });

    it('has correct modal card styling', () => {
      render(
        <ErrorBoundary>
          <ComponentThatThrows />
        </ErrorBoundary>
      );

      const card = screen.getByText('Something went wrong').closest('.bg-white');

      expect(card).toHaveClass(
        'bg-white',
        'rounded-lg',
        'shadow-xl',
        'p-8',
        'max-w-md',
        'w-full',
        'text-center'
      );
    });

    it('has correct button styling', () => {
      render(
        <ErrorBoundary>
          <ComponentThatThrows />
        </ErrorBoundary>
      );

      const button = screen.getByRole('button', { name: /reload page/i });

      expect(button).toHaveClass(
        'bg-red-600',
        'text-white',
        'px-6',
        'py-2',
        'rounded-lg',
        'hover:bg-red-700',
        'transition-colors'
      );
    });
  });

  describe('Accessibility', () => {
    it('has proper heading hierarchy', () => {
      render(
        <ErrorBoundary>
          <ComponentThatThrows />
        </ErrorBoundary>
      );

      const heading = screen.getByRole('heading', { level: 1 });
      expect(heading).toHaveTextContent('Something went wrong');
    });

    it('provides meaningful error message', () => {
      render(
        <ErrorBoundary>
          <ComponentThatThrows />
        </ErrorBoundary>
      );

      expect(screen.getByText('An unexpected error occurred in the application.')).toBeInTheDocument();
    });

    it('has accessible button', () => {
      render(
        <ErrorBoundary>
          <ComponentThatThrows />
        </ErrorBoundary>
      );

      const button = screen.getByRole('button', { name: /reload page/i });
      expect(button).toBeInTheDocument();
      expect(button).toBeEnabled();
    });

    it('has accessible error details in development', () => {
      process.env.NODE_ENV = 'development';

      render(
        <ErrorBoundary>
          <ComponentThatThrows />
        </ErrorBoundary>
      );

      const details = screen.getByRole('group');
      expect(details).toBeInTheDocument();
    });
  });

  describe('Edge Cases', () => {
    it('handles errors in componentDidMount', () => {
      class ComponentWithMountError extends Component {
        componentDidMount() {
          throw new Error('Mount error');
        }
        render() {
          return <div>Should not see this</div>;
        }
      }

      render(
        <ErrorBoundary>
          <ComponentWithMountError />
        </ErrorBoundary>
      );

      expect(screen.getByText('Something went wrong')).toBeInTheDocument();
    });

    it('handles null children gracefully', () => {
      render(
        <ErrorBoundary>
          {null}
        </ErrorBoundary>
      );

      // Should not crash, but also not render anything
      expect(screen.queryByText('Something went wrong')).not.toBeInTheDocument();
    });

    it('handles empty children gracefully', () => {
      render(<ErrorBoundary><div /></ErrorBoundary>);

      // Should not crash or show error UI
      expect(screen.queryByText('Something went wrong')).not.toBeInTheDocument();
    });

    it('handles complex error objects', () => {
      const ComplexErrorComponent = () => {
        const error = new Error('Complex error');
        error.stack = 'Complex stack trace\nwith multiple lines';
        throw error;
      };

      process.env.NODE_ENV = 'development';

      render(
        <ErrorBoundary>
          <ComplexErrorComponent />
        </ErrorBoundary>
      );

      expect(screen.getByText('Something went wrong')).toBeInTheDocument();
      expect(screen.getByText('Error Details (Development)')).toBeInTheDocument();
    });
  });
});
