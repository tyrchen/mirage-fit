# Testing Infrastructure

This directory contains comprehensive testing utilities and configurations for the Mirage Fit UI application.

## Overview

The testing setup uses **Vitest** as the test runner with **React Testing Library** for component testing, providing a modern, fast, and comprehensive testing environment.

## Technology Stack

- **Test Runner**: Vitest (fast, modern alternative to Jest)
- **Testing Library**: React Testing Library (component testing)
- **Test Environment**: jsdom (DOM simulation)
- **User Interaction**: @testing-library/user-event
- **Mocking**: Vitest built-in mocks
- **Coverage**: v8 coverage provider

## File Structure

```
src/test/
├── setup.ts              # Test setup and global mocks
├── test-utils.tsx        # Custom render functions and utilities
├── accessibility.test.tsx # Comprehensive accessibility tests
└── README.md             # This documentation
```

## Key Features

### 1. Test Setup (`setup.ts`)
- Global test configuration
- Mocks for external dependencies (framer-motion, react-hot-toast, etc.)
- Browser API mocks (FileReader, ResizeObserver, etc.)
- Console error suppression for cleaner test output

### 2. Test Utilities (`test-utils.tsx`)
- Custom render function with providers (React Query, etc.)
- Mock data factories for consistent test data
- Accessibility testing helpers
- Keyboard navigation utilities
- Responsive behavior testing
- Form testing utilities

### 3. Accessibility Testing (`accessibility.test.tsx`)
- Comprehensive keyboard navigation tests
- Screen reader support validation
- Focus management verification
- Color contrast considerations
- Responsive accessibility testing
- ARIA compliance validation

## Usage Examples

### Basic Component Testing
```tsx
import { render, screen } from '../test/test-utils';
import MyComponent from './MyComponent';

test('renders component correctly', () => {
  render(<MyComponent title="Test" />);
  expect(screen.getByText('Test')).toBeInTheDocument();
});
```

### User Interaction Testing
```tsx
import { render, screen } from '../test/test-utils';

test('handles user clicks', async () => {
  const { user } = render(<Button onClick={handleClick}>Click me</Button>);
  await user.click(screen.getByRole('button'));
  expect(handleClick).toHaveBeenCalled();
});
```

### Accessibility Testing
```tsx
import { render, screen } from '../test/test-utils';

test('supports keyboard navigation', () => {
  render(<InteractiveComponent />);
  const button = screen.getByRole('button');

  button.focus();
  expect(document.activeElement).toBe(button);
});
```

### Mock Usage
```tsx
import { vi } from 'vitest';
import { api } from '../services/api';

vi.mock('../services/api');

test('handles API calls', async () => {
  vi.mocked(api.getData).mockResolvedValue({ data: 'test' });
  // ... test implementation
});
```

## Testing Categories

### 1. UI Component Tests
- **Location**: `src/components/ui/__tests__/`
- **Focus**: shadcn/ui base components
- **Coverage**: Props, variants, interactions, accessibility

### 2. Application Component Tests
- **Location**: `src/components/__tests__/`
- **Focus**: Application-specific components
- **Coverage**: Business logic, state management, API integration

### 3. Integration Tests
- **Location**: `src/test/`
- **Focus**: Cross-component interactions
- **Coverage**: User workflows, accessibility compliance

## Mock Strategies

### 1. External Libraries
```tsx
// Framer Motion - simplified for testing
vi.mock('framer-motion', () => ({
  motion: {
    div: 'div',
    button: 'button',
    // ... other elements
  },
}));
```

### 2. API Services
```tsx
// API service mocking
vi.mock('../services/api', () => ({
  api: {
    getData: vi.fn(),
    postData: vi.fn(),
  },
}));
```

### 3. Store/State Management
```tsx
// Zustand store mocking
vi.mock('../store/appStore', () => ({
  useAppStore: vi.fn(),
}));
```

## Coverage Configuration

The test configuration targets comprehensive coverage:
- **Branches**: 80%
- **Functions**: 80%
- **Lines**: 80%
- **Statements**: 80%

Coverage excludes:
- `node_modules/`
- Test files and utilities
- Configuration files
- Type definitions

## Running Tests

### Development Commands
```bash
# Run all tests
npm test

# Run tests in watch mode
npm run test:watch

# Run tests with coverage
npm run test:coverage

# Run tests with UI
npm run test:ui
```

### Test Patterns
- Test files: `*.{test,spec}.{ts,tsx}`
- Location: Adjacent to source files or in `__tests__` directories
- Naming: Descriptive test names using `describe` and `it`

## Best Practices

### 1. Test Organization
- Group related tests with `describe`
- Use descriptive test names
- Test user behavior, not implementation details
- Include edge cases and error scenarios

### 2. Accessibility Focus
- Test keyboard navigation
- Verify screen reader support
- Check ARIA attributes
- Validate focus management

### 3. User-Centric Testing
- Test from user perspective
- Use semantic queries (getByRole, getByLabelText)
- Simulate real user interactions
- Test error states and loading states

### 4. Performance Considerations
- Mock heavy dependencies
- Use efficient test utilities
- Batch related test setup
- Clean up after tests

## Troubleshooting

### Common Issues

1. **Import Errors**: Ensure proper path resolution in test files
2. **Mock Issues**: Check mock placement and timing
3. **Async Testing**: Use proper awaiting for user events
4. **DOM Cleanup**: Tests automatically clean up between runs

### Debug Tips
- Use `screen.debug()` to see rendered DOM
- Add `console.log` in test utilities for debugging
- Use `--reporter verbose` for detailed test output
- Check test isolation by running individual tests

## Contributing

When adding new tests:
1. Follow existing patterns and conventions
2. Include accessibility testing
3. Test both happy path and edge cases
4. Update coverage thresholds if needed
5. Document complex testing scenarios

## Configuration Files

- `vite.config.ts` - Test configuration and coverage settings
- `setup.ts` - Global test setup and mocks
- `tsconfig.json` - TypeScript configuration for tests
