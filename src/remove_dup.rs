fn remove_duplicates(numbers: &mut Vec<i32>) {
    numbers.sort(); // Sort the numbers in ascending order
    numbers.dedup(); // Remove duplicates
}

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 2, 5, 1, 6, 3];
    println!("Original array: {:?}", numbers);

    remove_duplicates(&mut numbers);

    println!("Array after removing duplicates: {:?}", numbers);
}
