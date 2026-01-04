# Task 10: Token Counting and Consolidation Triggers

## Overview

Implement accurate token counting and sophisticated consolidation trigger logic. This ensures memory consolidation happens at optimal times and prevents memory overflow.

## Dependencies

**REQUIRES:**
- ✅ **Task 6** (Short-Term Memory) - Basic token counting
- ✅ **Task 9** (Memory Manager) - Consolidation framework

## Objectives

1. Implement accurate token counting (beyond simple approximation)
2. Create configurable consolidation triggers
3. Add token budget tracking
4. Implement consolidation prioritization

## Implementation Tasks

### 1. Token Counter Trait

**Location**: `src/memory/token_counter.rs` (new file)

**Requirements**:
- Define trait for token counting
- Support multiple counting strategies
- Allow switching between simple and accurate counting

**Code Structure**:
```rust
pub trait TokenCounter: Send + Sync {
    fn count_tokens(&self, text: &str) -> u64;
    fn count_message(&self, msg: &CanonicalMessage) -> u64;
    fn count_messages(&self, messages: &[CanonicalMessage]) -> u64;
}

// Simple implementation (chars / 4)
pub struct SimpleTokenCounter;

// Accurate implementation (future: tiktoken or similar)
pub struct AccurateTokenCounter {
    // Tokenizer instance
}
```

### 2. Consolidation Trigger Configuration

**Location**: `src/memory/manager.rs` (extend)

**Requirements**:
- Configurable thresholds per tier
- Multiple trigger conditions
- Priority-based consolidation

**Trigger Configuration**:
```rust
pub struct ConsolidationConfig {
    pub short_term_token_threshold: u64,
    pub short_term_message_threshold: usize,
    pub medium_term_summary_threshold: usize,
    pub medium_term_age_threshold: Duration,
    pub enable_auto_consolidation: bool,
}
```

### 3. Token Budget Tracking

**Requirements**:
- Track total tokens across all tiers
- Enforce global token budget (if configured)
- Prevent memory overflow
- Log token usage for observability

**Budget Tracking**:
```rust
pub struct TokenBudget {
    pub short_term_tokens: u64,
    pub medium_term_tokens: u64,
    pub long_term_tokens: u64,
    pub max_total_tokens: Option<u64>,
}

impl TokenBudget {
    pub fn total(&self) -> u64 {
        self.short_term_tokens + self.medium_term_tokens + self.long_term_tokens
    }
    
    pub fn exceeds_budget(&self) -> bool {
        if let Some(max) = self.max_total_tokens {
            self.total() > max
        } else {
            false
        }
    }
}
```

### 4. Consolidation Prioritization

**Requirements**:
- Prioritize short-term consolidation (most urgent)
- Batch medium-term consolidation
- Schedule long-term consolidation during low activity
- Prevent consolidation storms

**Priority Logic**:
```rust
pub enum ConsolidationPriority {
    Critical,  // Short-term overflow imminent
    High,      // Short-term threshold exceeded
    Medium,    // Medium-term ready
    Low,       // Long-term maintenance
}

impl MemoryManager {
    fn determine_consolidation_priority(&self) -> Option<ConsolidationPriority> {
        if self.short_term_token_count() > self.config.short_term_token_threshold * 2 {
            Some(ConsolidationPriority::Critical)
        } else if self.should_consolidate_short() {
            Some(ConsolidationPriority::High)
        } else if self.should_consolidate_medium() {
            Some(ConsolidationPriority::Medium)
        } else {
            None
        }
    }
}
```

### 5. Integration with Memory Manager

**Requirements**:
- Update memory manager to use accurate token counting
- Implement consolidation trigger evaluation
- Add token budget enforcement
- Log consolidation events

## Testing Requirements

### Unit Tests

**Location**: `src/memory/token_counter.rs` and `src/memory/manager.rs`

**Test Cases**:
1. ✅ Simple token counter approximates correctly
2. ✅ Token counting for messages works
3. ✅ Consolidation triggers fire at correct thresholds
4. ✅ Token budget enforcement works
5. ✅ Consolidation priority logic correct
6. ✅ Multiple trigger conditions evaluated correctly

**Test Pattern**:
```rust
#[test]
fn test_token_counting() {
    let counter = SimpleTokenCounter;
    let text = "Hello, world!";
    let tokens = counter.count_tokens(text);
    assert!(tokens > 0);
    assert_eq!(tokens, text.len() as u64 / 4); // Simple approximation
}

#[test]
fn test_consolidation_trigger() {
    let config = ConsolidationConfig {
        short_term_token_threshold: 1000,
        // ...
    };
    let manager = MemoryManager::with_config(config);
    
    // Fill memory
    // ...
    
    assert!(manager.should_consolidate_short());
}
```

## Acceptance Criteria

- [ ] Token counter trait defined and implemented
- [ ] Consolidation triggers configurable
- [ ] Token budget tracking functional
- [ ] Consolidation prioritization works
- [ ] All tests pass: `cargo test memory::token_counter memory::manager`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code formatted: `cargo fmt --check`
- [ ] Configuration via environment/config file

## Error Handling

- Use `thiserror` for domain errors
- Handle token counting errors gracefully
- Log all consolidation decisions
- Never panic on budget exceeded - trigger consolidation

## Performance Considerations

- Token counting should be fast (cache if needed)
- Consolidation triggers should be lightweight checks
- Avoid blocking actor operations during consolidation

## Future Enhancements

- Accurate token counting with tiktoken or similar
- Machine learning for optimal consolidation timing
- Predictive consolidation based on usage patterns

## References

- PRD Section: "Memory System" (lines 94-108)
- PRD Section: "Consolidation Trigger" (lines 104-108)
- Architecture Doc: "Memory Management" (lines 162-179)

## Phase 3 Completion

After completing this task, **Phase 3: Memory System** is complete. All components are in place:
- ✅ Short-term memory (Task 6)
- ✅ Medium-term memory (Task 7)
- ✅ Long-term memory (Task 8)
- ✅ Memory manager (Task 9)
- ✅ Token counting and triggers (Task 10)

**Next Phase**: Proceed to **Phase 4: Integration** (see PRD lines 207-210)

