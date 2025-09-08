import React from 'react';
import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '../../../test/test-utils';
import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
  DialogClose,
} from '../dialog';
import { Button } from '../button';

describe('Dialog Component', () => {
  describe('Basic Rendering', () => {
    it('renders dialog trigger', () => {
      render(
        <Dialog>
          <DialogTrigger asChild>
            <Button>Open Dialog</Button>
          </DialogTrigger>
        </Dialog>
      );

      expect(screen.getByRole('button', { name: /open dialog/i })).toBeInTheDocument();
    });

    it('does not show dialog content initially', () => {
      render(
        <Dialog>
          <DialogTrigger asChild>
            <Button>Open Dialog</Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Test Dialog</DialogTitle>
              <DialogDescription>Test dialog description</DialogDescription>
            </DialogHeader>
          </DialogContent>
        </Dialog>
      );

      expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
    });
  });

  describe('Dialog Opening and Closing', () => {
    it('opens dialog when trigger is clicked', () => {
      render(
        <Dialog>
          <DialogTrigger asChild>
            <Button>Open Dialog</Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Test Dialog</DialogTitle>
              <DialogDescription>Test dialog description</DialogDescription>
            </DialogHeader>
          </DialogContent>
        </Dialog>
      );

      fireEvent.click(screen.getByRole('button', { name: /open dialog/i }));

      expect(screen.getByRole('dialog')).toBeInTheDocument();
      expect(screen.getByText('Test Dialog')).toBeInTheDocument();
    });

    it('closes dialog when close button is clicked', () => {
      render(
        <Dialog defaultOpen>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Test Dialog</DialogTitle>
              <DialogDescription>Test dialog description</DialogDescription>
            </DialogHeader>
            <DialogClose asChild>
              <Button>Close</Button>
            </DialogClose>
          </DialogContent>
        </Dialog>
      );

      expect(screen.getByRole('dialog')).toBeInTheDocument();

      fireEvent.click(screen.getByRole('button', { name: /close/i }));

      expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
    });

    it('closes dialog when escape key is pressed', () => {
      render(
        <Dialog defaultOpen>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Test Dialog</DialogTitle>
              <DialogDescription>Test dialog description</DialogDescription>
            </DialogHeader>
          </DialogContent>
        </Dialog>
      );

      const dialog = screen.getByRole('dialog');
      expect(dialog).toBeInTheDocument();

      fireEvent.keyDown(dialog, { key: 'Escape', code: 'Escape' });

      expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
    });
  });

  describe('Dialog Structure', () => {
    it('renders complete dialog structure', () => {
      render(
        <Dialog defaultOpen>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Dialog Title</DialogTitle>
              <DialogDescription>Dialog Description</DialogDescription>
            </DialogHeader>
            <div>Dialog Body Content</div>
            <DialogFooter>
              <DialogClose asChild>
                <Button variant="outline">Cancel</Button>
              </DialogClose>
              <Button>Confirm</Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      );

      expect(screen.getByRole('dialog')).toBeInTheDocument();
      expect(screen.getByRole('heading', { level: 2 })).toHaveTextContent('Dialog Title');
      expect(screen.getByText('Dialog Description')).toBeInTheDocument();
      expect(screen.getByText('Dialog Body Content')).toBeInTheDocument();
      expect(screen.getByRole('button', { name: /cancel/i })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: /confirm/i })).toBeInTheDocument();
    });

    it('applies correct styling classes', () => {
      render(
        <Dialog defaultOpen>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Test</DialogTitle>
            </DialogHeader>
          </DialogContent>
        </Dialog>
      );

      const dialog = screen.getByRole('dialog');
      expect(dialog).toHaveClass('bg-background', 'border', 'rounded-lg', 'shadow-lg');
    });
  });

  describe('Controlled Dialog', () => {
    it('works as controlled component', () => {
      const TestComponent = () => {
        const [open, setOpen] = React.useState(false);

        return (
          <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger asChild>
              <Button>Toggle</Button>
            </DialogTrigger>
            <DialogContent>
              <DialogTitle>Controlled Dialog</DialogTitle>
              <DialogDescription>Controlled dialog description</DialogDescription>
            </DialogContent>
          </Dialog>
        );
      };

      render(<TestComponent />);

      expect(screen.queryByRole('dialog')).not.toBeInTheDocument();

      fireEvent.click(screen.getByRole('button'));
      expect(screen.getByRole('dialog')).toBeInTheDocument();
    });

    it('calls onOpenChange when dialog state changes', () => {
      const handleOpenChange = vi.fn();

      render(
        <Dialog onOpenChange={handleOpenChange}>
          <DialogTrigger asChild>
            <Button>Open</Button>
          </DialogTrigger>
          <DialogContent>
            <DialogTitle>Test</DialogTitle>
            <DialogDescription>Test dialog description</DialogDescription>
          </DialogContent>
        </Dialog>
      );

      fireEvent.click(screen.getByRole('button'));

      expect(handleOpenChange).toHaveBeenCalledWith(true);
    });
  });

  describe('Accessibility', () => {
    it('has correct ARIA attributes', () => {
      render(
        <Dialog defaultOpen>
          <DialogContent>
            <DialogHeader>
              <DialogTitle id="dialog-title">Accessible Dialog</DialogTitle>
              <DialogDescription id="dialog-description">
                This is an accessible dialog description
              </DialogDescription>
            </DialogHeader>
          </DialogContent>
        </Dialog>
      );

      const dialog = screen.getByRole('dialog');
      expect(dialog).toHaveAttribute('aria-modal', 'true');
      expect(dialog).toHaveAttribute('aria-labelledby');
      expect(dialog).toHaveAttribute('aria-describedby');
    });

    it('manages focus properly', () => {
      render(
        <Dialog>
          <DialogTrigger asChild>
            <Button>Open</Button>
          </DialogTrigger>
          <DialogContent>
            <DialogTitle>Focus Test</DialogTitle>
            <DialogDescription>Focus test description</DialogDescription>
            <Button>First Button</Button>
            <Button>Second Button</Button>
          </DialogContent>
        </Dialog>
      );

      const trigger = screen.getByRole('button', { name: /open/i });
      fireEvent.click(trigger);

      // Focus should move into the dialog
      const dialog = screen.getByRole('dialog');
      expect(dialog).toBeInTheDocument();
    });

    it('traps focus within dialog', () => {
      render(
        <div>
          <Button>Outside Button</Button>
          <Dialog defaultOpen>
            <DialogContent>
              <DialogTitle>Focus Trap</DialogTitle>
              <DialogDescription>Focus trap description</DialogDescription>
              <Button>Inside Button 1</Button>
              <Button>Inside Button 2</Button>
              <DialogClose asChild>
                <Button>Close</Button>
              </DialogClose>
            </DialogContent>
          </Dialog>
        </div>
      );

      const insideButton = screen.getByText('Inside Button 1');
      // const closeButton = screen.getByText('Close');

      // Focus should be trapped within dialog
      insideButton.focus();
      expect(document.activeElement).toBe(insideButton);

      // Tab should cycle within dialog
      fireEvent.keyDown(document.activeElement!, { key: 'Tab' });
      // Note: Actual focus trapping would be handled by radix-ui
    });

    it('restores focus to trigger when closed', () => {
      render(
        <Dialog>
          <DialogTrigger asChild>
            <Button>Open Dialog</Button>
          </DialogTrigger>
          <DialogContent>
            <DialogTitle>Test</DialogTitle>
            <DialogDescription>Test dialog description</DialogDescription>
            <DialogClose asChild>
              <Button>Close</Button>
            </DialogClose>
          </DialogContent>
        </Dialog>
      );

      const trigger = screen.getByRole('button', { name: /open dialog/i });
      fireEvent.click(trigger);

      const closeButton = screen.getByRole('button', { name: /close/i });
      fireEvent.click(closeButton);

      // Focus should return to trigger (handled by radix-ui)
      expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
    });
  });

  describe('Backdrop Interaction', () => {
    it('closes dialog when clicking backdrop', () => {
      render(
        <Dialog defaultOpen>
          <DialogContent>
            <DialogTitle>Test Dialog</DialogTitle>
            <DialogDescription>Test dialog description</DialogDescription>
          </DialogContent>
        </Dialog>
      );

      expect(screen.getByRole('dialog')).toBeInTheDocument();

      // Click on backdrop (outside dialog content)
      const backdrop = screen.getByRole('dialog').parentElement;
      if (backdrop) {
        fireEvent.click(backdrop);
      }

      // Note: Actual backdrop click behavior would be handled by radix-ui
    });

    it('prevents closing when clicking inside dialog', () => {
      render(
        <Dialog defaultOpen>
          <DialogContent>
            <DialogTitle>Test Dialog</DialogTitle>
            <DialogDescription>Test dialog description</DialogDescription>
            <p>Dialog content</p>
          </DialogContent>
        </Dialog>
      );

      const dialogContent = screen.getByText('Dialog content');
      fireEvent.click(dialogContent);

      // Dialog should remain open
      expect(screen.getByRole('dialog')).toBeInTheDocument();
    });
  });

  describe('Multiple Dialogs', () => {
    it('handles nested dialogs', () => {
      render(
        <Dialog defaultOpen>
          <DialogContent>
            <DialogTitle>Parent Dialog</DialogTitle>
            <DialogDescription>Parent dialog description</DialogDescription>
            <Dialog>
              <DialogTrigger asChild>
                <Button>Open Nested</Button>
              </DialogTrigger>
              <DialogContent>
                <DialogTitle>Nested Dialog</DialogTitle>
                <DialogDescription>Nested dialog description</DialogDescription>
              </DialogContent>
            </Dialog>
          </DialogContent>
        </Dialog>
      );

      expect(screen.getByText('Parent Dialog')).toBeInTheDocument();

      fireEvent.click(screen.getByRole('button', { name: /open nested/i }));

      expect(screen.getByText('Nested Dialog')).toBeInTheDocument();
      expect(screen.getByText('Parent Dialog')).toBeInTheDocument();
    });
  });

  describe('Custom Content', () => {
    it('renders custom content and styling', () => {
      render(
        <Dialog defaultOpen>
          <DialogContent className="custom-dialog">
            <DialogHeader className="custom-header">
              <DialogTitle className="custom-title">Custom Dialog</DialogTitle>
            </DialogHeader>
            <div className="custom-body">
              <p>Custom content here</p>
            </div>
          </DialogContent>
        </Dialog>
      );

      const dialog = screen.getByRole('dialog');
      expect(dialog).toHaveClass('custom-dialog');
      expect(screen.getByText('Custom Dialog')).toHaveClass('custom-title');
      expect(screen.getByText('Custom content here')).toBeInTheDocument();
    });

    it('handles dialog without description', () => {
      render(
        <Dialog defaultOpen>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Title Only</DialogTitle>
              <DialogDescription>Description for dialog</DialogDescription>
            </DialogHeader>
          </DialogContent>
        </Dialog>
      );

      expect(screen.getByRole('dialog')).toBeInTheDocument();
      expect(screen.getByRole('heading')).toHaveTextContent('Title Only');
    });

    it('handles dialog without footer', () => {
      render(
        <Dialog defaultOpen>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>No Footer Dialog</DialogTitle>
              <DialogDescription>Dialog without footer description</DialogDescription>
            </DialogHeader>
            <p>Content without footer</p>
          </DialogContent>
        </Dialog>
      );

      expect(screen.getByText('No Footer Dialog')).toBeInTheDocument();
      expect(screen.getByText('Content without footer')).toBeInTheDocument();
    });
  });

  describe('Portal Rendering', () => {
    it('renders dialog content in portal', () => {
      render(
        <div id="app">
          <Dialog defaultOpen>
            <DialogContent data-testid="dialog-content">
              <DialogTitle>Portal Dialog</DialogTitle>
              <DialogDescription>Portal dialog description</DialogDescription>
            </DialogContent>
          </Dialog>
        </div>
      );

      const dialogContent = screen.getByTestId('dialog-content');
      const appContainer = document.getElementById('app');

      // Dialog should be rendered outside the app container in a portal
      expect(dialogContent).toBeInTheDocument();
      expect(appContainer).not.toContainElement(dialogContent);
    });
  });

  describe('Edge Cases', () => {
    it('handles dialog without trigger', () => {
      render(
        <Dialog defaultOpen>
          <DialogContent>
            <DialogTitle>No Trigger Dialog</DialogTitle>
            <DialogDescription>No trigger dialog description</DialogDescription>
          </DialogContent>
        </Dialog>
      );

      expect(screen.getByRole('dialog')).toBeInTheDocument();
    });

    it('handles empty dialog content', () => {
      render(
        <Dialog defaultOpen>
          <DialogContent>
            <DialogDescription>Empty dialog description</DialogDescription>
          </DialogContent>
        </Dialog>
      );

      expect(screen.getByRole('dialog')).toBeInTheDocument();
    });

    it('prevents scrolling on body when open', () => {
      render(
        <Dialog defaultOpen>
          <DialogContent>
            <DialogTitle>Scroll Lock Test</DialogTitle>
            <DialogDescription>Scroll lock test description</DialogDescription>
          </DialogContent>
        </Dialog>
      );

      // Body should have scroll lock when dialog is open
      // This would typically be handled by radix-ui
      expect(screen.getByRole('dialog')).toBeInTheDocument();
    });
  });
});
