use crate::layout::{LayoutTree, Direction, Rect};

impl LayoutTree {
    pub fn bsp(start_id: u32, window_count: u32, screen: &Rect) -> LayoutTree {
        build_bsp(start_id, window_count, screen.width, screen.height)
    }
}

fn build_bsp(start_id: u32, count: u32, width: u32, height: u32) -> LayoutTree {
    if count == 1 {
        return LayoutTree::Leaf(start_id);
    }

    // Split along longest axis
    let split_left_right: bool = width >= height;

    if split_left_right {
        LayoutTree::Split {
            direction: Direction::Horizontal,
            ratio: 0.5,

            // Master pane
            left: Box::new(LayoutTree::Leaf(start_id)),

            // Remaining windows recurse
            right: Box::new(build_bsp(
                start_id + 1,
                count - 1,
                width / 2,
                height,
            )),
        }
    } else {
        LayoutTree::Split {
            direction: Direction::Vertical,
            ratio: 0.5,

            // Master pane
            left: Box::new(LayoutTree::Leaf(start_id)),

            // Remaining windows recurse
            right: Box::new(build_bsp(
                start_id + 1,
                count - 1,
                width,
                height / 2,
            )),
        }
    }
}