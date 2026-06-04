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

pub struct Geometry {
    pub tree: Tree,
    pub internal_padding: u32,
    pub external_padding: u32,
}

impl Geometry {
    pub fn build(tree: Tree, internal_padding: u32, external_padding: u32) -> Self {
        Self {
            tree,
            internal_padding,
            external_padding,
        }
    }
}

pub enum Direction {
    Horizontal,
    Vertical,
}

pub enum Tree {
    Leaf(
        u32
    ),
    Branch { // Split
        direction: Direction,
        ratio: f32,
        left: Box<Tree>,
        right: Box<Tree>,
    },
}

impl Geometry {
    pub fn compute(&self, screen: &Rect) -> Vec<(u32, Rect)> {
        let ext = self.external_padding;

        // Apply external padding to the whole layout
        let padded: Rect = Rect {
            x: screen.x + ext,
            y: screen.y + ext,
            width: screen.width.saturating_sub(ext * 2),
            height: screen.height.saturating_sub(ext * 2),
        };

        let mut out = Vec::new();
        self.tree.compute_into(padded, &mut out, self.internal_padding);
        out
    }
}

impl Tree {
    pub fn compute_into(
        &self,
        screen: Rect,
        out: &mut Vec<(u32, Rect)>,
        internal: u32,
    ) {
        match self {
            Tree::Leaf(id) => {
                out.push((*id, screen));
            }

            Tree::Branch {
                direction,
                ratio,
                left,
                right,
            } => {
                match direction {
                    Direction::Vertical => {
                        let total_vertical_gaps = 1
                            + left.vertical_gap_count()
                            + right.vertical_gap_count();
                        let available_height = screen
                            .height
                            .saturating_sub(internal.saturating_mul(total_vertical_gaps));
                        let split = ((available_height as f32) * ratio).round() as u32;

                        let left_height = split
                            .saturating_add(internal.saturating_mul(left.vertical_gap_count()));
                        let right_height = available_height
                            .saturating_sub(split)
                            .saturating_add(internal.saturating_mul(right.vertical_gap_count()));

                        let top_rect = Rect {
                            x: screen.x,
                            y: screen.y,
                            width: screen.width,
                            height: left_height,
                        };

                        let bottom_rect = Rect {
                            x: screen.x,
                            y: screen.y + left_height + internal,
                            width: screen.width,
                            height: right_height,
                        };

                        left.compute_into(top_rect, out, internal);
                        right.compute_into(bottom_rect, out, internal);
                    }

                    Direction::Horizontal => {
                        let total_horizontal_gaps = 1
                            + left.horizontal_gap_count()
                            + right.horizontal_gap_count();
                        let available_width = screen
                            .width
                            .saturating_sub(internal.saturating_mul(total_horizontal_gaps));
                        let split = ((available_width as f32) * ratio).round() as u32;

                        let left_width = split
                            .saturating_add(internal.saturating_mul(left.horizontal_gap_count()));
                        let right_width = available_width
                            .saturating_sub(split)
                            .saturating_add(internal.saturating_mul(right.horizontal_gap_count()));

                        let left_rect = Rect {
                            x: screen.x,
                            y: screen.y,
                            width: left_width,
                            height: screen.height,
                        };

                        let right_rect = Rect {
                            x: screen.x + left_width + internal,
                            y: screen.y,
                            width: right_width,
                            height: screen.height,
                        };

                        left.compute_into(left_rect, out, internal);
                        right.compute_into(right_rect, out, internal);
                    }
                }
            }
        }
    }

    fn horizontal_gap_count(&self) -> u32 {
        match self {
            Tree::Leaf(_) => 0,
            Tree::Branch {
                direction,
                left,
                right,
                ..
            } => {
                let count = left.horizontal_gap_count() + right.horizontal_gap_count();
                if let Direction::Horizontal = direction {
                    count + 1
                } else {
                    count
                }
            }
        }
    }

    fn vertical_gap_count(&self) -> u32 {
        match self {
            Tree::Leaf(_) => 0,
            Tree::Branch {
                direction,
                left,
                right,
                ..
            } => {
                let count = left.vertical_gap_count() + right.vertical_gap_count();
                if let Direction::Vertical = direction {
                    count + 1
                } else {
                    count
                }
            }
        }
    }
}


#[derive(Debug)]
pub enum LayoutKind {
    Rows,
    Columns,
    VStack,
    HStack,
    Grid,
    BSP,
}

#[derive(Debug)]
pub struct Layout {
    kind: LayoutKind,
    start_id: u32,
    zones: u32,
    stacks: (u32, u32),
    internal_padding: u32,
    external_padding: u32,
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

    pub fn set_padding(&mut self, internal: u32, external: u32) {
        self.internal_padding = internal;
        self.external_padding = external;
    }

    pub fn compile(&self) -> Geometry {
        let tree = match self.kind {
            LayoutKind::Rows => Tree::rows(self.start_id, self.zones, self.stacks),
            LayoutKind::Columns => Tree::columns(self.start_id, self.zones, self.stacks),
            LayoutKind::VStack => Tree::vstack(self.start_id, self.zones, self.stacks),
            LayoutKind::HStack => Tree::hstack(self.start_id, self.zones, self.stacks),
            LayoutKind::Grid => Tree::grid(self.start_id, self.zones, self.stacks),
            LayoutKind::BSP => Tree::bsp(self.start_id, self.zones, self.stacks),
        };

        Geometry::build(tree, self.internal_padding, self.external_padding)
    }
}