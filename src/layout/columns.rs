use crate::layout::{Direction, LayoutTree, Rect};

impl LayoutTree {
    pub fn columns(
        start_id: u32,
        window_count: u32,
        stacks: (u32, u32),
        screen: &Rect,
    ) -> LayoutTree {
        let total_stacks = (stacks.0 + stacks.1).max(1);
        build_columns(start_id, window_count, total_stacks, screen)
    }
}

fn build_columns(start_id: u32, zones: u32, stacks: u32, _screen: &Rect) -> LayoutTree {
    if zones == 0 {
        return LayoutTree::Leaf(start_id);
    }

    let stack_counts = distribute_columns(zones, stacks);
    let mut next_id = start_id;
    let mut rows = Vec::with_capacity(stack_counts.len());

    for &count in &stack_counts {
        rows.push(build_row(next_id, count));
        next_id += count;
    }

    fold_vertical(rows)
}

fn distribute_columns(zones: u32, stacks: u32) -> Vec<u32> {
    let stacks = stacks.max(1).min(zones.max(1));
    let base = zones / stacks;
    let extra = zones % stacks;
    let mut counts = vec![base; stacks as usize];

    // Bottom-most stacks absorb growth first.
    for index in 0..extra as usize {
        let stack = counts.len() - 1 - index;
        counts[stack] += 1;
    }

    counts
}

fn build_row(start_id: u32, count: u32) -> LayoutTree {
    let mut leaves = Vec::with_capacity(count as usize);

    for offset in 0..count {
        leaves.push(LayoutTree::Leaf(start_id + offset));
    }

    fold_horizontal(leaves)
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
