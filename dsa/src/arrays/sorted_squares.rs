pub fn sorted_squares() {
    let nums = vec![-4,-1,0,3,10];
    let n = nums.len();
    let mut squared_nums = vec![0; n];

    let mut i: usize = 0;
    let mut j: usize = nums.len() - 1;

    for pos in (0..n).rev() {
        let left = nums[i] * nums[i];
        let right = nums[j] * nums[j];

        if left < right {
            squared_nums[pos] = right;
            j -= 1;
        } else {
            squared_nums[pos] = left;
            i += 1;
        }
    }

    for item in &squared_nums {
        println!("{}", item);
    }
}