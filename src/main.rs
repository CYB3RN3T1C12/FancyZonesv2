#![allow(dead_code)]

mod layout;

use layout::{Layout, Geometry, Rect};

/// Render a layout's zones as an ASCII graph\
/// Great for debugging and visualizing layouts
/// 
/// ## Arguments
/// * `zones`: A vector of tuples containing the zone's ID and its bounding Rect
/// ## Returns
/// * None
pub fn render_ascii_layout(zones: &[(u32, Rect)]) {
    let cell_w: u32 = 15;  // smaller cells = more detail
    let cell_h: u32 = 30;  // 2 spaces ≈ 1 line

    // Determine screen bounds
    let max_x: u32 = zones.iter().map(|(_, r)| r.x + r.width).max().unwrap_or(0);
    let max_y: u32 = zones.iter().map(|(_, r)| r.y + r.height).max().unwrap_or(0);

    let cols: usize = (max_x / cell_w).max(1) as usize;
    let rows: usize = (max_y / cell_h).max(1) as usize;

    // Create ASCII buffer
    let mut grid: Vec<Vec<char>> = vec![vec![' '; cols]; rows];

    // Draw each zone as a box
    for (zone_id, rect) in zones {
        let label: String = format!("{}", zone_id);

        // Skip zero-size zones
        if rect.width == 0 || rect.height == 0 {
            continue;
        }

        // Compute cell indices and clamp to valid ranges
        let cols_u32 = cols as u32;
        let rows_u32 = rows as u32;

        let mut x0_u32: u32 = rect.x / cell_w;
        let mut y0_u32: u32 = rect.y / cell_h;
        let mut x1_u32: u32 = (rect.x + rect.width) / cell_w;
        let mut y1_u32: u32 = (rect.y + rect.height) / cell_h;

        if cols_u32 > 0 {
            x0_u32 = x0_u32.min(cols_u32.saturating_sub(1));
            x1_u32 = x1_u32.min(cols_u32.saturating_sub(1));
        }
        if rows_u32 > 0 {
            y0_u32 = y0_u32.min(rows_u32.saturating_sub(1));
            y1_u32 = y1_u32.min(rows_u32.saturating_sub(1));
        }

        let x0: usize = x0_u32 as usize;
        let y0: usize = y0_u32 as usize;
        let x1: usize = x1_u32 as usize;
        let y1: usize = y1_u32 as usize;

        // If the computed box is invalid (empty or out of range), skip drawing it
        if x0 > x1 || y0 > y1 {
            continue;
        }

        // Draw top and bottom borders
        for x in x0..=x1 {
            grid[y0][x] = '-';
            grid[y1][x] = '-';
        }

        // Draw left and right borders
        for y in y0..=y1 {
            grid[y][x0] = '|';
            grid[y][x1] = '|';
        }

        // Draw corners
        grid[y0][x0] = '+';
        grid[y0][x1] = '+';
        grid[y1][x0] = '+';
        grid[y1][x1] = '+';

        // Place label inside the box (top left corner) if there's space
        let label_x: usize = x0 + 1;
        let label_y: usize = y0 + 1;

        if label_y < rows && label_x < x1 {
            for (i, ch) in label.chars().enumerate() {
                if label_x + i < x1 {
                    grid[label_y][label_x + i] = ch;
                }
            }
        }
    }

    // Print the ASCII map
    println!("\nASCII Tree:\n");
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

/// Entry point
fn main() {
    let screen: Rect = Rect {
        x: 0,
        y: 0,
        width: 1920,
        height: 1200,
    };

    // Testing below

    let start_id: u32 = 0;
    let count: u32 = 22;

    let mut layout: Layout = Layout::bsp(start_id, count);

    println!("{layout:?}");
    // layout.add_stacks(2, 0);
    // layout.remove_stacks(0, 1);
    // layout.set_padding(10, 20);
    println!("{layout:?}");
    
    let geometry: Geometry = layout.compile();
    let zones: Vec<(u32, Rect)> = geometry.compute(&screen);

    render_ascii_layout(&zones);

    for (id, rect) in zones {
        println!(
            "Zone {}: {}x{} at ({}, {})",
            id, rect.width, rect.height, rect.x, rect.y
        );
    }
}
