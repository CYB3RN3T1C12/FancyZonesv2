use crate::layout::{LayoutTree, stacks::generate_stack_pattern, fold::{fold_horizontal, fold_vertical}};

impl LayoutTree {
    pub fn hstack(
        start_id: u32,
        window_count: u32,
        stacks: (u32, u32),
    ) -> LayoutTree {
        build_hstack(start_id, window_count, stacks)
    }
}

fn build_hstack(start_id: u32, window_count: u32, stacks: (u32, u32)) -> LayoutTree {
    if window_count <= 1 {
        return LayoutTree::Leaf(start_id);
    }

    let requested_before = stacks.0;
    let requested_after = stacks.1;
    let mut total_stacks = requested_before + requested_after;

    if total_stacks == 0 {
        total_stacks = 1;
    }
    if total_stacks > window_count - 1 {
        total_stacks = window_count - 1;
    }

    let before_stacks = requested_before.min(total_stacks);

    // NEW: unified stack pattern generator
    let counts = generate_stack_pattern(window_count - 1, (before_stacks, total_stacks - before_stacks));

    let mut top_rows = Vec::new();
    let mut bottom_rows = Vec::new();
    let mut next_id = start_id + 1;

    for (i, count) in counts.into_iter().enumerate() {
        let row = build_stack(next_id, count);

        if (i as u32) < before_stacks {
            top_rows.push(row);
        } else {
            bottom_rows.push(row);
        }

        next_id += count;
    }

    let mut rows = Vec::new();
    rows.extend(top_rows);
    rows.push(LayoutTree::Leaf(start_id)); // primary
    rows.extend(bottom_rows);

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

pub struct LayoutHStack{
    start_id: u32,
    zones: u32,
    stacks: (u32, u32),
}

impl LayoutHStack {
    pub fn new(start_id: u32, zones: u32) -> Self {
        Self {
            start_id,
            zones,
            stacks: (0, 1),
        }
    }

    pub fn add_stacks(&mut self, stacks: (u32, u32)) {
        self.stacks.0 += stacks.0;
        self.stacks.1 += stacks.1;
    }

    pub fn remove_stacks(&mut self, stacks: (u32, u32)) {
        self.stacks.0 = self.stacks.0.saturating_sub(stacks.0);
        self.stacks.1 = self.stacks.1.saturating_sub(stacks.1);
    }

    pub fn compile(&self) -> LayoutTree {
        LayoutTree::hstack(self.start_id, self.zones, self.stacks)
    }
}
