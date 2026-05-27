use crate::layout::{Direction, LayoutTree};

impl LayoutTree {
    pub fn hstack(
        start_id: u32,
        window_count: u32,
        stacks: (u32, u32)
    ) -> LayoutTree {
        build_hstack(start_id, window_count, stacks)
    }
}

fn build_hstack(
    start_id: u32,
    window_count: u32,
    stacks: (u32, u32)
) -> LayoutTree {
    if window_count <= 1 {
        return LayoutTree::Leaf(start_id);
    }

    let requested_before: u32 = stacks.0;
    let requested_after: u32 = stacks.1;
    let mut total_stacks: u32 = requested_before + requested_after;

    if total_stacks == 0 {
        total_stacks = 1;
    }

    if total_stacks > window_count - 1 {
        total_stacks = window_count - 1;
    }

    let mut before_stacks: u32 = requested_before;
    if before_stacks > total_stacks {
        before_stacks = total_stacks;
    }

    let counts: Vec<u32> = compute_stack_counts(window_count - 1, total_stacks);

    let mut top_rows: Vec<LayoutTree> = Vec::new();
    let mut bottom_rows: Vec<LayoutTree> = Vec::new();
    let mut next_id = start_id + 1;

    for i in 0..counts.len() {
        let count = counts[i];
        let row = build_stack(next_id, count);

        if i < before_stacks as usize {
            top_rows.push(row);
        } else {
            bottom_rows.push(row);
        }

        next_id += count;
    }

    let mut rows: Vec<LayoutTree> = Vec::new();

    for row in top_rows {
        rows.push(row);
    }

    rows.push(LayoutTree::Leaf(start_id));

    for row in bottom_rows {
        rows.push(row);
    }

    fold_vertical(rows)
}

fn compute_stack_counts(zones: u32, stacks: u32) -> Vec<u32> {
    let mut real_stacks = stacks;

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

    let mut i: u32 = 0;
    while i < extra {
        let index: usize = counts.len() - 1 - i as usize;
        counts[index] += 1;
        i += 1;
    }

    counts
}

fn build_stack(start_id: u32, count: u32) -> LayoutTree {
    if count == 1 {
        return LayoutTree::Leaf(start_id);
    }

    let mut items: Vec<LayoutTree> = Vec::new();

    for offset in 0..count {
        items.push(LayoutTree::Leaf(start_id + offset));
    }

    fold_horizontal(items)
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