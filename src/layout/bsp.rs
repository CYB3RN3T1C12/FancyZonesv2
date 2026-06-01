use crate::layout::{Direction, LayoutTree};

impl LayoutTree {
    pub fn bsp(start_id: u32, window_count: u32, _stacks: (u32, u32)) -> LayoutTree {
        build_bsp(start_id, window_count, true)
    }
}

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
    pub fn new(start_id: u32, zones: u32) -> LayoutBSP{
        LayoutBSP{start_id, zones}
    }

    pub fn compile(&self) -> LayoutTree {
        LayoutTree::bsp(self.start_id, self.zones, (0, 1))
    }
}