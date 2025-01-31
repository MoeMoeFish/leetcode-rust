use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map:HashMap<i32, i32>  = HashMap::new();
        
        for (index, value) in nums.iter().enumerate() {
            let i = index as i32;
            let v = *value;

            let key = target - v;
            if map.contains_key(&key) {
                let i2 = map.get(&key).unwrap().clone();

                return vec![i2, i]
            }
            map.insert(v, i);
        }
        
        return vec![]
    }
}

pub(crate) struct Solution;