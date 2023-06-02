impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let n = bombs.len();
        let mut graph = vec![Vec::new(); n];

        for i in (0..n) {
            for j in (0..i) {
                let x_distance: i64 = (bombs[i][0] - bombs[j][0]).into();
                let y_distance: i64 = (bombs[i][1] - bombs[j][1]).into();
                let distance_sq: i64 = x_distance * x_distance + y_distance * y_distance;
                let i_r_sq = bombs[i][2] as i64 * bombs[i][2] as i64;

                if distance_sq <= i_r_sq {
                    graph[i].push(j);
                }

                let j_r_sq = bombs[j][2] as i64 * bombs[j][2] as i64;

                if distance_sq <= j_r_sq {
                    graph[j].push(i);
                }
            }
        }

        let mut skip = vec![false; n];
        let mut best = 0;

        for initial in (0..n) {
            if skip[initial] {
                continue;
            }

            let mut detonated = vec![false; n];
            let mut stack = vec![initial];

            while let Some(explodes) = stack.pop() {
                detonated[explodes] = true;

                for &chain_to in graph[explodes].iter() {
                    if !detonated[chain_to] {
                        stack.push(chain_to);
                    }
                }
            }
            best = best.max(detonated.iter().filter(|&&is_exploded| is_exploded).count() as i32);
            for i in detonated.into_iter().enumerate().filter_map(|(i, is_exploded)| is_exploded.then(|| i)) {
                skip[i] = true;
            }
        }
        best
    }
}
