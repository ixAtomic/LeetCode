function twoSum(nums: number[], target: number): number[] {
    let i = 0;
    let j = 1;
    while(true){
        let result = nums[i] + nums[j];
        if(result == target) return [i,j];
        else if(j == nums.length - 1){
            i++;
            j = i+1;
        }
        else{
            j++;
        }
    }
};