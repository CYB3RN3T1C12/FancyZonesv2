use crate::layout::{Tree, LayoutKind, Layout, stacks::generate_stack_pattern, fold::{fold_horizontal, fold_vertical}};

impl Tree {
    /// Build a rows layout
    /// 
    /// ## Arguments
    /// * `start_id` - The id of the first window
    /// * `count` - The number of windows
    /// * `stacks` - The number of stacks
    /// 
    /// ## Returns
    /// * a `Tree` - The root of the rows layout
    pub fn rows(
        start_id: u32,
        zones: u32,
        stacks: (u32, u32),
    ) -> Tree {
        build_rows(start_id, zones, stacks)
    }
}

/// Build a rows layout
/// 
/// ## Arguments
/// * `start_id` - The id of the first window
/// * `zones` - The number of zones
/// * `stacks` - The number of stacks
/// 
/// ## Returns
/// * a `Tree` - The root of the rows layout
fn build_rows(start_id: u32, zones: u32, stacks: (u32, u32)) -> Tree {
    if zones == 0 {
        return Tree::Leaf(start_id);
    }

    let counts = generate_stack_pattern(zones, stacks);

    let mut columns: Vec<Tree> = Vec::new();
    let mut next_id = start_id;

    for count in counts {
        let column = build_stack(next_id, count);
        columns.push(column);
        next_id += count;
    }

    fold_horizontal(columns)
}

/// Build a single column of rows (vertical stacking)
/// 
/// ## Arguments
/// * `start_id` - The id of the first window
/// * `count` - The number of windows
/// 
/// ## Returns
/// * a `Tree` - The root of the rows layout
fn build_stack(start_id: u32, count: u32) -> Tree {
    if count == 1 {
        return Tree::Leaf(start_id);
    }

    let mut items = Vec::new();
    for offset in 0..count {
        items.push(Tree::Leaf(start_id + offset));
    }

    fold_vertical(items)
}

impl Layout {    
    pub fn rows(start_id: u32, zones: u32) -> Self {
        Self {
            kind: LayoutKind::Rows,
            start_id,
            zones,
            stacks: (0, 1),
            external_padding: 0,
            internal_padding: 0,
        }
    }
}