fn main() {
    let mut stack = Vec::new();

    for i in 1..=100 {
        stack.push(i);
    }

    println!("stack: {:?}", stack);
}
