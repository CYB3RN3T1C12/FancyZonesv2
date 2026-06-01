use crate::layout::{Direction, LayoutTree, stacks::generate_stack_pattern, fold::fold_horizontal};

/// Generate a grid pattern
/// 
/// ## Arguments
/// * `n` - The number of windows
/// 
/// ## Returns
/// * a `Vec<u32>` - The grid pattern
fn generate_grid_pattern(zones: u32) -> Vec<u32> {
    let k = (zones as f32).sqrt() as u32; // floor(sqrt(n))
    let stacks = if k*k == zones { k } else { k + 1 };

    generate_stack_pattern(zones, (0, stacks))
}

impl LayoutTree {
    pub fn grid(start_id: u32, window_count: u32, _stacks: (u32, u32)) -> LayoutTree {
        let pattern = generate_grid_pattern(window_count);
        let (layout, _) = build_rows(start_id, &pattern);
        layout
    }
}

/// Build a rows layout
/// 
/// ## Arguments
/// * `start_id` - The id of the first window
/// * `rows` - The number of windows in each row
/// 
/// ## Returns
/// * a `LayoutTree` - The root of the rows layout
fn build_rows(start_id: u32, rows: &[u32]) -> (LayoutTree, u32) {
    if rows.len() == 1 {
        let layout = build_stack(start_id, rows[0]);
        return (layout, start_id + rows[0]);
    }

    let ratio = 1.0 / rows.len() as f32;

    let top = build_stack(start_id, rows[0]);
    let (rest, next_id) = build_rows(start_id + rows[0], &rows[1..]);

    (
        LayoutTree::Split {
            direction: Direction::Vertical,
            ratio,
            left: Box::new(top),
            right: Box::new(rest),
        },
        next_id,
    )
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

pub struct LayoutGrid{
    start_id: u32,
    zones: u32
}

impl LayoutGrid{
    /// Create a new LayoutGrid
    /// 
    /// ## Arguments
    /// * `start_id` - The id of the first window
    /// * `zones` - The number of zones
    /// 
    /// ## Returns
    /// * a `LayoutGrid` - The grid layout
    pub fn new(start_id: u32, zones: u32) -> Self {
        Self {
            start_id,
            zones
        }
    }

    /// Compile the grid layout
    /// 
    /// ## Returns
    /// * a `LayoutTree` - The root of the grid layout
    pub fn compile(&self) -> LayoutTree {
        LayoutTree::grid(self.start_id, self.zones, (0, 1))
    }
}