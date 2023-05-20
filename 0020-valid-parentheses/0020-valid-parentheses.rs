
use std::collections::HashMap;
impl Solution {
    pub fn is_valid(s: String) -> bool{
        let mut char_vec: Vec<char> = Vec::new();
        let char_map: HashMap<char, char> = HashMap::from([ ('(', ')'), ('{', '}'), ('[', ']')]);
        for char in s.chars() {
            if char_map.contains_key(&char) {
                char_vec.push(char_map[&char]);
            }
            else {
                if let Some(value) = char_vec.pop() {
                    if value != char {return false;}
                }
                else {
                    return false;
                }
            }
        }
        return if char_vec.len() > 0 {false} else {true};;
    }
}