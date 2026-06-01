use crate::layout::{Direction, LayoutTree};

impl LayoutTree {
    /// Build a BSP layout
    /// 
    /// ## Arguments
    /// * `start_id` - The id of the first window
    /// * `count` - The number of windows
    /// * `stacks` - The number of stacks
    /// 
    /// ## Returns
    /// * a `LayoutTree` - The root of the BSP layout
    pub fn bsp(start_id: u32, window_count: u32, _stacks: (u32, u32)) -> LayoutTree {
        build_bsp(start_id, window_count, true)
    }
}

/// Build a BSP layout
/// 
/// ## Arguments
/// * `start_id` - The id of the first window
/// * `count` - The number of windows
/// * `split_left_right` - Whether to split the tree left to right
/// 
/// ## Returns
/// * a `LayoutTree` - The root of the BSP layout
fn build_bsp(start_id: u32, count: u32, split_left_right: bool) -> LayoutTree {
    if count == 1 {
        return LayoutTree::Leaf(start_id);
    }

    let direction: Direction = if split_left_right {
        Direction::Horizontal
    } else {
        Direction::Vertical
    };

    LayoutTree::Split {
        direction,
        ratio: 0.5,
        left: Box::new(LayoutTree::Leaf(start_id)),
        right: Box::new(build_bsp(start_id + 1, count - 1, !split_left_right)),
    }
}

pub struct LayoutBSP{
    start_id: u32,
    zones: u32
}

impl LayoutBSP{
    /// Create a new BSP layout
    /// 
    /// ## Arguments
    /// * `start_id` - The id of the first window
    /// * `zones` - The number of zones
    /// 
    /// ## Returns
    /// * a `LayoutBSP` - The BSP layout
    pub fn new(start_id: u32, zones: u32) -> LayoutBSP{
        LayoutBSP{start_id, zones}
    }

    /// Compile the BSP layout
    /// 
    /// ## Returns
    /// * a `LayoutTree` - The root of the BSP layout
    pub fn compile(&self) -> LayoutTree {
        LayoutTree::bsp(self.start_id, self.zones, (0, 1))
    }
}