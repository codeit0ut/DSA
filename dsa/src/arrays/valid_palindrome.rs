pub fn palindrome() {
    let s = String::from(" ");

    let chars: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    if chars.len() <= 1 {
        println!("True");
        return;
    }

    let mut i: usize = 0;
    let mut j: usize = chars.len() - 1;

    while i < j {
        if chars[i] == chars[j] {
            i += 1;
            j -= 1;
        } else {
            println!("False");
            return
        }
    }
    println!("True");
}