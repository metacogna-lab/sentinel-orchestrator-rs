# Sentinel Orchestrator Frontend - Product Requirements Document

## Overview

Sentinel Orchestrator Frontend is a production-grade, interactive Single Page Application (SPA) built with React + TypeScript + Vite. The frontend provides a rich, engaging interface to visualize and interact with the high-performance Rust backend, emphasizing speed, functionality, and operational excellence.

**Design Philosophy**: Futuristic neo-punk aesthetic with dark mode default, celebrating the Rustafarian crab aesthetic while maintaining professional polish and usability.

## Core Requirements

### 1. Type System Alignment

**Requirement**: All TypeScript interfaces MUST match `src/core/types.rs` contracts exactly.

- Use `frontend/src/types/index.ts` as the source of truth
- All API contracts follow backend types
- No custom types that deviate from backend schema
- Types are immutable contracts

### 2. Design System

**Requirement**: Professional design system with futuristic neo-punk aesthetic, default dark mode.

#### 2.1 Color Palette

**Primary Colors** (Rust-inspired, neo-punk aesthetic):
- **Rust Orange** (`#FF6B35` / `rgb(255, 107, 53)`) - Primary accent, highlights
- **Dark Slate** (`#1A1A2E` / `rgb(26, 26, 46)`) - Primary background
- **Deep Navy** (`#16213E` / `rgb(22, 33, 62)`) - Secondary background, cards
- **Cyan Electric** (`#00D9FF` / `rgb(0, 217, 255)`) - Interactive elements, links
- **Neon Green** (`#39FF14` / `rgb(57, 255, 20)`) - Success states, active indicators
- **Warning Amber** (`#FFB800` / `rgb(255, 184, 0)`) - Warnings, attention
- **Error Red** (`#FF1744` / `rgb(255, 23, 68)`) - Errors, critical states

**Neutral Colors**:
- **Pure White** (`#FFFFFF` / `rgb(255, 255, 255)`) - Primary text
- **Light Gray** (`#E0E0E0` / `rgb(224, 224, 224)`) - Secondary text
- **Medium Gray** (`#9E9E9E` / `rgb(158, 158, 158)`) - Tertiary text, borders
- **Dark Gray** (`#424242` / `rgb(66, 66, 66)`) - Dividers, subtle borders
- **Charcoal** (`#2A2A3E` / `rgb(42, 42, 62)`) - Elevated surfaces

**Semantic Colors**:
- **Info**: Cyan Electric (`#00D9FF`)
- **Success**: Neon Green (`#39FF14`)
- **Warning**: Warning Amber (`#FFB800`)
- **Error**: Error Red (`#FF1744`)

#### 2.2 Typography

**Font Stack**:
- **Primary**: `'JetBrains Mono'`, `'Fira Code'`, `'Consolas'`, `monospace` - Monospace for technical feel
- **Display**: `'Orbitron'`, `'Exo 2'`, `'Rajdhani'`, `sans-serif` - Futuristic sans-serif for headings
- **Body**: `'Inter'`, `'Roboto'`, `system-ui`, `sans-serif` - Clean sans-serif for body text

**Type Scale**:
- **Display 1**: `4rem` / `64px` - Hero headlines
- **Display 2**: `3rem` / `48px` - Page titles
- **H1**: `2.25rem` / `36px` - Section headers
- **H2**: `1.875rem` / `30px` - Subsection headers
- **H3**: `1.5rem` / `24px` - Card titles
- **H4**: `1.25rem` / `20px` - Small headers
- **Body Large**: `1.125rem` / `18px` - Emphasized body
- **Body**: `1rem` / `16px` - Default body text
- **Body Small**: `0.875rem` / `14px` - Secondary text
- **Caption**: `0.75rem` / `12px` - Labels, metadata
- **Code**: `0.875rem` / `14px` - Inline code, terminal

**Font Weights**:
- **Light**: `300` - Display text
- **Regular**: `400` - Body text
- **Medium**: `500` - Emphasized text
- **Semi-Bold**: `600` - Headings
- **Bold**: `700` - Strong emphasis

#### 2.3 Visual Effects

**Neo-Punk Aesthetic Elements**:
- **Glow Effects**: Subtle glow on interactive elements (buttons, inputs, cards on hover)
  - Rust Orange glow: `box-shadow: 0 0 20px rgba(255, 107, 53, 0.3)`
  - Cyan Electric glow: `box-shadow: 0 0 15px rgba(0, 217, 255, 0.4)`
- **Gradients**: Subtle gradients for depth
  - Background: `linear-gradient(135deg, #1A1A2E 0%, #16213E 100%)`
  - Cards: `linear-gradient(180deg, rgba(22, 33, 62, 0.8) 0%, rgba(26, 26, 46, 0.6) 100%)`
