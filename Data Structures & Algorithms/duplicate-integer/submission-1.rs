impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut nums1 = nums.clone();
        // Sort the given vector
        nums1.sort();
        for num in 1..nums.len() {
            if nums1[num]== nums1[num-1] {
                return true;
            }
        }
        false

    }
}
