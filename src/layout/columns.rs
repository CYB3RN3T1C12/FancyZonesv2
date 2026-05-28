use crate::layout::{Direction, LayoutTree, pattern::generate_stack_pattern};

impl LayoutTree {
    pub fn columns(
        start_id: u32,
        window_count: u32,
        stacks: (u32, u32),
    ) -> LayoutTree {
        build_columns(start_id, window_count, stacks)
    }
}

fn build_columns(start_id: u32, zones: u32, stacks: (u32, u32)) -> LayoutTree {
    if zones == 0 {
        return LayoutTree::Leaf(start_id);
    }

    // NEW: unified stack pattern generator
    let counts = generate_stack_pattern(zones, stacks);

    let mut rows = Vec::new();
    let mut next_id = start_id;

    for count in counts {
        let row = build_stack(next_id, count);
        rows.push(row);
        next_id += count;
    }

    fold_vertical(rows)
}

fn build_stack(start_id: u32, count: u32) -> LayoutTree {
    if count == 1 {
        return LayoutTree::Leaf(start_id);
    }

    let mut items = Vec::new();
    for offset in 0..count {
        items.push(LayoutTree::Leaf(start_id + offset));
    }

    fold_horizontal(items)
}

fn fold_horizontal(mut items: Vec<LayoutTree>) -> LayoutTree {
    if items.len() == 1 {
        return items.remove(0);
    }

    let right = items.pop().unwrap();
    let left_count = items.len() as f32;
    let total_count = left_count + 1.0;
    let left = fold_horizontal(items);

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

    let bottom = items.pop().unwrap();
    let top_count = items.len() as f32;
    let total_count = top_count + 1.0;
    let top = fold_vertical(items);

    LayoutTree::Split {
        direction: Direction::Vertical,
        ratio: top_count / total_count,
        left: Box::new(top),
        right: Box::new(bottom),
    }
}
