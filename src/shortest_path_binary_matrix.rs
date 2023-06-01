use std::collections::VecDeque;

pub fn shortest_path_binary_matrix(grid: &mut Vec<Vec<i32>>) -> i32 {
       let n = grid.len();
       let mut queue: VecDeque<((usize, usize), i32)> = VecDeque::new();

       if grid[0][0] == 0 {
         queue.push_back(((0, 0), 1));
       }  

       while queue.len() != 0 {
        let ((x, y), dist) = queue.pop_front().unwrap();

        if (x, y) == (n-1, n-1) {
            return dist;
        }

        for i in y - (y != 0) as usize ..= y + (y != (n - 1)) as usize {
            for j in x - (x != 0) as usize ..= x + ( x != (n - 1)) as usize {
                if grid[i][j] == 0 && (j, i) != (x, y) {
                    grid[i][j] = -1;
                    queue.push_back(((j, i), dist + 1))
                }
            }
        }


       }

       -1 
}

fn main() {

    let mut matrix: Vec<Vec<i32>> = [[0,0,0].to_vec(),[1,1,0].to_vec(),[1,1,0].to_vec()].to_vec();

    let answer = shortest_path_binary_matrix(&mut matrix);

    println!("{}", answer);
}
