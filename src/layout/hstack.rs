use crate::layout::{Tree, LayoutKind, Layout, stacks::generate_stack_pattern, fold::{fold_horizontal, fold_vertical}};

impl Tree {
    /// Build a hstack layout
    /// 
    /// ## Arguments
    /// * `start_id` - The id of the first window
    /// * `count` - The number of windows
    /// * `stacks` - The number of stacks
    /// 
    /// ## Returns
    /// * a `Tree` - The root of the hstack layout
    pub fn hstack(
        start_id: u32,
        zones: u32,
        stacks: (u32, u32),
    ) -> Tree {
        build_hstack(start_id, zones, stacks)
    }
}

/// Build a hstack layout
/// 
/// ## Arguments
/// * `start_id` - The id of the first window
/// * `zones` - The number of windows
/// * `stacks` - The number of stacks
/// 
/// ## Returns
/// * a `Tree` - The root of the hstack layout
fn build_hstack(start_id: u32, zones: u32, stacks: (u32, u32)) -> Tree {
    if zones <= 1 {
        return Tree::Leaf(start_id);
    }

    let requested_before = stacks.0;
    let requested_after = stacks.1;
    let mut total_stacks = requested_before + requested_after;

    if total_stacks == 0 {
        total_stacks = 1;
    }
    if total_stacks > zones - 1 {
        total_stacks = zones - 1;
    }

    let before_stacks = requested_before.min(total_stacks);

    // NEW: unified stack pattern generator
    let counts = generate_stack_pattern(zones - 1, (before_stacks, total_stacks - before_stacks));

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
    rows.push(Tree::Leaf(start_id)); // primary
    rows.extend(bottom_rows);

    fold_vertical(rows)
}

/// Build a single row of columns (horizontal stacking)
/// 
/// ## Arguments
/// * `start_id` - The id of the first window
/// * `count` - The number of windows
/// 
/// ## Returns
/// * a `Tree` - The root of the row
fn build_stack(start_id: u32, count: u32) -> Tree {
    if count == 1 {
        return Tree::Leaf(start_id);
    }

    let mut items = Vec::new();
    for offset in 0..count {
        items.push(Tree::Leaf(start_id + offset));
    }

    fold_horizontal(items)
}

impl Layout {
    pub fn hstack(start_id: u32, zones: u32) -> Self {
        Self {
            kind: LayoutKind::HStack,
            start_id,
            zones,
            stacks: (0, 1),
            external_padding: 0,
            internal_padding: 0,
        }
    }
}