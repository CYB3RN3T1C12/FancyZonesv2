use crate::layout::{Geometry, LayoutKind, Layout, stacks::generate_stack_pattern, fold::{fold_horizontal, fold_vertical}};

impl Geometry {
    /// Build a vstack layout
    /// 
    /// ## Arguments
    /// * `start_id` - The id of the first window
    /// * `count` - The number of windows
    /// * `stacks` - The number of stacks
    /// 
    /// ## Returns
    /// * a `Geometry` - The root of the vstack layout
    pub fn vstack(
        start_id: u32,
        zones: u32,
        stacks: (u32, u32),
    ) -> Geometry {
        build_vstack(start_id, zones, stacks)
    }
}

/// Build a vstack layout
/// 
/// ## Arguments
/// * `start_id` - The id of the first window
/// * `zones` - The number of windows
/// * `stacks` - The number of stacks
/// 
/// ## Returns
/// * a `Geometry` - The root of the vstack layout
fn build_vstack(start_id: u32, zones: u32, stacks: (u32, u32)) -> Geometry {
    if zones <= 1 {
        return Geometry::Leaf(start_id);
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

    let mut left_columns = Vec::new();
    let mut right_columns = Vec::new();
    let mut next_id = start_id + 1;

    for (i, count) in counts.into_iter().enumerate() {
        let column = build_stack(next_id, count);

        if (i as u32) < before_stacks {
            left_columns.push(column);
        } else {
            right_columns.push(column);
        }

        next_id += count;
    }

    let mut columns = Vec::new();
    columns.extend(left_columns);
    columns.push(Geometry::Leaf(start_id)); // primary
    columns.extend(right_columns);

    fold_horizontal(columns)
}

/// Build a single column of rows (vertical stacking)
/// 
/// ## Arguments
/// * `start_id` - The id of the first window
/// * `count` - The number of windows
/// 
/// ## Returns
/// * a `Geometry` - The root of the vstack layout
fn build_stack(start_id: u32, count: u32) -> Geometry {
    if count == 1 {
        return Geometry::Leaf(start_id);
    }

    let mut items = Vec::new();
    for offset in 0..count {
        items.push(Geometry::Leaf(start_id + offset));
    }

    fold_vertical(items)
}

impl Layout {
    pub fn vstack(start_id: u32, zones: u32) -> Self {
        Self {
            kind: LayoutKind::VStack,
            start_id,
            zones,
            stacks: (0, 1),
            external_padding: 0,
            internal_padding: 0,
        }
    }
}