- **Border Accents**: Thin neon borders on cards and panels
  - Default: `1px solid rgba(0, 217, 255, 0.2)`
  - Active: `1px solid rgba(0, 217, 255, 0.6)`
- **Grid Patterns**: Subtle background grid pattern for technical feel
- **Animations**: Smooth, purposeful transitions (200-300ms ease-out)
  - Hover states: Scale transforms, glow intensity changes
  - State changes: Color transitions, slide animations
  - Loading: Pulsing glow effects, shimmer loading states

#### 2.4 Component Styling

**Buttons**:
- Primary: Rust Orange background, white text, glow on hover
- Secondary: Transparent with Cyan Electric border, glow on hover
- Ghost: Transparent, colored text, minimal styling
- Danger: Error Red background, white text

**Cards**:
- Background: Dark Slate with subtle gradient
- Border: Thin Cyan Electric border (subtle)
- Hover: Increased glow, slight scale transform
- Padding: `1.5rem` / `24px`

**Inputs**:
- Background: Deep Navy
- Border: Medium Gray (default), Cyan Electric (focus), Error Red (error)
- Text: White
- Focus: Glow effect, border color change
- Placeholder: Medium Gray

**Status Indicators**:
- Active: Pulsing Neon Green dot
- Idle: Static Medium Gray dot
- Thinking: Pulsing Cyan Electric dot
- Error: Pulsing Error Red dot
- Warning: Pulsing Warning Amber dot

## Application Structure

### 1. Views/Routes

**SPA with multiple views**:

1. **Dashboard** (`/`) - Main landing page
   - System health overview
   - Quick metrics cards
   - Recent activity feed
   - Quick action buttons

2. **Chat Interface** (`/chat`) - Interactive chat with agents
   - Real-time streaming responses
   - Conversation history
   - Message input with markdown support
   - Token usage display
   - Agent state indicators

3. **Agent Management** (`/agents`) - Agent monitoring and control
   - Agent list with status cards
   - Real-time state visualization
   - Agent details panel
   - State machine diagram
   - Activity timeline

4. **Metrics & Analytics** (`/metrics`) - Performance metrics and graphs
   - Real-time metrics dashboard
   - Time-series graphs (request rate, latency, error rate)
   - Agent performance metrics
   - Memory tier visualization
   - Token usage trends
   - System resource usage

5. **Memory System** (`/memory`) - Three-tier memory visualization
   - Short-term memory view (in-memory)
   - Medium-term memory view (Sled)
   - Long-term memory view (Qdrant)
   - Consolidation status
   - Search interface for long-term memory

6. **Configuration** (`/config`) - System configuration guide
   - API key management UI
   - Configuration documentation
   - Environment variable reference
   - Connection settings
   - Backend URL configuration

7. **Documentation** (`/docs`) - Links to Rust documentation
   - Embedded documentation viewer
   - Links to backend docs
   - API reference
   - Architecture diagrams
   - Getting started guide

8. **CLI Integration** (`/cli`) - rs_cli integration view
   - CLI command reference
   - Terminal emulator component (if feasible)
   - CLI output visualization
   - Integration guide

### 2. Core Features

#### 2.1 Real-Time Updates
- WebSocket or Server-Sent Events for real-time data
- Polling fallback for compatibility
- Optimistic UI updates
- Connection status indicator

#### 2.2 Streaming Chat
- Real-time token streaming from backend
- Markdown rendering
- Syntax highlighting for code blocks
- Copy-to-clipboard functionality
- Message actions (regenerate, copy, delete)

#### 2.3 Metrics Visualization
- Real-time charts using Chart.js or Recharts
- Time-series graphs with configurable time ranges
- Custom metrics cards with sparklines
- Export functionality (CSV, PNG)

#### 2.4 State Machine Visualization
- Interactive state diagram
- Current state highlighting
- State transition history
- Visual state machine representation

#### 2.5 Memory Visualization
- Three-tier memory hierarchy visualization
- Token count indicators
- Consolidation progress
- Search interface with semantic search

## Technical Stack

### Core Dependencies

**Framework & Build**:
- `react` (`^19.1.0`) - UI framework
- `react-dom` (`^19.1.0`) - React DOM renderer
- `vite` (`^6.3.5`) - Build tool and dev server
- `typescript` (`~5.8.3`) - Type safety

**Routing**:
- `react-router-dom` (`^7.x`) - Client-side routing
- `react-router` - Core routing

**State Management**:
- `zustand` (`^5.x`) or `jotai` (`^2.x`) - Lightweight state management
- React Context for theme and global state

**Styling**:
- `tailwindcss` (`^3.x`) - Utility-first CSS framework
- `@tailwindcss/typography` - Typography plugin
- `postcss` - CSS processing
- `autoprefixer` - CSS vendor prefixes
- Custom CSS variables for theme system

