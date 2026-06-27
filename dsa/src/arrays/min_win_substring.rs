use std::collections::{HashMap, HashSet};

pub fn min_win_substring() {

    let s = "ADOBECODEBANC";
    let chars: Vec<char> = s.chars().collect();
    let t = "ABC";
    let tchars: Vec<char> = s.chars().collect();

    let mut i = 0;
    let mut j = 0;
    let min_window = i32::MAX;
    let mut substring: HashMap<char, i32> = HashMap::new();

    while i < chars.len() && j < chars.len() {
        *substring.entry(chars[j]).or_insert(0) += 1;
        j += 1;
    }
}