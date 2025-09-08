import React from 'react';
import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '../../../test/test-utils';
import { Input } from '../input';

describe('Input Component', () => {
  describe('Rendering', () => {
    it('renders input field', () => {
      render(<Input placeholder="Enter text" />);
      expect(screen.getByRole('textbox')).toBeInTheDocument();
    });

    it('renders with placeholder text', () => {
      render(<Input placeholder="Test placeholder" />);
      expect(screen.getByPlaceholderText('Test placeholder')).toBeInTheDocument();
    });

    it('applies custom className', () => {
      render(<Input className="custom-input" placeholder="test" />);
      const input = screen.getByRole('textbox');
      expect(input).toHaveClass('custom-input');
    });

    it('has correct data-slot attribute', () => {
      render(<Input placeholder="test" />);
      const input = screen.getByRole('textbox');
      expect(input).toHaveAttribute('data-slot', 'input');
    });
  });

  describe('Input Types', () => {
    it('renders as text input by default', () => {
      render(<Input />);
      const input = screen.getByRole('textbox');
      expect(input).toHaveAttribute('type', 'text');
    });

    it('renders as password input', () => {
      const { container } = render(<Input type="password" />);
      const input = container.querySelector('input[type="password"]');
      expect(input).toHaveAttribute('type', 'password');
    });

    it('renders as email input', () => {
      render(<Input type="email" placeholder="Email" />);
      const input = screen.getByRole('textbox');
      expect(input).toHaveAttribute('type', 'email');
    });

    it('renders as number input', () => {
      render(<Input type="number" placeholder="Number" />);
      const input = screen.getByRole('spinbutton');
      expect(input).toHaveAttribute('type', 'number');
    });

    it('renders as search input', () => {
      render(<Input type="search" placeholder="Search" />);
      const input = screen.getByRole('searchbox');
      expect(input).toHaveAttribute('type', 'search');
    });

    it('renders as file input', () => {
      const { container } = render(<Input type="file" />);
      const input = container.querySelector('input[type="file"]');
      expect(input).toHaveAttribute('type', 'file');
    });
  });

  describe('Styling', () => {
    it('has correct base styles', () => {
      render(<Input placeholder="test" />);
      const input = screen.getByRole('textbox');

      expect(input).toHaveClass(
        'flex',
        'h-9',
        'w-full',
        'rounded-md',
        'border',
        'bg-transparent',
        'px-3',
        'py-1',
        'text-base'
      );
    });

    it('has correct focus styles', () => {
      render(<Input placeholder="test" />);
      const input = screen.getByRole('textbox');

      expect(input).toHaveClass(
        'focus-visible:border-ring',
        'focus-visible:ring-ring/50',
        'focus-visible:ring-[3px]'
      );
    });

    it('has correct disabled styles', () => {
      render(<Input disabled placeholder="test" />);
      const input = screen.getByRole('textbox');

      expect(input).toHaveClass(
        'disabled:pointer-events-none',
        'disabled:cursor-not-allowed',
        'disabled:opacity-50'
      );
    });

    it('has correct invalid styles', () => {
      render(<Input aria-invalid={true} placeholder="test" />);
      const input = screen.getByRole('textbox');

      expect(input).toHaveClass(
        'aria-invalid:ring-destructive/20',
        'aria-invalid:border-destructive'
      );
    });
  });

  describe('User Interactions', () => {
    it('handles text input', async () => {
      const { user } = render(<Input placeholder="Type here" />);
      const input = screen.getByRole('textbox');

      await user.type(input, 'Hello World');
      expect(input).toHaveValue('Hello World');
    });

    it('handles onChange events', async () => {
      const handleChange = vi.fn();
      const { user } = render(<Input onChange={handleChange} placeholder="test" />);
      const input = screen.getByRole('textbox');

      await user.type(input, 'test');
      expect(handleChange).toHaveBeenCalled();
    });

    it('handles onFocus events', () => {
      const handleFocus = vi.fn();
      render(<Input onFocus={handleFocus} placeholder="test" />);
      const input = screen.getByRole('textbox');

      fireEvent.focus(input);
      expect(handleFocus).toHaveBeenCalledTimes(1);
    });

    it('handles onBlur events', () => {
      const handleBlur = vi.fn();
      render(<Input onBlur={handleBlur} placeholder="test" />);
      const input = screen.getByRole('textbox');

      fireEvent.focus(input);
      fireEvent.blur(input);
      expect(handleBlur).toHaveBeenCalledTimes(1);
    });

    it('handles keyboard events', () => {
      const handleKeyDown = vi.fn();
      render(<Input onKeyDown={handleKeyDown} placeholder="test" />);
      const input = screen.getByRole('textbox');

      fireEvent.keyDown(input, { key: 'Enter', code: 'Enter' });
      expect(handleKeyDown).toHaveBeenCalledWith(
        expect.objectContaining({ key: 'Enter' })
      );
    });
  });

  describe('States', () => {
    it('shows disabled state correctly', () => {
      render(<Input disabled placeholder="test" />);
      const input = screen.getByRole('textbox');

      expect(input).toBeDisabled();
    });

    it('shows readonly state correctly', () => {
      render(<Input readOnly value="readonly text" />);
      const input = screen.getByRole('textbox');

      expect(input).toHaveAttribute('readonly');
      expect(input).toHaveValue('readonly text');
    });

    it('shows required state correctly', () => {
      render(<Input required placeholder="test" />);
      const input = screen.getByRole('textbox');

      expect(input).toBeRequired();
    });

    it('handles controlled input', async () => {
      const TestComponent = () => {
        const [value, setValue] = React.useState('');
        return (
          <Input
            value={value}
            onChange={(e) => setValue(e.target.value)}
            placeholder="controlled"
          />
        );
      };

      const { user } = render(<TestComponent />);
      const input = screen.getByRole('textbox');

      await user.type(input, 'controlled value');
      expect(input).toHaveValue('controlled value');
    });

    it('handles uncontrolled input with defaultValue', () => {
      render(<Input defaultValue="default text" />);
      const input = screen.getByRole('textbox');

      expect(input).toHaveValue('default text');
    });
  });

  describe('Accessibility', () => {
    it('supports aria-label', () => {
      render(<Input aria-label="Username input" />);
      const input = screen.getByLabelText('Username input');
      expect(input).toBeInTheDocument();
    });

    it('supports aria-describedby', () => {
      render(
        <>
          <Input aria-describedby="help-text" placeholder="test" />
          <div id="help-text">Enter your username</div>
        </>
      );

      const input = screen.getByRole('textbox');
      expect(input).toHaveAttribute('aria-describedby', 'help-text');
    });

    it('supports aria-invalid', () => {
      render(<Input aria-invalid={true} placeholder="test" />);
      const input = screen.getByRole('textbox');
      expect(input).toHaveAttribute('aria-invalid', 'true');
    });

    it('works with labels', () => {
      render(
        <>
          <label htmlFor="test-input">Test Label</label>
          <Input id="test-input" />
        </>
      );

      const input = screen.getByLabelText('Test Label');
      expect(input).toBeInTheDocument();
    });

    it('has proper focus management', () => {
      render(<Input placeholder="test" />);
      const input = screen.getByRole('textbox');

      input.focus();
      expect(document.activeElement).toBe(input);
    });
  });

  describe('File Input Specific', () => {
    it('handles file selection', () => {
      const handleChange = vi.fn();
      const { container } = render(<Input type="file" onChange={handleChange} />);
      const input = container.querySelector('input[type="file"]');

      const file = new File(['content'], 'test.txt', { type: 'text/plain' });
      if (input) {
        fireEvent.change(input, { target: { files: [file] } });
      }

      expect(handleChange).toHaveBeenCalled();
    });

    it('supports multiple file selection', () => {
      const { container } = render(<Input type="file" multiple />);
      const input = container.querySelector('input[type="file"]');

      expect(input).toHaveAttribute('multiple');
    });

    it('supports file type restrictions', () => {
      const { container } = render(<Input type="file" accept=".jpg,.png" />);
      const input = container.querySelector('input[type="file"]');

      expect(input).toHaveAttribute('accept', '.jpg,.png');
    });
  });

  describe('Edge Cases', () => {
    it('handles empty value', () => {
      render(<Input value="" onChange={() => {}} />);
      const input = screen.getByRole('textbox');
      expect(input).toHaveValue('');
    });

    it('handles undefined value gracefully', () => {
      render(<Input value={undefined} placeholder="test" />);
      const input = screen.getByRole('textbox');
      expect(input).toBeInTheDocument();
    });

    it('preserves all HTML input attributes', () => {
      render(
        <Input
          name="test-input"
          id="test-id"
          autoComplete="username"
          maxLength={50}
          minLength={2}
          pattern="[A-Za-z]*"
          placeholder="test"
        />
      );

      const input = screen.getByRole('textbox');
      expect(input).toHaveAttribute('name', 'test-input');
      expect(input).toHaveAttribute('id', 'test-id');
      expect(input).toHaveAttribute('autocomplete', 'username');
      expect(input).toHaveAttribute('maxlength', '50');
      expect(input).toHaveAttribute('minlength', '2');
      expect(input).toHaveAttribute('pattern', '[A-Za-z]*');
    });

    it('handles form integration', () => {
      render(
        <form>
          <Input name="username" placeholder="Username" />
        </form>
      );

      const input = screen.getByRole('textbox');
      expect(input.closest('form')).toBeInTheDocument();
    });
  });
});
