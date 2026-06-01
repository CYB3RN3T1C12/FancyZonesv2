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
    pub fn compute(&self, screen: &Rect) -> Vec<(u32, Rect)> {
        let mut out: Vec<(u32, Rect)> = Vec::new();
        self.compute_into(*screen, &mut out);
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

pub enum LayoutType {
    Rows(rows::LayoutRows),
    Columns(columns::LayoutColumns),
    VStack(vstack::LayoutVStack),
    HStack(hstack::LayoutHStack),
    Grid(grid::LayoutGrid),
    BSP(bsp::LayoutBSP),
}

impl LayoutType {
    pub fn rows(start_id: u32, zones: u32) -> Self {
        LayoutType::Rows(rows::LayoutRows::new(start_id, zones))
    }

    pub fn columns(start_id: u32, zones: u32) -> Self {
        LayoutType::Columns(columns::LayoutColumns::new(start_id, zones))
    }

    pub fn hstack(start_id: u32, zones: u32) -> Self {
        LayoutType::HStack(hstack::LayoutHStack::new(start_id, zones))
    }

    pub fn vstack(start_id: u32, zones: u32) -> Self {
        LayoutType::VStack(vstack::LayoutVStack::new(start_id, zones))
    }

    pub fn grid(start_id: u32, zones: u32) -> Self {
        LayoutType::Grid(grid::LayoutGrid::new(start_id, zones))
    }

    pub fn bsp(start_id: u32, zones: u32) -> Self {
        LayoutType::BSP(bsp::LayoutBSP::new(start_id, zones))
    }

    pub fn add_stacks(&mut self, before: u32, after: u32) {
        match self {
            LayoutType::VStack(v) => v.add_stacks((before, after)),
            LayoutType::HStack(h) => h.add_stacks((before, after)),
            LayoutType::Rows(r)   => r.add_stacks(after),   // rows only use one value
            LayoutType::Columns(c)=> c.add_stacks(after),
            _ => {} // Grid and BSP ignore stack changes
        }
    }

    pub fn compile(&self) -> LayoutTree {
        match self {
            LayoutType::VStack(v) => v.compile(),
            LayoutType::HStack(h) => h.compile(),
            LayoutType::Rows(r)   => r.compile(),
            LayoutType::Columns(c)=> c.compile(),
            LayoutType::Grid(g)   => g.compile(),
            LayoutType::BSP(b)    => b.compile(),
        }
    }
}
