use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut encountered = HashMap::new();
        for (idx, num) in nums.into_iter().enumerate() {
            let complement = target - num;
            if let Some(complement_idx) = encountered.get(&complement) {
                return vec![*complement_idx, idx as i32];
            }
            encountered.insert(num, idx as i32);
        }
        unreachable!()
    }
}
