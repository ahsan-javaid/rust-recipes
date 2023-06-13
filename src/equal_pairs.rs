// Given a 0-indexed n x n integer matrix grid, return the number of pairs (ri, cj) such that row ri and column cj are equal.

// A row and column pair is considered equal if they contain the same elements in the same order (i.e., an equal array).

// Example 1:


// Input: grid = [[3,2,1],[1,7,6],[2,7,7]]
// Output: 1
// Explanation: There is 1 equal row and column pair:
// - (Row 2, Column 1): [2,7,7]
// Example 2:


// Input: grid = [[3,1,2,2],[1,4,4,5],[2,4,2,2],[2,4,2,2]]
// Output: 3
// Explanation: There are 3 equal row and column pairs:
// - (Row 0, Column 0): [3,1,2,2]
// - (Row 2, Column 2): [2,4,2,2]
// - (Row 3, Column 2): [2,4,2,2]
 

// Constraints:

// n == grid.length == grid[i].length
// 1 <= n <= 200
// 1 <= grid[i][j] <= 105


use std::collections::HashMap;

pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let mut map = HashMap::new();

    let (m, n) = (grid.len(), grid[0].len());
    grid.iter().for_each(|row| *map.entry(row).or_insert(0) += 1);

    let mut pairs = 0;

    for j in 0..n {
        let col = (0..m).map(|i| { grid[i][j] }).collect::<Vec<i32>>();
       
        if let Some(&count) = map.get(&col) {
            pairs += count;
        }
    }


    pairs

}

fn main() {
    let x = vec![vec![3,1,2,2],vec![1,4,4,5],vec![2,4,2,2],vec![2,4,2,2]];
    let y = equal_pairs(x);

    println!("{}", y);
}
