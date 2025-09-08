# Mirage Fit - Frontend

A React-based frontend for the Mirage Fit AI-powered fashion remix application.

## Overview

Mirage Fit allows users to upload their photos and remix them with AI-generated fashion items to create stylized fashion combinations. The frontend provides an intuitive interface for uploading photos, selecting fashion items, and generating remixes.

## Features

- **Photo Upload**: Drag-and-drop photo upload with file validation
- **Fashion Categories**: Organized categories including hats, tops, shoes, accessories, etc.
- **AI Item Generation**: Generate new fashion items for each category
- **Interactive Selection**: Select items from each category for remix
- **Real-time Remix**: Generate AI-powered fashion remixes
- **Gallery View**: Browse generated remixes with modal view
- **Responsive Design**: Works on desktop, tablet, and mobile devices
- **Modern UI**: Glass-morphism effects, smooth animations, and intuitive interactions

## Tech Stack

- **React 19** - UI framework
- **TypeScript** - Type safety
- **Vite** - Build tool and development server
- **TailwindCSS** - Styling and responsive design
- **shadcn/ui** - Component library
- **Framer Motion** - Animations and transitions
- **React Query** - API state management
- **Zustand** - Global state management
- **React Dropzone** - File upload handling
- **React Hot Toast** - Notification system
- **Lucide React** - Icon library

## Development

### Prerequisites

- Node.js 18+ and npm
- Rust backend server running

### Setup

1. Install dependencies:
```bash
npm install
```

2. Start development server:
```bash
npm run dev
```

3. Build for production:
```bash
npm run build
```

### Key Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run lint` - Run ESLint
- `npm run test` - Run tests

## Architecture

The application uses a modern React architecture with:

- **Component-based design** with TypeScript for type safety
- **State management** using Zustand for global state
- **API integration** with a clean service layer
- **Responsive design** using TailwindCSS
- **Animations** with Framer Motion
- **Error handling** with error boundaries and toast notifications

## API Integration

The frontend communicates with the Rust backend through:

- `GET /api/categories` - List available categories
- `GET /api/items/{category}` - Get items for a category
- `POST /api/items/{category}` - Generate new item
- `POST /api/upload` - Upload user photo
- `POST /api/remix` - Generate remix image
- `GET /api/outputs` - List generated remixes
