use std::collections::HashMap;

impl Solution {

    pub fn check_anagram(map : &mut HashMap<[usize;26],Vec<String>>, check: String){

        let mut arr = [0usize; 26];
        for i in check.as_bytes(){
            arr[(i-b'a') as usize] += 1;
        }

        // check if key exist
        if let Some(list) = map.get_mut(&arr){
            // key exist then add current string in the list
            list.push(check);
        }else{
            // key doesnt exist , we need to create key value pair
            map.insert(arr,vec![check]);
        }

    }

    
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {

        // We will make use of hashmap with key as string , and value as array of 26 size
        let mut map: HashMap<[usize; 26], Vec<String>> = HashMap::new();

        for i in 0..strs.len(){
            // Check if the current string have anagram in hashmap
            // by comparing the key , then perform insertion accordingly
            Self::check_anagram(&mut map,strs[i].clone());
        }

        // Get all the vectors from the map
        let result : Vec<Vec<String>> = map.into_values().collect();

        return result;        

    }

    
}
