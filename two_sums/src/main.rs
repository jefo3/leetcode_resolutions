use std::collections::HashMap;

struct  Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_memorized: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            let complement = target - nums[i];
            if let Some(&j) = num_memorized.get(&complement) {
                return  vec![i as i32, j as i32];
            }

            num_memorized.insert(nums[i], i as i32);   
        }
        
        vec![]
    }
}
fn main() {
    let vec_test = vec![1, 2, 3];
    let target: i32 = 5;

    let result = Solution::two_sum(vec_test, target);
    print!("{:?}", result)
}
