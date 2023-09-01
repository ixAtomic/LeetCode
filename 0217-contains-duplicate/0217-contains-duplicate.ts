function containsDuplicate(nums: number[]): boolean {
    let myMap = {};
    for(let num of nums){
        if(myMap.hasOwnProperty(num)) return true;
        else myMap[num] = num;
    }
    return false;
};