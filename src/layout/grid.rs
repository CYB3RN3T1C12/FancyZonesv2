use crate::layout::{LayoutTree, Direction, Rect};

pub fn generate_grid_pattern(n: u32) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }

    let k: u32 = (n as f32).sqrt() as u32; // floor(sqrt(n))
    let mut rows: Vec<u32> = vec![k; k as usize]; // start from k×k, the k as usize is the number of rows
    let mut extras: u32 = n - (k * k);

    // distribute extras bottom-up, cyclically
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
    pub fn grid(start_id: u32, window_count: u32, _stacks: (u32, u32), _screen: &Rect) -> LayoutTree {
        let rows: Vec<u32> = generate_grid_pattern(window_count);
        let (layout, _) = build_grid_rows(start_id, &rows);
        layout
    }
}

fn build_grid_rows(start_id: u32, rows: &[u32]) -> (LayoutTree, u32) {
    let rows_remaining: u32 = rows.len() as u32;

    if rows_remaining == 1 {
        let layout = build_grid_row_columns(start_id, rows[0]);
        return (layout, start_id + rows[0]);
    }

    let ratio: f32 = 1.0 / rows_remaining as f32;

    let top: LayoutTree = build_grid_row_columns(start_id, rows[0]);
    let (rest_layout, next_id) = build_grid_rows(start_id + rows[0], &rows[1..]);

    (
        LayoutTree::Split {
            direction: Direction::Vertical,
            ratio,
            left: Box::new(top),
            right: Box::new(rest_layout),
        },
        next_id,
    )
}

fn build_grid_row_columns(start_id: u32, cols: u32) -> LayoutTree {
    if cols == 1 {
        return LayoutTree::Leaf(start_id);
    }

    let ratio: f32 = 1.0 / cols as f32;

    LayoutTree::Split {
        direction: Direction::Horizontal,
        ratio,
        left: Box::new(LayoutTree::Leaf(start_id)),
        right: Box::new(build_grid_row_columns(start_id + 1, cols - 1)),
    }
}