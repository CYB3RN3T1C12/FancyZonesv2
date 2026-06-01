use crate::layout::{LayoutTree, stacks::generate_stack_pattern, fold::{fold_horizontal, fold_vertical}};

impl LayoutTree {
    /// Build a vstack layout
    /// 
    /// ## Arguments
    /// * `start_id` - The id of the first window
    /// * `count` - The number of windows
    /// * `stacks` - The number of stacks
    /// 
    /// ## Returns
    /// * a `LayoutTree` - The root of the vstack layout
    pub fn vstack(
        start_id: u32,
        window_count: u32,
        stacks: (u32, u32),
    ) -> LayoutTree {
        build_vstack(start_id, window_count, stacks)
    }
}

/// Build a vstack layout
/// 
/// ## Arguments
/// * `start_id` - The id of the first window
/// * `window_count` - The number of windows
/// * `stacks` - The number of stacks
/// 
/// ## Returns
/// * a `LayoutTree` - The root of the vstack layout
fn build_vstack(start_id: u32, window_count: u32, stacks: (u32, u32)) -> LayoutTree {
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
    columns.push(LayoutTree::Leaf(start_id)); // primary
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
/// * a `LayoutTree` - The root of the vstack layout
fn build_stack(start_id: u32, count: u32) -> LayoutTree {
    if count == 1 {
        return LayoutTree::Leaf(start_id);
    }

    let mut items = Vec::new();
    for offset in 0..count {
        items.push(LayoutTree::Leaf(start_id + offset));
    }

    fold_vertical(items)
}

pub struct LayoutVStack{
    start_id: u32,
    zones: u32,
    stacks: (u32, u32),
}

impl LayoutVStack {
    /// Create a new LayoutVStack
    /// 
    /// ## Arguments
    /// * `start_id` - The id of the first window
    /// * `zones` - The number of zones
    /// 
    /// ## Returns
    /// * a `LayoutVStack` - The vstack layout
    pub fn new(start_id: u32, zones: u32) -> Self {
        Self {
            start_id,
            zones,
            stacks: (0, 1), // default
        }
    }

    /// Add stacks
    /// 
    /// ## Arguments
    /// * `stacks` - The number of stacks
    pub fn add_stacks(&mut self, stacks: (u32, u32)) {
        self.stacks.0 += stacks.0;
        self.stacks.1 += stacks.1;
    }

    /// Remove stacks
    /// 
    /// ## Arguments
    /// * `stacks` - The number of stacks
    pub fn remove_stacks(&mut self, stacks: (u32, u32)) {
        self.stacks.0 = self.stacks.0.saturating_sub(stacks.0);
        self.stacks.1 = self.stacks.1.saturating_sub(stacks.1);
    }

    /// Compile the vstack layout
    /// 
    /// ## Returns
    /// * a `LayoutTree` - The root of the vstack layout
    pub fn compile(&self) -> LayoutTree {
        LayoutTree::vstack(self.start_id, self.zones, self.stacks)
    }
}
