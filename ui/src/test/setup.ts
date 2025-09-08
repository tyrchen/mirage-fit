import '@testing-library/jest-dom';
import { vi, beforeAll, afterAll } from 'vitest';

// Mock framer-motion
vi.mock('framer-motion', () => ({
  motion: {
    div: 'div',
    h1: 'h1',
    button: 'button',
    span: 'span',
    img: 'img',
    section: 'section',
    header: 'header',
    main: 'main',
    aside: 'aside',
    nav: 'nav',
    footer: 'footer',
  },
  AnimatePresence: ({ children }: { children: React.ReactNode }) => children,
  useAnimation: () => ({
    start: vi.fn(),
    stop: vi.fn(),
    set: vi.fn(),
  }),
}));

// Mock react-hot-toast
vi.mock('react-hot-toast', () => ({
  toast: {
    success: vi.fn(),
    error: vi.fn(),
    loading: vi.fn(),
    dismiss: vi.fn(),
  },
  Toaster: () => null,
}));

// Mock React Query
vi.mock('@tanstack/react-query', () => ({
  QueryClient: vi.fn(() => ({
    invalidateQueries: vi.fn(),
    getQueryData: vi.fn(),
    setQueryData: vi.fn(),
  })),
  QueryClientProvider: ({ children }: { children: React.ReactNode }) => children,
  useQuery: vi.fn(() => ({
    data: null,
    isLoading: false,
    error: null,
    isError: false,
    isSuccess: true,
  })),
  useMutation: vi.fn(() => ({
    mutate: vi.fn(),
    isLoading: false,
    error: null,
    data: null,
  })),
}));

// Mock file reading APIs
Object.defineProperty(global, 'FileReader', {
  writable: true,
  value: vi.fn().mockImplementation(() => ({
    readAsDataURL: vi.fn(),
    readAsText: vi.fn(),
    onload: null,
    onerror: null,
    result: null,
  })),
});

// Mock URL.createObjectURL
Object.defineProperty(global.URL, 'createObjectURL', {
  writable: true,
  value: vi.fn(() => 'mock-url'),
});

Object.defineProperty(global.URL, 'revokeObjectURL', {
  writable: true,
  value: vi.fn(),
});

// Mock ResizeObserver
global.ResizeObserver = vi.fn().mockImplementation(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn(),
}));

// Mock IntersectionObserver
global.IntersectionObserver = vi.fn().mockImplementation(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn(),
}));

// Mock window.matchMedia
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation((query) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(), // deprecated
    removeListener: vi.fn(), // deprecated
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});

// Mock scrollTo
Object.defineProperty(window, 'scrollTo', {
  writable: true,
  value: vi.fn(),
});

// Mock console methods for cleaner test output
const originalError = console.error;
beforeAll(() => {
  console.error = vi.fn();
});

afterAll(() => {
  console.error = originalError;
});
