pub fn max_vowels_substring() {
    let s = "abciiidef";
    let k = 3;
    let str: Vec<char> = s.chars().collect();
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut count = 0;
    
    for i in &str[0..k] {
        if vowels.contains(i) {
            count += 1;
        }
    }

    let mut i = 1;
    let mut max = count;

    while i + k <= str.len() {
        let is_previous = vowels.contains(&str[i-1]);
        let is_next = vowels.contains(&str[i+k-1]);

        if is_previous && !is_next {
            count -= 1;
        } else if !is_previous && is_next {
            count += 1;
        } 

        if count > max {
            max = count;
        }

        i += 1;
    }

    println!("{}", max);
}