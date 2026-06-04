use crate::layout::{Direction, Tree};

/// Fold a vector of layout trees into a single layout tree
/// 
/// ## Arguments
/// * `items` - The vector of layout trees to fold
/// 
/// ## Returns
/// * a `Tree` - The folded layout tree
pub fn fold_horizontal(mut items: Vec<Tree>) -> Tree {
    if items.len() == 1 {
        return items.remove(0);
    }

    let right = items.pop().unwrap();
    let left_count = items.len() as f32;
    let total_count = left_count + 1.0;
    let left = fold_horizontal(items);

    Tree::Branch {
        direction: Direction::Horizontal,
        ratio: left_count / total_count,
        left: Box::new(left),
        right: Box::new(right),
    }
}

/// Fold a vector of layout trees into a single layout tree
/// 
/// ## Arguments
/// * `items` - The vector of layout trees to fold
/// 
/// ## Returns
/// * a `Tree` - The folded layout tree
pub fn fold_vertical(mut items: Vec<Tree>) -> Tree {
    if items.len() == 1 {
        return items.remove(0);
    }

    let bottom = items.pop().unwrap();
    let top_count = items.len() as f32;
    let total_count = top_count + 1.0;
    let top = fold_vertical(items);

    Tree::Branch {
        direction: Direction::Vertical,
        ratio: top_count / total_count,
        left: Box::new(top),
        right: Box::new(bottom),
    }
}