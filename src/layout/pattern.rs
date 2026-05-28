// stack_pattern.rs

/// Pattern generator for all 1‑D stack‑based layouts:
/// - rows
/// - columns
/// - vstack
/// - hstack
///
/// This replaces all per‑file compute_stack_counts() functions.
pub fn generate_stack_pattern(zones: u32, stacks: (u32, u32)) -> Vec<u32> {
    let mut total = stacks.0 + stacks.1;

    // At least one stack
    if total == 0 {
        total = 1;
    }

    // Cannot have more stacks than zones
    if total > zones && zones > 0 {
        total = zones;
    }

    // Base distribution
    let base = zones / total;
    let extra = zones % total;

    let mut counts = vec![base; total as usize];

    // Distribute extras bottom‑up (rightmost or bottom‑most stacks)
    for i in 0..extra {
        let idx = counts.len() - 1 - i as usize;
        counts[idx] += 1;
    }

    counts
}
