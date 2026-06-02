use crate::layout::{Geometry, LayoutKind, Layout, stacks::generate_stack_pattern, fold::{fold_horizontal, fold_vertical}};

impl Geometry {
    /// Build a columns layout
    /// 
    /// ## Arguments
    /// * `start_id` - The id of the first window
    /// * `count` - The number of windows
    /// * `stacks` - The number of stacks
    /// 
    /// ## Returns
    /// * a `Geometry` - The root of the columns layout
    pub fn columns(
        start_id: u32,
        zones: u32,
        stacks: (u32, u32),
    ) -> Geometry {
        build_columns(start_id, zones, stacks)
    }
}

/// Build a columns layout
/// 
/// ## Arguments
/// * `start_id` - The id of the first window
/// * `zones` - The number of zones
/// * `stacks` - The number of stacks
/// 
/// ## Returns
/// * a `Geometry` - The root of the columns layout
fn build_columns(start_id: u32, zones: u32, stacks: (u32, u32)) -> Geometry {
    if zones == 0 {
        return Geometry::Leaf(start_id);
    }

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

/// Build a single row of columns (horizontal stacking)
/// 
/// ## Arguments
/// * `start_id` - The id of the first window
/// * `count` - The number of windows
/// 
/// ## Returns
/// * a `Geometry` - The root of the row
fn build_stack(start_id: u32, count: u32) -> Geometry {
    if count == 1 {
        return Geometry::Leaf(start_id);
    }

    let mut items = Vec::new();
    for offset in 0..count {
        items.push(Geometry::Leaf(start_id + offset));
    }

    fold_horizontal(items)
}

impl Layout {
    pub fn columns(start_id: u32, zones: u32) -> Self {
        Self {
            kind: LayoutKind::Columns,
            start_id,
            zones,
            stacks: (0, 1),
            external_padding: 0,
            internal_padding: 0,
        }
    }
}