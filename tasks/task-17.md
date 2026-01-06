# Task 17: Frontend Phase 9 - Complete Missing Core Features

## Overview

Complete the critical missing infrastructure and views for the frontend application. This task focuses on restoring and implementing core functionality required for production readiness, including API client infrastructure, authentication service, Config view, Metrics view, Memory view, and testing infrastructure.

## Dependencies

**REQUIRES:**
- ✅ **Phase 8** - Frontend polish and optimization complete
- ✅ **Phase 5** - Backend API layer complete
- ✅ **OpenAPI Schema** - Generated and available

## Objectives

1. Restore and implement missing API infrastructure
2. Complete placeholder views (Config, Metrics, Memory)
3. Set up testing infrastructure foundation
4. Ensure all views are functional and integrated

## Implementation Tasks

### Item 1: API Client Infrastructure ✅ COMPLETE

**Location**: `frontend/src/services/api.ts`

**Status**: ✅ Complete
- Centralized API client with retry logic
- Error handling and transformation
- Authentication header injection
- Backend URL configuration
- Timeout handling

**Completed**: 2025-01-20

---

### Item 2: Authentication Service ✅ COMPLETE

**Location**: `frontend/src/store/auth.tsx`

**Status**: ✅ Complete
- React Context provider for auth state
- API key storage in localStorage
- `useAuth` hook for consuming auth state
- Integrated in `App.tsx`

**Completed**: 2025-01-20

---

### Item 3: Config View Implementation

**Location**: `frontend/src/views/Config.tsx`

**Requirements**:
- API key input field with show/hide toggle
- API key validation (format checking)
- Save/clear API key buttons
- Backend URL configuration input
- Connection test button
- Status indicators (connected/disconnected)
- Settings persistence
- Integration with auth service
- Form validation
- Success/error notifications

**Code Structure**:
```typescript
export function Config() {
  const { apiKey, setApiKey, clearApiKey } = useAuth();
  const [backendUrl, setBackendUrl] = useState(api.getBaseUrl());
  const [showApiKey, setShowApiKey] = useState(false);
  const [connectionStatus, setConnectionStatus] = useState<'idle' | 'testing' | 'connected' | 'error'>('idle');
  
  // API key management
  // Backend URL configuration
  // Connection testing
  // Form validation
}
```

**Acceptance Criteria**:
- API key can be set and cleared
- Backend URL can be configured
- Connection test works
- Settings persist correctly
- UI follows neo-punk design system
- Form validation works
- Error states display correctly

---

### Item 4: Metrics View Implementation

**Location**: `frontend/src/views/Metrics.tsx` and `frontend/src/components/metrics/`

**Requirements**:
- Create `MetricCard.tsx` component
  - Display single metric with label
  - Value formatting
  - Trend indicators (up/down)
  - Sparkline support (optional)
- Create `TimeSeriesChart.tsx` component
  - Recharts integration
  - Time series data visualization
  - Configurable time ranges
  - Multiple series support
- Create `BarChart.tsx` component
  - Recharts integration
  - Categorical data visualization
  - Customizable colors
- Implement Metrics view
  - Layout with metric cards
  - Time series charts (request rate, latency, error rate)
  - Agent performance metrics
  - State distribution chart
  - Mock data for development (until backend endpoint available)
  - Auto-refresh functionality

**Acceptance Criteria**:
- All metric components render correctly
- Charts display mock data
- Layout is responsive
- Auto-refresh works
- Ready for backend integration

---

### Item 5: Memory View Implementation

**Location**: `frontend/src/views/Memory.tsx` and `frontend/src/components/memory/`

**Requirements**:
- Create `MemoryTierCard.tsx` component
  - Display memory tier information
  - Token count display
  - Capacity indicators
  - Status indicators
- Create `MemoryHierarchy.tsx` component
  - Visual hierarchy of three tiers
  - Connection visualization
  - Token flow indicators
- Create `ConsolidationStatus.tsx` component
  - Consolidation progress
  - Status indicators
  - Last consolidation time
- Implement Memory view
  - Three-tier memory visualization
  - Short-term memory view
  - Medium-term memory view
  - Long-term memory view
  - Consolidation status display
  - Mock data for development
  - Search interface placeholder (for future backend integration)

**Acceptance Criteria**:
- All memory components render correctly
- Three-tier hierarchy is visualized
- Mock data displays correctly
- Layout is responsive
- Ready for backend integration

---

### Item 6: Testing Infrastructure Setup

**Location**: `frontend/` (test setup files)

**Requirements**:
- Install testing dependencies:
  - `vitest` - Test runner
  - `@testing-library/react` - Component testing
  - `@testing-library/jest-dom` - DOM matchers
  - `@testing-library/user-event` - User interaction testing
  - `msw` (Mock Service Worker) - API mocking
  - `@vitest/ui` - Test UI
- Create `vitest.config.ts`
  - Configure test environment
  - Set up React Testing Library
  - Configure paths and aliases
- Create `frontend/src/test/setup.ts`
  - MSW setup
  - Test utilities
  - Mock data factories
- Create `frontend/src/test/mocks/` directory
  - API response mocks
  - Handler setup for MSW
- Add test scripts to `package.json`:
  - `"test": "vitest"`
  - `"test:ui": "vitest --ui"`
  - `"test:coverage": "vitest --coverage"`
- Write initial test examples:
  - API client test
  - Auth service test
  - Simple component test

**Acceptance Criteria**:
- Test infrastructure is set up
- Tests can run successfully
- Test utilities are available
- MSW is configured for API mocking

---

## Testing Requirements

### Unit Tests
- API client functions
- Auth service functions
- Component rendering
- Form validation

### Integration Tests
- Config view flow (set API key, test connection)
- Metrics view rendering
- Memory view rendering

### Test Coverage
- Aim for 70%+ coverage on critical paths
- Test error paths, not just happy paths
- Test user interactions

---

## Design System Compliance

All new components must:
- Follow neo-punk aesthetic
- Use dark mode colors
- Match existing component patterns
- Be responsive (mobile/tablet)
- Include ARIA labels for accessibility

---

## Success Criteria

- [x] API client is functional and tested
- [x] Auth service is functional and tested
- [ ] Config view is complete and functional
- [ ] Metrics view is complete with mock data
- [ ] Memory view is complete with mock data
- [ ] Testing infrastructure is set up
- [ ] All views integrate with API client
- [ ] Code follows project standards
- [ ] All tests pass
- [ ] No linting errors

---

## Implementation Order

1. ✅ **Item 1: API Client** (Foundation) - COMPLETE
2. ✅ **Item 2: Auth Service** (Foundation) - COMPLETE
3. **Item 3: Config View** (Uses auth service) - IN PROGRESS
4. **Item 6: Testing Setup** (Early for validation)
5. **Item 4: Metrics View** (Uses API client)
6. **Item 5: Memory View** (Uses API client)

---

## Notes

- Mock data will be used until backend endpoints are available
- Components should be designed to easily switch from mock to real data
- All API calls should handle errors gracefully
- Follow existing code patterns and structure
- Use TypeScript types matching `src/core/types.rs` exactly

---

## References

- [Frontend PRD](../frontend/tasks/prd.md)
- [Frontend Evaluation](../frontend/tasks/evaluation.md)
- [Phase 9 Plan](../frontend/tasks/phase-9-plan.md)
- [Backend PRD](./prd.md)
- [Backend Architecture](../docs/architecture.md)

---

**Branch**: `feature/frontend-phase-9-core-features`  
**Status**: In Progress  
**Started**: 2025-01-20

