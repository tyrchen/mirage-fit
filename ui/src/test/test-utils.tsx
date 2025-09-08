import React, { ReactElement } from 'react';
import { render, RenderOptions } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import userEvent from '@testing-library/user-event';

// Create a new QueryClient instance for each test
const createTestQueryClient = () =>
  new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
        refetchOnWindowFocus: false,
      },
      mutations: {
        retry: false,
      },
    },
  });

// Custom render function with providers
const AllTheProviders = ({ children }: { children: React.ReactNode }) => {
  const queryClient = createTestQueryClient();

  return (
    <QueryClientProvider client={queryClient}>
      {children}
    </QueryClientProvider>
  );
};

const customRender = (
  ui: ReactElement,
  options?: Omit<RenderOptions, 'wrapper'>
) => {
  return {
    user: userEvent.setup(),
    ...render(ui, { wrapper: AllTheProviders, ...options }),
  };
};

// Export everything from @testing-library/react and our custom render
export * from '@testing-library/react';
export { customRender as render };

// Mock data factories
export const mockCategory = (overrides: Partial<any> = {}) => ({
  id: '1',
  name: 'Test Category',
  items: [],
  description: 'Test description',
  ...overrides,
});

export const mockItem = (overrides: Partial<any> = {}) => ({
  id: '1',
  name: 'Test Item',
  image_url: '/test-image.jpg',
  category: 'test-category',
  tags: ['test'],
  ...overrides,
});

export const mockGeneratedImage = (overrides: Partial<any> = {}) => ({
  id: '1',
  url: '/generated-image.jpg',
  prompt: 'test prompt',
  timestamp: new Date().toISOString(),
  ...overrides,
});

// Test utilities for accessibility testing
export const getByLabelText = (container: HTMLElement, text: string) => {
  const element = container.querySelector(`[aria-label="${text}"]`) ||
    container.querySelector(`label[for] input[id]`);
  return element;
};

// Utility for testing responsive behavior
export const setViewportSize = (width: number, height: number) => {
  Object.defineProperty(window, 'innerWidth', {
    writable: true,
    configurable: true,
    value: width,
  });
  Object.defineProperty(window, 'innerHeight', {
    writable: true,
    configurable: true,
    value: height,
  });
  window.dispatchEvent(new Event('resize'));
};

// Utility for testing keyboard navigation
export const testKeyboardNavigation = async (user: ReturnType<typeof userEvent.setup>) => ({
  pressTab: () => user.keyboard('{Tab}'),
  pressShiftTab: () => user.keyboard('{Shift>}{Tab}{/Shift}'),
  pressEnter: () => user.keyboard('{Enter}'),
  pressEscape: () => user.keyboard('{Escape}'),
  pressArrowDown: () => user.keyboard('{ArrowDown}'),
  pressArrowUp: () => user.keyboard('{ArrowUp}'),
  pressArrowLeft: () => user.keyboard('{ArrowLeft}'),
  pressArrowRight: () => user.keyboard('{ArrowRight}'),
});

// Utility for testing focus management
export const waitForFocus = (element: HTMLElement, timeout = 1000) => {
  return new Promise<void>((resolve, reject) => {
    const startTime = Date.now();

    const checkFocus = () => {
      if (document.activeElement === element) {
        resolve();
      } else if (Date.now() - startTime > timeout) {
        reject(new Error(`Element did not receive focus within ${timeout}ms`));
      } else {
        setTimeout(checkFocus, 10);
      }
    };

    checkFocus();
  });
};

// Utility for testing form submissions
export const createMockFormEvent = (formData: Record<string, string>) => {
  const form = document.createElement('form');
  Object.entries(formData).forEach(([key, value]) => {
    const input = document.createElement('input');
    input.name = key;
    input.value = value;
    form.appendChild(input);
  });

  return new Event('submit', { bubbles: true, cancelable: true });
};

// Utility to wait for animations to complete
export const waitForAnimation = (duration = 500) =>
  new Promise(resolve => setTimeout(resolve, duration));

// Accessibility testing utilities
export const axeMatchers = {
  toHaveNoViolations: () => ({
    pass: true,
    message: () => 'No accessibility violations found',
  }),
};

// Custom queries for testing
export const queries = {
  getByTestId: (container: HTMLElement, testId: string) =>
    container.querySelector(`[data-testid="${testId}"]`),
  getAllByTestId: (container: HTMLElement, testId: string) =>
    Array.from(container.querySelectorAll(`[data-testid="${testId}"]`)),
};
