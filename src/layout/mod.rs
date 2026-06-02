pub mod bsp;
pub mod rows;
pub mod columns;
pub mod vstack;
pub mod hstack;
pub mod grid;
pub mod fold;   
pub mod stacks;

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

pub enum Geometry {
    Leaf(
        u32
    ),
    Split {
        direction: Direction,
        ratio: f32,
        left: Box<Geometry>,
        right: Box<Geometry>,
    },
}

impl Geometry {
    pub fn compute(&self, screen: &Rect) -> Vec<(u32, Rect)> {
        let mut out: Vec<(u32, Rect)> = Vec::new();
        self.compute_into(*screen, &mut out);
        out
    }

    fn compute_into(&self, screen: Rect, out: &mut Vec<(u32, Rect)>) {
        match self {
            Geometry::Leaf(id) => out.push((*id, screen)),
            Geometry::Split {
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

pub enum LayoutKind {
    Rows,
    Columns,
    VStack,
    HStack,
    Grid,
    BSP,
}

pub struct Layout {
    kind: LayoutKind,
    start_id: u32,
    zones: u32,
    stacks: (u32, u32),
    external_padding: u32,
    internal_padding: u32
}

impl Layout {
    pub fn add_stacks(&mut self, before: u32, after: u32) {
        match self.kind {
            LayoutKind::Rows | LayoutKind::Columns => {
                self.stacks.1 += after;
            }
            LayoutKind::VStack | LayoutKind::HStack => {
                self.stacks.0 += before;
                self.stacks.1 += after;
            }
            LayoutKind::Grid | LayoutKind::BSP => {
                // ignore
            }
        }
    }

    pub fn remove_stacks(&mut self, before: u32, after: u32) {
        match self.kind {
            LayoutKind::Rows | LayoutKind::Columns => {
                self.stacks.1 -= after;
            }
            LayoutKind::VStack | LayoutKind::HStack => {
                self.stacks.0 -= before;
                self.stacks.1 -= after;
            }
            LayoutKind::Grid | LayoutKind::BSP => {
                // ignore
            }
        }
    }

    pub fn compile(&self) -> Geometry {
        match self.kind {
            LayoutKind::Rows => Geometry::rows(self.start_id, self.zones, self.stacks),
            LayoutKind::Columns => Geometry::columns(self.start_id, self.zones, self.stacks),
            LayoutKind::VStack => Geometry::vstack(self.start_id, self.zones, self.stacks),
            LayoutKind::HStack => Geometry::hstack(self.start_id, self.zones, self.stacks),
            LayoutKind::Grid => Geometry::grid(self.start_id, self.zones, self.stacks),
            LayoutKind::BSP => Geometry::bsp(self.start_id, self.zones, self.stacks),
        }
    }
}