**Data Fetching**:
- `axios` (`^1.x`) or `fetch` API - HTTP client
- `react-query` (`tanstack-query` `^5.x`) - Server state management
- WebSocket client for real-time updates

**Charts & Visualization**:
- `recharts` (`^2.x`) or `chart.js` + `react-chartjs-2` - Data visualization
- `react-flow` (`^11.x`) - State machine diagrams

**UI Components**:
- `@headlessui/react` (`^2.x`) - Unstyled accessible components
- `framer-motion` (`^11.x`) - Animation library
- `react-markdown` (`^9.x`) - Markdown rendering
- `prism-react-renderer` (`^2.x`) - Syntax highlighting

**Utilities**:
- `date-fns` (`^3.x`) - Date formatting
- `clsx` (`^2.x`) - Conditional class names
- `zod` (`^3.x`) - Runtime type validation (optional)

### Development Dependencies

- `@types/react` (`^19.1.2`)
- `@types/react-dom` (`^19.1.2`)
- `eslint` (`^9.25.0`)
- `typescript-eslint` (`^8.30.1`)
- `@vitejs/plugin-react` (`^4.4.1`)

## API Integration

### Backend API Endpoints

**Base URL**: `http://localhost:3000` (development, configurable)

**Endpoints**:
- `GET /health` - Health check (public)
- `GET /health/ready` - Readiness check (public)
- `GET /health/live` - Liveness check (public)
- `POST /v1/chat/completions` - Chat completions (requires Write auth)
- `GET /v1/agents/status` - Agent status (requires Read auth)
- `GET /metrics` - Prometheus metrics (public, when available)

### Authentication

- API key authentication via `Authorization: Bearer <key>` header
- API key stored in localStorage (configurable)
- API key management UI in Configuration view
- Support for multiple API keys (future)

### Error Handling

- Global error boundary
- API error handling with user-friendly messages
- Retry logic for transient failures
- Connection status monitoring

## Development Phases

### Phase 1: Foundation & Design System

**Objectives**: Establish design system, project structure, and core infrastructure.

**Deliverables**:
1. Design system implementation (colors, typography, components)
2. Theme system (dark mode default, CSS variables)
3. Project structure (folders, routing setup)
4. Basic layout components (Header, Sidebar, Footer)
5. Global styles and typography
6. Tailwind configuration with custom theme
7. Basic navigation

**Acceptance Criteria**:
- Design system documented and implemented
- Dark mode default functional
- Typography and color system in place
- Basic layout renders correctly
- Navigation between placeholder views works

### Phase 2: Core Views & API Integration

**Objectives**: Implement core views and API client integration.

**Deliverables**:
1. API client with authentication
2. React Query setup for data fetching
3. Dashboard view with health status
4. Chat interface view (basic structure)
5. Agent Management view (basic structure)
6. Error handling and loading states
7. Configuration view (API key management)

**Acceptance Criteria**:
- API client connects to backend successfully
- Health checks display correctly
- Authentication works (API key)
- All core views render with placeholder data
- Error states display correctly

### Phase 3: Chat Interface & Streaming

**Objectives**: Full chat interface with streaming support.

**Deliverables**:
1. Chat message list component
2. Message input with markdown preview
3. Streaming response handling
4. Markdown rendering with syntax highlighting
5. Message actions (copy, regenerate)
6. Conversation history
7. Token usage display

**Acceptance Criteria**:
- Chat interface sends messages to backend
- Streaming responses display in real-time
- Markdown renders correctly
- Code blocks have syntax highlighting
- Message history persists (localStorage)
- Token usage displays correctly

### Phase 4: Agent Management & State Visualization

**Objectives**: Agent monitoring with state machine visualization.

**Deliverables**:
1. Agent list component
2. Agent status cards
3. State machine diagram component
4. Agent details panel
5. Real-time state updates
6. Activity timeline
7. State transition visualization

**Acceptance Criteria**:
- Agent list displays all agents
- Agent status updates in real-time
- State machine diagram shows current state
- Agent details panel shows full information
- State transitions visualize correctly

### Phase 5: Metrics & Analytics

**Objectives**: Comprehensive metrics dashboard with real-time charts.

**Deliverables**:
1. Metrics dashboard layout
2. Time-series charts (request rate, latency, error rate)
3. Agent performance metrics
4. Memory tier visualization
5. Token usage trends
6. System resource charts
7. Metric cards with sparklines

**Acceptance Criteria**:
- Metrics dashboard displays correctly
- Charts update in real-time
- All metric types render correctly
- Time range selection works
- Export functionality works

### Phase 6: Memory System Visualization

