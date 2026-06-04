use crate::layout::{Direction, Tree, LayoutKind, Layout};

impl Tree {
    /// Build a BSP layout
    /// 
    /// ## Arguments
    /// * `start_id` - The id of the first window
    /// * `count` - The number of windows
    /// * `stacks` - The number of stacks
    /// 
    /// ## Returns
    /// * a `Tree` - The root of the BSP layout
    pub fn bsp(start_id: u32, zones: u32, _stacks: (u32, u32)) -> Tree {
        build_bsp(start_id, zones, true)
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
/// * a `Tree` - The root of the BSP layout
fn build_bsp(start_id: u32, count: u32, split_left_right: bool) -> Tree {
    if count == 1 {
        return Tree::Leaf(start_id);
    }

    let direction: Direction = if split_left_right {
        Direction::Horizontal
    } else {
        Direction::Vertical
    };

    Tree::Branch {
        direction,
        ratio: 0.5,
        left: Box::new(Tree::Leaf(start_id)),
        right: Box::new(build_bsp(start_id + 1, count - 1, !split_left_right)),
    }
}

impl Layout {
    pub fn bsp(start_id: u32, zones: u32) -> Self {
        Self {
            kind: LayoutKind::BSP,
            start_id,
            zones,
            stacks: (0, 0), // bsp ignores stacks
            external_padding: 0,
            internal_padding: 0,
        }
    }
}