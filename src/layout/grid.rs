use crate::layout::{Direction, LayoutTree};

/// Grid-specific pattern generator.
/// Attempts to approximate a square grid by:
/// - starting with floor(sqrt(n)) rows
/// - distributing extras bottom-up cyclically
fn generate_grid_pattern(n: u32) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }

    let k = (n as f32).sqrt() as u32; // floor(sqrt(n))
    let mut rows = vec![k; k as usize];
    let mut extras = n - (k * k);

    // Distribute extras bottom-up cyclically
    let mut idx: i32 = rows.len() as i32 - 1;
    while extras > 0 {
        rows[idx as usize] += 1;
        extras -= 1;

        idx -= 1;
        if idx < 0 {
            idx = rows.len() as i32 - 1;
        }
    }

    rows
}

impl LayoutTree {
    pub fn grid(start_id: u32, window_count: u32, _stacks: (u32, u32)) -> LayoutTree {
        let pattern = generate_grid_pattern(window_count);
        let (layout, _) = build_rows(start_id, &pattern);
        layout
    }
}

/// Build all rows of the grid (vertical stacking)
fn build_rows(start_id: u32, rows: &[u32]) -> (LayoutTree, u32) {
    if rows.len() == 1 {
        let layout = build_stack(start_id, rows[0]);
        return (layout, start_id + rows[0]);
    }

    let ratio = 1.0 / rows.len() as f32;

    let top = build_stack(start_id, rows[0]);
    let (rest, next_id) = build_rows(start_id + rows[0], &rows[1..]);

    (
        LayoutTree::Split {
            direction: Direction::Vertical,
            ratio,
            left: Box::new(top),
            right: Box::new(rest),
        },
        next_id,
    )
}

/// Build a single row of the grid (horizontal stacking)
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

/// Fold a row horizontally into a BSP tree
fn fold_horizontal(mut items: Vec<LayoutTree>) -> LayoutTree {
    if items.len() == 1 {
        return items.remove(0);
    }

    let right = items.pop().unwrap();
    let left_count = items.len() as f32;
    let total_count = left_count + 1.0;
    let left = fold_horizontal(items);

    LayoutTree::Split {
        direction: Direction::Horizontal,
        ratio: left_count / total_count,
        left: Box::new(left),
        right: Box::new(right),
    }
}
