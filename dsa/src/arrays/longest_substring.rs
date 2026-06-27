use std::collections::HashSet;

pub fn longest_substring() {
    let s = "abcabcbb";
    let chars: Vec<char> = s.chars().collect();

    let mut set = HashSet::new();

    let mut i = 0;
    let mut j = 0;
    let mut max_len = 0;

    while j < chars.len() {

        if !set.contains(&chars[j]) {
            set.insert(chars[j]);
            max_len = max_len.max(j - i + 1);
            j += 1;
        } else {
            set.remove(&chars[i]);
            i += 1;
        }
    }

    println!("{}", max_len);
}