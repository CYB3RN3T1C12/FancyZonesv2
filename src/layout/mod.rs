pub mod bsp;
pub mod rows;
pub mod columns;
pub mod vstack;
pub mod hstack;
pub mod grid;
pub mod pattern;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub enum Direction {
    Horizontal,
    Vertical,
}

pub enum LayoutTree {
    Leaf(
        u32
    ),
    Split {
        direction: Direction,
        ratio: f32,
        left: Box<LayoutTree>,
        right: Box<LayoutTree>,
    },
}

impl LayoutTree {
    pub fn compute(&self, screen: Rect) -> Vec<(u32, Rect)> {
        let mut out: Vec<(u32, Rect)> = Vec::new();
        self.compute_into(screen, &mut out);
        out
    }

    fn compute_into(&self, screen: Rect, out: &mut Vec<(u32, Rect)>) {
        match self {
            LayoutTree::Leaf(id) => out.push((*id, screen)),
            LayoutTree::Split {
                direction,
                ratio,
                left,
                right,
            } => match direction {
                Direction::Vertical => {
                    let top_h: u32 = (screen.height as f32 * ratio) as u32;
                    let bottom_h: u32 = screen.height - top_h;

                    let top_rect: Rect = Rect {
                        height: top_h,
                        ..screen
                    };
                    let bottom_rect: Rect = Rect {
                        y: screen.y + top_h,
                        height: bottom_h,
                        ..screen
                    };

                    left.compute_into(top_rect, out);
                    right.compute_into(bottom_rect, out);
                }
                Direction::Horizontal => {
                    let left_w: u32 = (screen.width as f32 * ratio) as u32;
                    let right_w: u32 = screen.width - left_w;

                    let left_rect: Rect = Rect {
                        width: left_w,
                        ..screen
                    };
                    let right_rect: Rect = Rect {
                        x: screen.x + left_w,
                        width: right_w,
                        ..screen
                    };

                    left.compute_into(left_rect, out);
                    right.compute_into(right_rect, out);
                }
            },
        }
    }
}
