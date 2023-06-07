pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
    arr.sort();
  
    let diff: Vec<i32> = arr.windows(2).map(|a| a[1] - a[0]).collect();
    if let Some(x) = diff.first() {
      diff.iter().all(|y| y == x)
    } else {
        false
    }
}
