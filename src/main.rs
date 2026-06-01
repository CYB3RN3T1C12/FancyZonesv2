#![allow(dead_code)]

mod layout;

use layout::{LayoutType, LayoutTree, Rect};


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

        let x0: usize = (rect.x / cell_w) as usize;
        let y0: usize = (rect.y / cell_h) as usize;
        let x1: usize = ((rect.x + rect.width) / cell_w).min(cols as u32 - 1) as usize;
        let y1: usize = ((rect.y + rect.height) / cell_h).min(rows as u32 - 1) as usize;

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

        // Place label inside the box (top left corner)
        let label_x: usize = x0 + 1;
        let label_y: usize = y0 + 1;

        for (i, ch) in label.chars().enumerate() {
            if label_x + i < x1 {
                grid[label_y][label_x + i] = ch;
            }
        }
    }

    // Print the ASCII map
    println!("\nASCII LayoutTree:\n");
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn main() {
    let screen: Rect = Rect {
        x: 0,
        y: 0,
        width: 1920,
        height: 1200,
    };

    // Testing below

    let start_id: u32 = 0;
    let count: u32 = 21;

    let mut layout: LayoutType = LayoutType::vstack(start_id, count);
    layout.add_stacks(0, 2);
    
    let geometry: LayoutTree = layout.compile();
    let zones: Vec<(u32, Rect)> = geometry.compute(&screen);

    render_ascii_layout(&zones);

    for (id, rect) in zones {
        println!(
            "Zone {}: {}x{} at ({}, {})",
            id, rect.width, rect.height, rect.x, rect.y
        );
    }
}
