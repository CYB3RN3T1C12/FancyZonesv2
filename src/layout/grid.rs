use crate::layout::{Direction, Geometry, LayoutKind, Layout, stacks::generate_stack_pattern, fold::fold_horizontal};

/// Generate a grid pattern
/// 
/// ## Arguments
/// * `n` - The number of windows
/// 
/// ## Returns
/// * a `Vec<u32>` - The grid pattern
fn generate_grid_pattern(zones: u32) -> Vec<u32> {
    let k = (zones as f32).sqrt() as u32; // floor(sqrt(n))
    let stacks = if k*k == zones { k } else { k + 1 };

    generate_stack_pattern(zones, (0, stacks))
}

impl Geometry {
    pub fn grid(start_id: u32, zones: u32, _stacks: (u32, u32)) -> Geometry {
        let pattern = generate_grid_pattern(zones);
        let (layout, _) = build_rows(start_id, &pattern);
        layout
    }
}

/// Build a rows layout
/// 
/// ## Arguments
/// * `start_id` - The id of the first window
/// * `rows` - The number of windows in each row
/// 
/// ## Returns
/// * a `Geometry` - The root of the rows layout
fn build_rows(start_id: u32, rows: &[u32]) -> (Geometry, u32) {
    if rows.len() == 1 {
        let layout = build_stack(start_id, rows[0]);
        return (layout, start_id + rows[0]);
    }

    let ratio = 1.0 / rows.len() as f32;

    let top = build_stack(start_id, rows[0]);
    let (rest, next_id) = build_rows(start_id + rows[0], &rows[1..]);

    (
        Geometry::Split {
            direction: Direction::Vertical,
            ratio,
            left: Box::new(top),
            right: Box::new(rest),
        },
        next_id,
    )
}

/// Build a single row of columns (horizontal stacking)
/// 
/// ## Arguments
/// * `start_id` - The id of the first window
/// * `count` - The number of windows
/// 
/// ## Returns
/// * a `Geometry` - The root of the row
fn build_stack(start_id: u32, count: u32) -> Geometry {
    if count == 1 {
        return Geometry::Leaf(start_id);
    }

    let mut items = Vec::new();
    for offset in 0..count {
        items.push(Geometry::Leaf(start_id + offset));
    }

    fold_horizontal(items)
}

impl Layout {
    pub fn grid(start_id: u32, zones: u32) -> Self {
        Self {
            kind: LayoutKind::Grid,
            start_id,
            zones,
            stacks: (0, 0), // grid ignores stacks
            external_padding: 0,
            internal_padding: 0,
        }
    }
}