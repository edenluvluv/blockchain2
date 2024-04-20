//main.rs
mod lib;

fn main() {
    let mut arr1 = [6,8,1,4,2,5];
    println!("Original array: {:?}", arr1);
    lib::merge_sort(&mut arr1);
    println!("Merge sorted array: {:?}", arr1);

    let mut arr2 = [6,8,1,4,2,5];
    println!("Original array: {:?}", arr2);
    lib::insertion_sort(&mut arr2);
    println!("Insertion sorted array: {:?}", arr2);

    let mut arr3 = [6,8,1,4,2,5];
    println!("Original array: {:?}", arr3);
    lib::quick_sort(&mut arr3);
    println!("Quick sorted array: {:?}", arr3);

    let mut arr4 = [6,8,1,4,2,5];
    println!("Original array: {:?}", arr4);
    lib::selection_sort(&mut arr4);
    println!("Selection sorted array: {:?}", arr4);
}
