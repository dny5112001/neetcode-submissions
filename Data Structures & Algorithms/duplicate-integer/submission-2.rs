impl Solution {
    pub fn has_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort();
        for num in 1..nums.len() {
            if nums[num]== nums[num-1] {
                return true;
            }
        }
        false

    }
}
