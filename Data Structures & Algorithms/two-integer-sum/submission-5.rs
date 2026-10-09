use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        // declare the hashmap for storign the value and its index in key value format

        let mut map: HashMap<i32 , usize> = HashMap::new();

        for (i , &num ) in nums.iter().enumerate(){
            let diff = target-num;

            if let Some(&prev_idx) = map.get(&diff){
                return vec![prev_idx as i32, i as i32];
            }

            map.insert(num,i);
        }

        vec![]
    }
}
