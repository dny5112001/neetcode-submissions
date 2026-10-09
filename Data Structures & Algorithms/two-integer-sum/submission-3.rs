use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        // declare the hashmap for storign the value and its index in key value format

        let mut map: HashMap<i32 , usize> = HashMap::new();
        for i in 0..nums.len(){
            // check if target-current value in vec exists in hashmap . if then return the indexes
            if map.contains_key(&(target-nums[i])) {
               let prev_idx = *map.get(&(target-nums[i])).unwrap();
               return vec![prev_idx as i32,i as i32];
            }

            map.insert(nums[i],i);
        }

        vec![-1,-1]
    }
}
