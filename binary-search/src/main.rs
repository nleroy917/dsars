pub mod search;

use search::{binary_search, linear_search};
use std::time::Instant;

fn main() {
    let large_sorted_vec: Vec<i32> = (1..=100_000_000).step_by(2).collect();

    let target = 70_654_321;
    // let target = 70_654_320;

    // binary search
    let start = Instant::now();
    let result = binary_search(&large_sorted_vec, target);
    let duration = start.elapsed();

    println!("Time taken for binary search: {:?}", duration);
    println!("Result of binary search: {:?}", result);

    // linear search
    let start = Instant::now();
    let result = linear_search(&large_sorted_vec, target);

    let duration = start.elapsed();

    println!("Time taken for linear search: {:?}", duration);
    println!("Result of linear search: {:?}", result);

}
