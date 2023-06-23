
struct Solution {}
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let n = nums.len();
        let mut map = HashMap::new();

        for i in 0..n {
            for j in i+1..n {
                let v = *map.entry((i, nums[i] - nums[j])).or_insert(1);
                println!("{i}:{j}:{v}");
                map.insert((j, nums[i] - nums[j]), v + 1);
            }
        }

        *map.values().max().unwrap()
    }
}

fn main() {
    let nums1 = vec![3,6,9,12];

    let r = Solution::longest_arith_seq_length(nums1);

    println!("{r}")
}
