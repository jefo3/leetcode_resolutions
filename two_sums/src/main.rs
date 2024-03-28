struct  Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 2 {
            return vec![0, 1];
        }

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
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
