use crate::layout::{LayoutTree, Direction, Rect};

impl LayoutTree {
    pub fn columns(start_id: u32, window_count: u32, screen: &Rect) -> LayoutTree {
        build_columns(start_id, window_count, screen.width, screen.height)
    }
}

fn build_columns(start_id: u32, count: u32, width: u32, height: u32) -> LayoutTree {
    if count == 1 {
        return LayoutTree::Leaf(start_id);
    }

    let left_width: u32 = width / count;
    let right_width: u32 = width - left_width;
    let ratio: f32 = left_width as f32 / width as f32;

    LayoutTree::Split {
        direction: Direction::Horizontal,
        ratio,
        left: Box::new(LayoutTree::Leaf(start_id)),
        right: Box::new(build_columns(start_id + 1, count - 1, right_width, height)),
    }
}
