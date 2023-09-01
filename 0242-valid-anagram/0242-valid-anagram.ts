function isAnagram(s: string, t: string): boolean {
    let myMap = {};
    for(let val of s){
        myMap[val] = ++myMap[val] || 1;
    }

    for(let val of t){
        if(!myMap.hasOwnProperty(val)){
            return false;
        }

        if(myMap[val] > 0){
            myMap[val]--;
        }

        if(myMap[val] == 0) delete myMap[val];
    }

    return Object.keys(myMap).length == 0;
};