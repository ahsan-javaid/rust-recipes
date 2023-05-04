macro_rules! four {
    () => {
        Vec::new()
    };
    ($e: expr) => {{
        let mut vs = Vec::new();
        vs.push($e);
        vs
    }};
}
fn main () {
    let x: Vec<i32> = four![2];
    println!("{:?}", x);
}
