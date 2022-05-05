use std::collections::HashSet;

#[allow(dead_code)]
pub fn compute_books_cost(books: Vec<i32>) -> f32 {
    
    let mut cnt: Vec<u32> = vec![0, 0, 0, 0, 0];
    for &book_id in books.iter() {
        if book_id >= 0 && book_id < 5 {
            cnt[book_id as usize] += 1;
        }
    }

    let mut total_cost: f32 = 0.0;
    while cnt.len() > 0 {
        // println!("cnt: {:#?}", cnt);
        cnt.sort();
        if cnt.len() == 5 && cnt[1] == 1 && cnt[2] == 2 && cnt[4] == 2 {
            total_cost += 4.0 * 8.0 * (1.0 - discount_map(4)) * 2.0;
            return total_cost
        }

        let n = cnt.len() as f32;
        let min_count = *cnt.iter().min().unwrap();
        let min_count = min_count.min(1);
        let discount_factor = 1.0 - discount_map(cnt.len());
        total_cost += n * 8.0 * discount_factor * min_count as f32;
        cnt.iter_mut().for_each(|num| *num -= min_count);
        cnt = cnt.into_iter().filter(|&num| num > 0).collect()
    }
    
    return total_cost
}

#[allow(dead_code)]
fn compute_discount(books: Vec<i32>) -> f32{
    let s: HashSet<i32> = books.iter().cloned().collect();
    return discount_map(s.len())
}

fn discount_map(n: usize) -> f32 {
    return match n {
        1 => 0.0,
        2 => 0.05,
        3 => 0.1,
        4 => 0.2,
        _ => 0.25
    }
}

// fn compute_discount(books: Vec<i32>) -> f32{
//     let s: HashSet<i32> = books.iter().cloned().collect();
//     return match s.len() {
//         1 => 0.0,
//         2 => 0.05,
//         3 => 0.1,
//         4 => 0.2,
//         _ => 0.25
//     }
// }