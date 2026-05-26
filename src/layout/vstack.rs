use crate::layout::{Direction, LayoutTree, Rect};

impl LayoutTree {
    pub fn vstack(
        start_id: u32,
        window_count: u32,
        stacks: (u32, u32),
        screen: &Rect,
    ) -> LayoutTree {
        build_vstack(start_id, window_count, stacks, screen)
    }
}

fn build_vstack(
    start_id: u32,
    window_count: u32,
    stacks: (u32, u32),
    _screen: &Rect,
) -> LayoutTree {
    if window_count <= 1 {
        return LayoutTree::Leaf(start_id);
    }

    let requested_before = stacks.0;
    let requested_after = stacks.1;
    let total_requested = requested_before + requested_after;
    let total_stacks = total_requested.max(1).min(window_count - 1);
    let before_stacks = requested_before.min(total_stacks);
    let after_stacks = total_stacks - before_stacks;

    let stack_counts = distribute_rows(window_count - 1, total_stacks);
    let split_at = before_stacks as usize;

    let left_counts = &stack_counts[..split_at];
    let right_counts = &stack_counts[split_at..];

    let mut next_id = start_id + 1;
    let mut left_columns = Vec::with_capacity(left_counts.len());
    for &count in left_counts {
        left_columns.push(build_stack(next_id, count));
        next_id += count;
    }

    let priority = LayoutTree::Leaf(start_id);

    let mut right_columns = Vec::with_capacity(right_counts.len());
    for &count in right_counts {
        right_columns.push(build_stack(next_id, count));
        next_id += count;
    }

    let mut columns = Vec::with_capacity(left_columns.len() + 1 + right_columns.len());
    columns.extend(left_columns);
    columns.push(priority);
    columns.extend(right_columns);

    fold_horizontal(columns)
}

fn distribute_rows(zones: u32, stacks: u32) -> Vec<u32> {
    let stacks = stacks.max(1).min(zones.max(1));
    let base = zones / stacks;
    let extra = zones % stacks;
    let mut counts = vec![base; stacks as usize];

    // Rightmost stacks absorb growth first.
    for index in 0..extra as usize {
        let stack = counts.len() - 1 - index;
        counts[stack] += 1;
    }

    counts
}

fn build_stack(start_id: u32, count: u32) -> LayoutTree {
    if count == 1 {
        return LayoutTree::Leaf(start_id);
    }

    let mut leaves = Vec::with_capacity(count as usize);
    for offset in 0..count {
        leaves.push(LayoutTree::Leaf(start_id + offset));
    }

    fold_vertical(leaves)
}

fn fold_horizontal(mut items: Vec<LayoutTree>) -> LayoutTree {
    debug_assert!(!items.is_empty());

    if items.len() == 1 {
        return items.remove(0);
    }

    let right = items.pop().unwrap();
    let right_weight = 1.0f32;
    let left_weight = items.len() as f32;
    let left = fold_horizontal(items);

    LayoutTree::Split {
        direction: Direction::Horizontal,
        ratio: left_weight / (left_weight + right_weight),
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn fold_vertical(mut items: Vec<LayoutTree>) -> LayoutTree {
    debug_assert!(!items.is_empty());

    if items.len() == 1 {
        return items.remove(0);
    }

    let bottom = items.pop().unwrap();
    let bottom_weight = 1.0f32;
    let top_weight = items.len() as f32;
    let top = fold_vertical(items);

    LayoutTree::Split {
        direction: Direction::Vertical,
        ratio: top_weight / (top_weight + bottom_weight),
        left: Box::new(top),
        right: Box::new(bottom),
    }
}
