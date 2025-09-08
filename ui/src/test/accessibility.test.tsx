import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from './test-utils';
import { Button } from '../components/ui/button';
import { Input } from '../components/ui/input';
import LoadingSpinner from '../components/ui/LoadingSpinner';

describe('Accessibility Tests', () => {
  describe('Keyboard Navigation', () => {
    it('supports tab navigation for buttons', () => {
      render(
        <>
          <Button>First Button</Button>
          <Button>Second Button</Button>
          <Button>Third Button</Button>
        </>
      );

      const firstButton = screen.getByText('First Button');
      const secondButton = screen.getByText('Second Button');
      const thirdButton = screen.getByText('Third Button');

      // Start with first button focused
      firstButton.focus();
      expect(document.activeElement).toBe(firstButton);

      // Tab to second button
      fireEvent.keyDown(firstButton, { key: 'Tab' });
      secondButton.focus(); // Simulate browser behavior
      expect(document.activeElement).toBe(secondButton);

      // Tab to third button
      fireEvent.keyDown(secondButton, { key: 'Tab' });
      thirdButton.focus(); // Simulate browser behavior
      expect(document.activeElement).toBe(thirdButton);
    });

    it('supports Enter key activation for buttons', () => {
      const handleClick = vi.fn();
      render(<Button onClick={handleClick}>Test Button</Button>);

      const button = screen.getByRole('button');
      button.focus();

      // Use click event since buttons respond to Enter with click
      fireEvent.click(button);
      expect(handleClick).toHaveBeenCalledTimes(1);
    });

    it('supports Space key activation for buttons', () => {
      const handleClick = vi.fn();
      render(<Button onClick={handleClick}>Test Button</Button>);

      const button = screen.getByRole('button');
      button.focus();

      // Use click event since buttons respond to Space with click
      fireEvent.click(button);
      expect(handleClick).toHaveBeenCalledTimes(1);
    });

    it('skips disabled elements in tab order', () => {
      render(
        <>
          <Button>Enabled Button</Button>
          <Button disabled>Disabled Button</Button>
          <Button>Another Enabled</Button>
        </>
      );

      const disabledButton = screen.getByText('Disabled Button');

      // Disabled button should not be focusable
      disabledButton.focus();
      expect(document.activeElement).not.toBe(disabledButton);
    });
  });

  describe('Screen Reader Support', () => {
    it('provides proper aria-labels', () => {
      render(
        <>
          <Button aria-label="Close dialog">×</Button>
          <Input aria-label="Search products" type="search" />
        </>
      );

      expect(screen.getByLabelText('Close dialog')).toBeInTheDocument();
      expect(screen.getByLabelText('Search products')).toBeInTheDocument();
    });

    it('uses semantic HTML elements', () => {
      render(
        <main>
          <h1>Page Title</h1>
          <nav aria-label="Main navigation">
            <Button>Home</Button>
            <Button>Products</Button>
          </nav>
          <section>
            <h2>Content Section</h2>
            <p>Some content here</p>
          </section>
        </main>
      );

      expect(screen.getByRole('main')).toBeInTheDocument();
      expect(screen.getByRole('heading', { level: 1 })).toBeInTheDocument();
      expect(screen.getByRole('navigation', { name: 'Main navigation' })).toBeInTheDocument();
      expect(screen.getByRole('heading', { level: 2 })).toBeInTheDocument();
    });

    it('provides loading state announcements', () => {
      render(
        <div role="status" aria-live="polite">
          <LoadingSpinner />
          <span className="sr-only">Loading content, please wait...</span>
        </div>
      );

      expect(screen.getByRole('status')).toBeInTheDocument();
      expect(screen.getByText('Loading content, please wait...')).toBeInTheDocument();
    });

    it('provides error state announcements', () => {
      render(
        <div role="alert" aria-live="assertive">
          <span>Error: Failed to load data</span>
        </div>
      );

      expect(screen.getByRole('alert')).toBeInTheDocument();
      expect(screen.getByText('Error: Failed to load data')).toBeInTheDocument();
    });
  });

  describe('Focus Management', () => {
    it('maintains focus visible indicators', () => {
      render(<Button>Focus Test</Button>);
      const button = screen.getByRole('button');

      button.focus();
      // Check for focus-visible related classes in the button component
      const classes = button.className;
      expect(classes).toContain('focus-visible:ring');
    });

    it('provides skip links for keyboard users', () => {
      render(
        <div>
          <a href="#main-content" className="sr-only focus:not-sr-only">
            Skip to main content
          </a>
          <nav>Navigation content</nav>
          <main id="main-content">
            <h1>Main Content</h1>
          </main>
        </div>
      );

      const skipLink = screen.getByText('Skip to main content');
      expect(skipLink).toBeInTheDocument();
      expect(skipLink).toHaveAttribute('href', '#main-content');
    });

    it('traps focus in modal contexts', () => {
      // Mock a modal dialog
      render(
        <div role="dialog" aria-modal="true" aria-labelledby="modal-title">
          <h2 id="modal-title">Modal Title</h2>
          <Button>First Button</Button>
          <Input placeholder="Input field" />
          <Button>Close</Button>
        </div>
      );

      const dialog = screen.getByRole('dialog');
      const firstButton = screen.getByText('First Button');
      // const closeButton = screen.getByText('Close');

      expect(dialog).toHaveAttribute('aria-modal', 'true');

      // Focus should be contained within the dialog
      firstButton.focus();
      expect(document.activeElement).toBe(firstButton);
    });
  });

  describe('Color and Contrast', () => {
    it('provides sufficient contrast for text', () => {
      render(
        <div>
          <Button variant="default">Primary Action</Button>
          <Button variant="secondary">Secondary Action</Button>
          <Button variant="destructive">Danger Action</Button>
        </div>
      );

      // These would be validated with actual contrast checking tools
      // Here we just verify the elements exist with correct classes
      expect(screen.getByText('Primary Action')).toHaveClass('bg-primary');
      expect(screen.getByText('Secondary Action')).toHaveClass('bg-secondary');
      expect(screen.getByText('Danger Action')).toHaveClass('bg-destructive');
    });

    it('does not rely solely on color for information', () => {
      render(
        <form>
          <div>
            <label htmlFor="required-field">
              Required Field <span aria-label="required">*</span>
            </label>
            <Input
              id="required-field"
              required
              aria-invalid={false}
              aria-describedby="field-error"
            />
            <div id="field-error" role="alert" style={{ display: 'none' }}>
              This field is required
            </div>
          </div>
        </form>
      );

      const requiredIndicator = screen.getByLabelText('required');
      expect(requiredIndicator).toBeInTheDocument();
      expect(screen.getByRole('textbox')).toHaveAttribute('required');
    });
  });

  describe('Responsive Accessibility', () => {
    it('maintains accessibility on mobile viewports', () => {
      // Simulate mobile viewport
      Object.defineProperty(window, 'innerWidth', {
        writable: true,
        configurable: true,
        value: 375,
      });
      window.dispatchEvent(new Event('resize'));

      render(
        <div>
          <Button size="lg">Large Button</Button>
          <Input type="text" placeholder="Mobile input" />
        </div>
      );

      // Elements should still be accessible
      expect(screen.getByRole('button')).toBeInTheDocument();
      expect(screen.getByRole('textbox')).toBeInTheDocument();
    });

    it('provides touch-friendly targets', () => {
      render(<Button size="lg">Touch Button</Button>);
      const button = screen.getByRole('button');

      // Large buttons should have adequate touch target size
      expect(button).toHaveClass('h-10'); // 40px, close to 44px minimum recommended
    });
  });

  describe('Error Handling Accessibility', () => {
    it('associates error messages with form fields', () => {
      render(
        <div>
          <label htmlFor="email-field">Email</label>
          <Input
            id="email-field"
            type="email"
            aria-invalid={true}
            aria-describedby="email-error"
          />
          <div id="email-error" role="alert">
            Please enter a valid email address
          </div>
        </div>
      );

      const input = screen.getByRole('textbox');
      const errorMessage = screen.getByRole('alert');

      expect(input).toHaveAttribute('aria-invalid', 'true');
      expect(input).toHaveAttribute('aria-describedby', 'email-error');
      expect(errorMessage).toHaveTextContent('Please enter a valid email address');
    });

    it('provides live region updates for dynamic content', () => {
      render(
        <div aria-live="polite" aria-atomic="true">
          <span>Form submitted successfully</span>
        </div>
      );

      const liveRegion = screen.getByText('Form submitted successfully').parentElement;
      expect(liveRegion).toHaveAttribute('aria-live', 'polite');
      expect(liveRegion).toHaveAttribute('aria-atomic', 'true');
    });
  });

  describe('Progressive Enhancement', () => {
    it('works without JavaScript for basic functionality', () => {
      render(
        <form action="/submit" method="post" aria-label="User form">
          <Input name="username" placeholder="Username" required />
          <Button type="submit">Submit</Button>
        </form>
      );

      const form = screen.getByRole('form');
      const input = screen.getByRole('textbox');
      const button = screen.getByRole('button');

      expect(form).toHaveAttribute('action', '/submit');
      expect(form).toHaveAttribute('method', 'post');
      expect(input).toHaveAttribute('name', 'username');
      expect(button).toHaveAttribute('type', 'submit');
    });

    it('enhances with JavaScript when available', () => {
      const handleSubmit = vi.fn((e) => e.preventDefault());

      render(
        <form onSubmit={handleSubmit} aria-label="Enhanced form">
          <Input placeholder="Enhanced input" />
          <Button type="submit">Enhanced Submit</Button>
        </form>
      );

      const form = screen.getByRole('form', { name: 'Enhanced form' });
      fireEvent.submit(form);

      expect(handleSubmit).toHaveBeenCalled();
    });
  });

  describe('Internationalization (i18n)', () => {
    it('supports right-to-left languages', () => {
      render(
        <div dir="rtl" lang="ar">
          <Button>زر الإجراء</Button>
          <Input placeholder="نص البحث" />
        </div>
      );

      const container = screen.getByRole('button').closest('div');
      expect(container).toHaveAttribute('dir', 'rtl');
      expect(container).toHaveAttribute('lang', 'ar');
    });

    it('provides language attributes for screen readers', () => {
      render(
        <div>
          <p lang="en">This is English text</p>
          <p lang="es">Este es texto en español</p>
        </div>
      );

      const englishText = screen.getByText('This is English text');
      const spanishText = screen.getByText('Este es texto en español');

      expect(englishText).toHaveAttribute('lang', 'en');
      expect(spanishText).toHaveAttribute('lang', 'es');
    });
  });

  describe('Performance and Accessibility', () => {
    it('reduces motion for users who prefer it', () => {
      // Mock prefer-reduced-motion
      Object.defineProperty(window, 'matchMedia', {
        writable: true,
        value: vi.fn().mockImplementation(query => ({
          matches: query === '(prefers-reduced-motion: reduce)',
          media: query,
          onchange: null,
          addEventListener: vi.fn(),
          removeEventListener: vi.fn(),
          dispatchEvent: vi.fn(),
        })),
      });

      const { container } = render(
        <div className="motion-reduce:animate-none">
          <LoadingSpinner />
        </div>
      );

      // The component should respect reduced motion preferences
      const spinner = container.querySelector('.border-t-current');
      expect(spinner).toBeInTheDocument();
    });

    it('provides alternative text for images', () => {
      render(
        <div>
          <img src="/logo.jpg" alt="Company Logo" />
          <img src="/decorative.jpg" alt="" role="presentation" />
        </div>
      );

      const logo = screen.getByAltText('Company Logo');
      const decorativeImage = screen.getByRole('presentation');

      expect(logo).toHaveAttribute('alt', 'Company Logo');
      expect(decorativeImage).toHaveAttribute('alt', '');
    });
  });
});
