use std::num::ParseIntError;

fn give_number(input: &str) -> Result<i32, ParseIntError> {
    input.parse::<i32>()
}

fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() -> Result<(), ParseIntError> {
    let r = give_number("88")?;
    
    print_type(&r);
    Ok(())
}
