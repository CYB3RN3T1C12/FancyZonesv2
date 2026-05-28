use crate::layout::{Direction, LayoutTree, pattern::generate_stack_pattern};

impl LayoutTree {
    pub fn hstack(
        start_id: u32,
        window_count: u32,
        stacks: (u32, u32),
    ) -> LayoutTree {
        build_hstack(start_id, window_count, stacks)
    }
}

fn build_hstack(start_id: u32, window_count: u32, stacks: (u32, u32)) -> LayoutTree {
    if window_count <= 1 {
        return LayoutTree::Leaf(start_id);
    }

    let requested_before = stacks.0;
    let requested_after = stacks.1;
    let mut total_stacks = requested_before + requested_after;

    if total_stacks == 0 {
        total_stacks = 1;
    }
    if total_stacks > window_count - 1 {
        total_stacks = window_count - 1;
    }

    let before_stacks = requested_before.min(total_stacks);

    // NEW: unified stack pattern generator
    let counts = generate_stack_pattern(window_count - 1, (before_stacks, total_stacks - before_stacks));

    let mut top_rows = Vec::new();
    let mut bottom_rows = Vec::new();
    let mut next_id = start_id + 1;

    for (i, count) in counts.into_iter().enumerate() {
        let row = build_stack(next_id, count);

        if (i as u32) < before_stacks {
            top_rows.push(row);
        } else {
            bottom_rows.push(row);
        }

        next_id += count;
    }

    let mut rows = Vec::new();
    rows.extend(top_rows);
    rows.push(LayoutTree::Leaf(start_id)); // primary
    rows.extend(bottom_rows);

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
