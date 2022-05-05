use std::collections::HashSet;

#[allow(dead_code)]
pub fn compute_books_cost(books: Vec<i32>) -> f32 {
    let n = books.len() as f32;
    return n * 8.0 * (1.0 - compute_discount(books))
}

fn compute_discount(books: Vec<i32>) -> f32{
    let s: HashSet<i32> = books.iter().cloned().collect();
    return match s.len() {
        1 => 0.0,
        2 => 0.05,
        3 => 0.1,
        4 => 0.2,
        _ => 0.25
    }
}