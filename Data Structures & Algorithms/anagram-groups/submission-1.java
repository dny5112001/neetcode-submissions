class Solution {
    public List<List<String>> groupAnagrams(String[] strs) {

        HashMap <String,List<String>> map = new HashMap<>();
        for (String str : strs){
            String key = getCharacterNum(str);
            List<String> list = map.getOrDefault(key, new ArrayList<>());
            list.add(str);
            map.put(key,list);
        }

        return new ArrayList<>(map.values());

    }

    public String getCharacterNum(String str){
        int[] arr = new int[26];
        for(char c : str.toCharArray()){
            arr[c-'a']++;
        }

        return Arrays.toString(arr);
    }
}
