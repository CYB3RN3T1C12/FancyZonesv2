use crate::layout::{LayoutTree, Direction, Rect};

impl LayoutTree {
    pub fn hstack(start_id: u32, window_count: u32, screen: &Rect) -> LayoutTree {
        build_hstack(start_id, window_count, screen.width, screen.height)
    }
}

fn build_hstack(start_id: u32, count: u32, width: u32, height: u32) -> LayoutTree {
    if count == 1 {
        return LayoutTree::Leaf(start_id);
    }

    // Master gets top half
    let master_height = height / 2;
    let stack_height = height - master_height;

    LayoutTree::Split {
        direction: Direction::Vertical, // top/bottom
        ratio: master_height as f32 / height as f32,

        left: Box::new(LayoutTree::Leaf(start_id)),

        // Stack is just columns of remaining windows
        right: Box::new(build_columns(start_id + 1, count, width, stack_height)),
    }
}

// Local helper: identical to your columns layout but height is fixed
fn build_columns(start_id: u32, total: u32, width: u32, height: u32) -> LayoutTree {
    let remaining = total - start_id;

    if remaining == 1 {
        return LayoutTree::Leaf(start_id);
    }

    let left_width = width / remaining;
    let right_width = width - left_width;
    let ratio = left_width as f32 / width as f32;

    LayoutTree::Split {
        direction: Direction::Horizontal, // left/right
        ratio,
        left: Box::new(LayoutTree::Leaf(start_id)),
        right: Box::new(build_columns(start_id + 1, total, right_width, height)),
    }
}