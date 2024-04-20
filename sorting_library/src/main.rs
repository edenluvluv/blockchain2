//main.rs
mod lib;
use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter space-separated numbers:");
    io::stdin().read_line(&mut input)
        .expect("Failed");

    let mut arr: Vec<i32> = input.trim().split_whitespace()
        .map(|x| x.parse().expect("Failed"))
        .collect();

    println!("Your array: {:?}", arr);

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

