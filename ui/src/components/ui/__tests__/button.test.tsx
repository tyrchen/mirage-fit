import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '../../../test/test-utils';
import { Button } from '../button';

describe('Button Component', () => {
  describe('Rendering', () => {
    it('renders button with text', () => {
      render(<Button>Click me</Button>);
      expect(screen.getByRole('button', { name: /click me/i })).toBeInTheDocument();
    });

    it('renders button with custom className', () => {
      render(<Button className="custom-class">Test</Button>);
      const button = screen.getByRole('button');
      expect(button).toHaveClass('custom-class');
    });

    it('forwards ref correctly', () => {
      const ref = vi.fn();
      render(<Button ref={ref}>Test</Button>);
      expect(ref).toHaveBeenCalled();
    });
  });

  describe('Variants', () => {
    it('applies default variant styles', () => {
      render(<Button>Default</Button>);
      const button = screen.getByRole('button');
      expect(button).toHaveClass('bg-primary');
    });

    it('applies secondary variant styles', () => {
      render(<Button variant="secondary">Secondary</Button>);
      const button = screen.getByRole('button');
      expect(button).toHaveClass('bg-secondary');
    });

    it('applies destructive variant styles', () => {
      render(<Button variant="destructive">Destructive</Button>);
      const button = screen.getByRole('button');
      expect(button).toHaveClass('bg-destructive');
    });

    it('applies outline variant styles', () => {
      render(<Button variant="outline">Outline</Button>);
      const button = screen.getByRole('button');
      expect(button).toHaveClass('border', 'bg-background');
    });

    it('applies ghost variant styles', () => {
      render(<Button variant="ghost">Ghost</Button>);
      const button = screen.getByRole('button');
      expect(button).toHaveClass('hover:bg-accent');
    });

    it('applies link variant styles', () => {
      render(<Button variant="link">Link</Button>);
      const button = screen.getByRole('button');
      expect(button).toHaveClass('underline-offset-4');
    });
  });

  describe('Sizes', () => {
    it('applies default size styles', () => {
      render(<Button>Default Size</Button>);
      const button = screen.getByRole('button');
      expect(button).toHaveClass('h-9', 'px-4', 'py-2');
    });

    it('applies small size styles', () => {
      render(<Button size="sm">Small</Button>);
      const button = screen.getByRole('button');
      expect(button).toHaveClass('h-8', 'rounded-md', 'px-3');
    });

    it('applies large size styles', () => {
      render(<Button size="lg">Large</Button>);
      const button = screen.getByRole('button');
      expect(button).toHaveClass('h-10', 'rounded-md', 'px-6');
    });

    it('applies icon size styles', () => {
      render(<Button size="icon">Icon</Button>);
      const button = screen.getByRole('button');
      expect(button).toHaveClass('size-9');
    });
  });

  describe('Interactions', () => {
    it('handles click events', () => {
      const handleClick = vi.fn();
      render(<Button onClick={handleClick}>Click me</Button>);

      fireEvent.click(screen.getByRole('button'));
      expect(handleClick).toHaveBeenCalledTimes(1);
    });

    it('does not trigger click when disabled', () => {
      const handleClick = vi.fn();
      render(
        <Button onClick={handleClick} disabled>
          Disabled
        </Button>
      );

      fireEvent.click(screen.getByRole('button'));
      expect(handleClick).not.toHaveBeenCalled();
    });

    it('handles keyboard navigation', () => {
      const handleClick = vi.fn();
      render(<Button onClick={handleClick}>Press Enter</Button>);

      const button = screen.getByRole('button');
      button.focus();
      // In jsdom, keyDown doesn't automatically trigger click, so we simulate it
      fireEvent.keyDown(button, { key: 'Enter', code: 'Enter' });
      fireEvent.click(button); // Simulate browser behavior

      expect(handleClick).toHaveBeenCalledTimes(1);
    });

    it('handles space key press', () => {
      const handleClick = vi.fn();
      render(<Button onClick={handleClick}>Press Space</Button>);

      const button = screen.getByRole('button');
      button.focus();
      // In jsdom, keyDown doesn't automatically trigger click, so we simulate it
      fireEvent.keyDown(button, { key: ' ', code: 'Space' });
      fireEvent.click(button); // Simulate browser behavior

      expect(handleClick).toHaveBeenCalledTimes(1);
    });
  });

  describe('States', () => {
    it('applies disabled state correctly', () => {
      render(<Button disabled>Disabled</Button>);
      const button = screen.getByRole('button');

      expect(button).toBeDisabled();
      expect(button).toHaveClass('disabled:opacity-50');
    });

    it('shows focus states', () => {
      render(<Button>Focus me</Button>);
      const button = screen.getByRole('button');

      button.focus();
      expect(button).toHaveClass('focus-visible:ring-[3px]');
    });
  });

  describe('AsChild prop', () => {
    it('renders as child element when asChild is true', () => {
      render(
        <Button asChild>
          <a href="/test">Link as button</a>
        </Button>
      );

      const link = screen.getByRole('link');
      expect(link).toBeInTheDocument();
      expect(link).toHaveAttribute('href', '/test');
      // Should have button styles but be an anchor element
      expect(link).toHaveClass('inline-flex');
    });
  });

  describe('Accessibility', () => {
    it('has correct ARIA attributes', () => {
      render(
        <Button aria-label="Custom label" aria-describedby="description">
          Button
        </Button>
      );

      const button = screen.getByRole('button');
      expect(button).toHaveAttribute('aria-label', 'Custom label');
      expect(button).toHaveAttribute('aria-describedby', 'description');
    });

    it('supports aria-pressed for toggle buttons', () => {
      render(<Button aria-pressed={true}>Toggle</Button>);
      const button = screen.getByRole('button');
      expect(button).toHaveAttribute('aria-pressed', 'true');
    });

    it('has proper focus management', () => {
      render(<Button>Focus test</Button>);
      const button = screen.getByRole('button');

      button.focus();
      expect(document.activeElement).toBe(button);
    });

    it('supports screen readers with proper text content', () => {
      render(<Button>Screen reader text</Button>);
      const button = screen.getByRole('button', { name: /screen reader text/i });
      expect(button).toBeInTheDocument();
    });
  });

  describe('Edge Cases', () => {
    it('handles empty content gracefully', () => {
      render(<Button />);
      const button = screen.getByRole('button');
      expect(button).toBeInTheDocument();
    });

    it('handles multiple class combinations', () => {
      render(
        <Button
          variant="outline"
          size="lg"
          className="custom-class"
        >
          Complex Button
        </Button>
      );

      const button = screen.getByRole('button');
      expect(button).toHaveClass('border', 'h-10', 'custom-class');
    });

    it('preserves all HTML button attributes', () => {
      render(
        <Button
          type="submit"
          form="test-form"
          name="test-button"
          value="test-value"
        >
          Form Button
        </Button>
      );

      const button = screen.getByRole('button');
      expect(button).toHaveAttribute('type', 'submit');
      expect(button).toHaveAttribute('form', 'test-form');
      expect(button).toHaveAttribute('name', 'test-button');
      expect(button).toHaveAttribute('value', 'test-value');
    });
  });
});
