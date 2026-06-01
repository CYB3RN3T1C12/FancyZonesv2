use crate::layout::{LayoutTree, stacks::generate_stack_pattern, fold::{fold_horizontal, fold_vertical}};

impl LayoutTree {
    /// Build a columns layout
    /// 
    /// ## Arguments
    /// * `start_id` - The id of the first window
    /// * `count` - The number of windows
    /// * `stacks` - The number of stacks
    /// 
    /// ## Returns
    /// * a `LayoutTree` - The root of the columns layout
    pub fn columns(
        start_id: u32,
        window_count: u32,
        stacks: (u32, u32),
    ) -> LayoutTree {
        build_columns(start_id, window_count, stacks)
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
/// * a `LayoutTree` - The root of the columns layout
fn build_columns(start_id: u32, zones: u32, stacks: (u32, u32)) -> LayoutTree {
    if zones == 0 {
        return LayoutTree::Leaf(start_id);
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
/// * a `LayoutTree` - The root of the row
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

pub struct LayoutColumns{
    start_id: u32,
    zones: u32,
    stacks: u32, // number of stacks AFTER the primary
}

impl LayoutColumns{
    /// Create a new columns layout
    /// 
    /// ## Arguments
    /// * `start_id` - The id of the first window
    /// * `zones` - The number of zones
    /// 
    /// ## Returns
    /// * a `LayoutColumns` - The columns layout
    pub fn new(start_id: u32, zones: u32) -> Self{
        Self{
            start_id,
            zones,
            stacks: 1, // default stack count
        }
    }
    
    /// Add stacks
    /// 
    /// ## Arguments
    /// * `stacks` - The number of stacks
    pub fn add_stacks(&mut self, stacks: u32) {
        self.stacks += stacks;
    }

    /// Remove stacks
    /// 
    /// ## Arguments
    /// * `stacks` - The number of stacks
    pub fn remove_stacks(&mut self, stacks: u32) {
        self.stacks = self.stacks.saturating_sub(stacks);
    }

    /// Compile the columns layout
    /// 
    /// ## Returns
    /// * a `LayoutTree` - The root of the columns layout
    pub fn compile(&self) -> LayoutTree {
        LayoutTree::columns(self.start_id, self.zones, (0, self.stacks))
    }
}
