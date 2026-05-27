use crate::layout::{Direction, LayoutTree};

impl LayoutTree {
    pub fn rows(
        start_id: u32,
        window_count: u32,
        stacks: (u32, u32)
    ) -> LayoutTree {
        let total_stacks: u32 = stacks.0 + stacks.1;

        if total_stacks == 0 {
            return build_rows(start_id, window_count, 1);
        }

        build_rows(start_id, window_count, total_stacks)
    }
}

fn build_rows(start_id: u32, zones: u32, stacks: u32) -> LayoutTree {
    if zones == 0 {
        return LayoutTree::Leaf(start_id);
    }

    let counts: Vec<u32> = compute_stack_counts(zones, stacks);

    let mut columns: Vec<LayoutTree> = Vec::new();
    let mut next_id = start_id;

    for i in 0..counts.len() {
        let count: u32 = counts[i];
        let column: LayoutTree = build_column(next_id, count);
        columns.push(column);
        next_id += count;
    }

    fold_horizontal(columns)
}

fn compute_stack_counts(zones: u32, stacks: u32) -> Vec<u32> {
    let mut real_stacks: u32 = stacks;

    if real_stacks == 0 {
        real_stacks = 1;
    }

    if real_stacks > zones && zones > 0 {
        real_stacks = zones;
    }

    let base: u32 = zones / real_stacks;
    let extra: u32 = zones % real_stacks;

    let mut counts: Vec<u32> = Vec::new();

    for _ in 0..real_stacks {
        counts.push(base);
    }

    let mut i = 0;
    while i < extra {
        let index = counts.len() - 1 - i as usize;
        counts[index] += 1;
        i += 1;
    }

    counts
}

fn build_column(start_id: u32, count: u32) -> LayoutTree {
    if count == 1 {
        return LayoutTree::Leaf(start_id);
    }

    let mut items: Vec<LayoutTree> = Vec::new();

    for offset in 0..count {
        items.push(LayoutTree::Leaf(start_id + offset));
    }

    fold_vertical(items)
}

fn fold_horizontal(mut items: Vec<LayoutTree>) -> LayoutTree {
    if items.len() == 1 {
        return items.remove(0);
    }

    let right: LayoutTree = items.pop().unwrap();
    let left_count: f32 = items.len() as f32;
    let total_count: f32 = left_count + 1.0;
    let left: LayoutTree = fold_horizontal(items);

    LayoutTree::Split {
        direction: Direction::Horizontal,
        ratio: left_count / total_count,
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn fold_vertical(mut items: Vec<LayoutTree>) -> LayoutTree {
    if items.len() == 1 {
        return items.remove(0);
    }

    let bottom: LayoutTree = items.pop().unwrap();
    let top_count: f32 = items.len() as f32;
    let total_count: f32 = top_count + 1.0;
    let top: LayoutTree = fold_vertical(items);

    LayoutTree::Split {
        direction: Direction::Vertical,
        ratio: top_count / total_count,
        left: Box::new(top),
        right: Box::new(bottom),
    }
}