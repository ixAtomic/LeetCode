impl Solution {
pub fn is_additive_number(num: String) -> bool {
    if num.len() <= 2 {return false;}
    let string_bytes = num.as_bytes();
    //index = 0
    let mut index = 0;
    //start sum left at 0
    //let mut left_digits = 0;
    //remaining at positive infinity
    //let mut remaining = i64::MAX;
    //while sum left < sum remaining
    loop{
        if index >= string_bytes.len() {return false;}
        if string_bytes[0] as char == '0' && index > 0{return false;}
        let left_digits = string_bytes[0..=index].iter().fold(String::from(""), |acc, value| acc + &String::from(*value as char)).parse::<i64>().unwrap_or(i64::MIN);
        if left_digits == i64::MIN {return false;}

        index = index + 1; //increment

        let right_digits = &string_bytes[index..];


        let result = Solution::get_possible_combinations(left_digits, right_digits);
        if result == true { return result; }


        //remaining = string_bytes[index..].iter().fold(0, |acc, value| acc + String::from(*value as char).parse::<i64>().unwrap());
    };
}

fn get_possible_combinations(left: i64, remaining_array: &[u8]) -> bool {
    // let array_values = remaining_array.iter().map(|&x| (x as char)).collect::<Vec<char>>();
    // println!("new recursion: left: {left}, remaining array: {array_values:?}");
    if remaining_array.is_empty() {return false;}
    let mut sum = i64::MIN;
    let mut remaining = i64::MAX;
    let mut index = 0;
    while sum < remaining || index < remaining_array.len() {
        if remaining_array[0] as char == '0' && index > 0{return false;}
        let right = remaining_array[0..=index].iter().fold(String::from(""), |acc, value| acc + &String::from(*value as char)).parse::<i64>().unwrap_or(i64::MIN);
        if right == i64::MIN {return false;}
        sum = left + right;
        index = index + 1; //increment
        let num_digits = (Solution::doing_it_myself_since_your_compiler_is_old_leet_code(sum, 10)) as usize;
        if remaining_array.len()-1 < num_digits {return false;}
        let expected_result = remaining_array.get(index..index+num_digits).unwrap_or_default().iter().fold(String::from(""), |acc, value| acc + &String::from(*value as char)).parse::<i64>().unwrap_or(i64::MIN);
        if expected_result == i64::MIN {return false;}
        // println!("index: {index}, right: {right}, sum: {sum}, expected result: {expected_result}, number of digits: {num_digits}\n");
        if num_digits == remaining_array[index..].len() && sum == expected_result {return  true;}
        let mut res = false;
        if sum == expected_result { res = Solution::get_possible_combinations(right, &remaining_array[index..]); }
        if res {return true;}
        remaining = remaining_array[index..].iter().fold(0, |acc, value| acc + String::from(*value as char).parse::<i64>().unwrap());
    }
    return false;
}

fn doing_it_myself_since_your_compiler_is_old_leet_code(n: i64, base: i64) -> u32 {
    let mut power = base;
    let mut count = 1;
    while n >= power {
        count += 1;
        if let Some(new_power) = power.checked_mul(base) {
            power = new_power;
        } else {
            break;
        }
    }
    count
}
}

