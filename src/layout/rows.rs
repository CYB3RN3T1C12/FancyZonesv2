use crate::layout::{LayoutTree, stacks::generate_stack_pattern, fold::{fold_horizontal, fold_vertical}};

impl LayoutTree {
    pub fn rows(
        start_id: u32,
        window_count: u32,
        stacks: (u32, u32),
    ) -> LayoutTree {
        build_rows(start_id, window_count, stacks)
    }
}

fn build_rows(start_id: u32, zones: u32, stacks: (u32, u32)) -> LayoutTree {
    if zones == 0 {
        return LayoutTree::Leaf(start_id);
    }

    // NEW: unified stack pattern generator
    let counts = generate_stack_pattern(zones, stacks);

    let mut columns: Vec<LayoutTree> = Vec::new();
    let mut next_id = start_id;

    for count in counts {
        let column = build_stack(next_id, count);
        columns.push(column);
        next_id += count;
    }

    fold_horizontal(columns)
}

fn build_stack(start_id: u32, count: u32) -> LayoutTree {
    if count == 1 {
        return LayoutTree::Leaf(start_id);
    }

    let mut items = Vec::new();
    for offset in 0..count {
        items.push(LayoutTree::Leaf(start_id + offset));
    }

    fold_vertical(items)
}

pub struct LayoutRows {
    start_id: u32,
    zones: u32,
    stacks: u32, // number of stacks AFTER the primary
}

impl LayoutRows {
    pub fn new(start_id: u32, zones: u32) -> Self {
        Self {
            start_id,
            zones,
            stacks: 1, // default stack count   
        }
    }

    pub fn add_stacks(&mut self, stacks: u32) {
        self.stacks += stacks;
    }

    pub fn remove_stacks(&mut self, stacks: u32) {
        self.stacks = self.stacks.saturating_sub(stacks);
    }

    pub fn compile(&self) -> LayoutTree {
        LayoutTree::rows(self.start_id, self.zones, (0, self.stacks))
    }
}
