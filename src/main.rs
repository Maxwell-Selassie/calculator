use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}",slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    let xs = [1, 2, 3, 4, 5];

    let ys: [i32; 500] = [0; 500];

    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    println!("Number of elements in the array: {}", xs.len());

    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // arrays can be automatically borrowed as slices
    println!("Borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);
}