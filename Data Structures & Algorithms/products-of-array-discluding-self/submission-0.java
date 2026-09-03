class Solution {
    public int[] productExceptSelf(int[] nums) {
        int [] result = new int[nums.length];
        int prefixCount = 1;
        for(int i = 0;i<nums.length;i++){
            if(i==0){
                result[i] = 1;
            }else{
                prefixCount *= nums[i-1];
                result[i] = prefixCount;
            }
        }

        int suffixCount = 1;
        for(int i = nums.length - 1; i>=0;i--){
            if(i==nums.length -1){
                result[i] *= 1;
            }else{
                suffixCount *= nums[i+1];
                result[i] *= suffixCount;
            }
        }

        return result;
    }
}  
