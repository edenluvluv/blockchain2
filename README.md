# blockchain2

# Sorting Library

## Usage

This project contains implementations of sorting algorithms in Rust, such as merge,selection,insertion and quick sortings. The project contains 2 file, the lib.rs an a library crate and main.rs as a binary crate. Yhe lib.rs bay be used in projects by importing(ex. mod lib;) it and specified in dependencies. 

```
mod lib;

OR

use sorting_library::<the sorting method>;


(Cargo.toml)

[dependencies]

sorting_library = { path = "./sorting_library" }

```

## Screenshots
![image](https://github.com/edenluvluv/blockchain2/assets/124028577/2088c668-1760-44ac-a0a7-4ce7622b7f1d)

## Example

```rust
//main.rs
mod lib;
use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter space-separated integers:");
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let mut arr: Vec<i32> = input.trim().split_whitespace()
        .map(|x| x.parse().expect("Failed to parse integer"))
        .collect();

    println!("Original array: {:?}", arr);

    // Merge Sort
    let mut merge_sorted_arr = arr.clone();
    lib::merge_sort(&mut merge_sorted_arr);
    println!("Merge sorted array: {:?}", merge_sorted_arr);

    // Insertion Sort
    let mut insertion_sorted_arr = arr.clone();
    lib::insertion_sort(&mut insertion_sorted_arr);
    println!("Insertion sorted array: {:?}", insertion_sorted_arr);

    // Quick Sort
    let mut quick_sorted_arr = arr.clone();
    lib::quick_sort(&mut quick_sorted_arr);
    println!("Quick sorted array: {:?}", quick_sorted_arr);

    // Selection Sort
    let mut selection_sorted_arr = arr.clone();
    lib::selection_sort(&mut selection_sorted_arr);
    println!("Selection sorted array: {:?}", selection_sorted_arr);
}
```
Here is provided example code of using it in main.rs where user can input numbers in terminal and see a result.
