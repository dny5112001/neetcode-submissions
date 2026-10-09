impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {

        // if their length are different , they are not anagrams
        if s.len() != t.len(){
            return false;
        }

        // create the array of size 26 for freq counting of the alphabets
        let mut freq = [0 as i64 ; 26];

        // we will iterate both the string , will do add count for one and subract for other
        let first = s.as_bytes();
        let second = t.as_bytes();
        for i in 0..s.len(){
            freq[(first[i] - b'a') as usize] +=1;
            freq[(second[i] - b'a') as usize] -= 1;
        }

        for i in 0..26{
            if freq[i] !=0 {
                return false;
            }
        }

        true


    }
}
