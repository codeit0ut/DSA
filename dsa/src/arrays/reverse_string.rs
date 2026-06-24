pub fn reverse_string() {
    let mut s = vec!["h","e","l","l","o"];

    let mut i: usize = 0;
    let mut j: usize = s.len() - 1;

    while i < j {
        let temp = s[i];
        s[i] = s[j];
        s[j] = temp;

        i += 1;
        j -= 1;
    }

    for item in s {
        println!("{}", item);
    }
}