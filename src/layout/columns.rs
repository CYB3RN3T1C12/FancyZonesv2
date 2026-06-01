use crate::layout::{LayoutTree, stacks::generate_stack_pattern, fold::{fold_horizontal, fold_vertical}};

impl LayoutTree {
    pub fn columns(
        start_id: u32,
        window_count: u32,
        stacks: (u32, u32),
    ) -> LayoutTree {
        build_columns(start_id, window_count, stacks)
    }
}

fn build_columns(start_id: u32, zones: u32, stacks: (u32, u32)) -> LayoutTree {
    if zones == 0 {
        return LayoutTree::Leaf(start_id);
    }

    // NEW: unified stack pattern generator
    let counts = generate_stack_pattern(zones, stacks);

    let mut rows = Vec::new();
    let mut next_id = start_id;

    for count in counts {
        let row = build_stack(next_id, count);
        rows.push(row);
        next_id += count;
    }

    fold_vertical(rows)
}

fn build_stack(start_id: u32, count: u32) -> LayoutTree {
    if count == 1 {
        return LayoutTree::Leaf(start_id);
    }

    let mut items = Vec::new();
    for offset in 0..count {
        items.push(LayoutTree::Leaf(start_id + offset));
    }

    fold_horizontal(items)
}

pub struct LayoutColumns{
    start_id: u32,
    zones: u32,
    stacks: u32, // number of stacks AFTER the primary
}

impl LayoutColumns{
    pub fn new(start_id: u32, zones: u32) -> Self{
        Self{
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
        LayoutTree::columns(self.start_id, self.zones, (0, self.stacks))
    }
}
