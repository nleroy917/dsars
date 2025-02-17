pub mod sort;

use sort::quicksort;

fn main() {
    let mut lst = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];

    quicksort(&mut lst[..]);

    println!("{:?}", lst);

}