**Objectives**: Three-tier memory system visualization and search.

**Deliverables**:
1. Memory tier hierarchy visualization
2. Short-term memory view
3. Medium-term memory view (Sled)
4. Long-term memory view (Qdrant)
5. Consolidation status display
6. Memory search interface
7. Token count indicators

**Acceptance Criteria**:
- All three memory tiers display correctly
- Memory visualization is interactive
- Search interface works (when backend supports it)
- Consolidation status displays correctly
- Token counts update in real-time

### Phase 7: Documentation & CLI Integration

**Objectives**: Documentation viewer and CLI integration view.

**Deliverables**:
1. Documentation viewer component
2. Embedded documentation (if possible)
3. Links to backend documentation
4. CLI integration view
5. CLI command reference
6. Integration guide

**Acceptance Criteria**:
- Documentation view displays correctly
- Links to external docs work
- CLI integration view provides useful information
- Command reference is complete

### Phase 8: Polish & Optimization

**Objectives**: Performance optimization, accessibility, and polish.

**Deliverables**:
1. Performance optimization (code splitting, lazy loading)
2. Accessibility improvements (ARIA labels, keyboard navigation)
3. Loading states and skeletons
4. Error boundaries
5. Responsive design (mobile, tablet)
6. PWA features (optional)
7. Testing (unit tests, integration tests)

**Acceptance Criteria**:
- Application loads quickly (< 3s initial load)
- All views are accessible
- Mobile responsive design works
- Error boundaries catch errors gracefully
- Loading states provide good UX

## File Structure

```
frontend/
├── src/
│   ├── components/          # Reusable UI components
│   │   ├── ui/              # Base UI components (Button, Card, Input, etc.)
│   │   ├── layout/          # Layout components (Header, Sidebar, Footer)
│   │   ├── chat/            # Chat-specific components
│   │   ├── agents/          # Agent-specific components
│   │   ├── metrics/         # Metrics-specific components
│   │   └── memory/          # Memory-specific components
│   ├── views/               # Page-level components
│   │   ├── Dashboard.tsx
│   │   ├── Chat.tsx
│   │   ├── Agents.tsx
│   │   ├── Metrics.tsx
│   │   ├── Memory.tsx
│   │   ├── Config.tsx
│   │   ├── Docs.tsx
│   │   └── CLI.tsx
│   ├── hooks/               # Custom React hooks
│   ├── services/            # API services
│   │   ├── api.ts           # API client
│   │   ├── auth.ts          # Authentication
│   │   └── websocket.ts     # WebSocket client (if used)
│   ├── store/               # State management
│   │   ├── theme.ts         # Theme store
│   │   ├── auth.ts          # Auth store
│   │   └── app.ts           # App state
│   ├── types/               # TypeScript types (matches backend)
│   │   └── index.ts
│   ├── utils/               # Utility functions
│   ├── styles/              # Global styles
│   │   ├── globals.css
│   │   └── theme.css
│   ├── App.tsx              # Main App component
│   ├── main.tsx             # Entry point
│   └── vite-env.d.ts        # Vite type definitions
├── public/                  # Static assets
├── tasks/                   # Frontend task documentation
│   ├── prd.md              # This document
│   └── bridge.md           # State tracking
├── package.json
├── tsconfig.json
├── vite.config.ts
├── tailwind.config.js
├── postcss.config.js
└── README.md
```

## Design Inspirations

**High-Capacity API-Based Interfaces**:
- GitHub Actions UI - Clean metrics visualization
- Grafana Dashboards - Rich analytics interface
- Stripe Dashboard - Professional polish
- Vercel Dashboard - Modern design language
- Railway Dashboard - Developer-focused UX

**Neo-Punk Aesthetic References**:
- Cyberpunk 2077 UI - Neon accents, dark themes
- Blade Runner aesthetic - Futuristic, tech-forward
- Tron Legacy - Grid patterns, glowing borders
- Rust programming language branding - Orange and black

## Success Criteria

1. **Performance**: 
   - Initial load < 3 seconds
   - Page transitions < 200ms
   - Real-time updates < 100ms latency

2. **Usability**:
   - Intuitive navigation
   - Clear visual hierarchy
   - Accessible (WCAG 2.1 AA compliance)

3. **Visual Design**:
   - Consistent design system
   - Professional neo-punk aesthetic
   - Dark mode default

4. **Functionality**:
   - All views functional
   - Real-time updates working
   - API integration complete

## References

- [Backend PRD](../tasks/prd.md)
- [Backend Architecture](../docs/architecture.md)
- [Backend API Documentation](../docs/api.md)
- [rs_cli README](../rs_cli/README.md)
- [Frontend Types](../src/types/index.ts)

---

**Last Updated**: 2025-01-20
**Version**: 1.0.0

