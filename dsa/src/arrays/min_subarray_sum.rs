pub fn min_subarray_sum() {
    let nums = vec![2,3,1,2,4,3];
    let target = 7;

    let mut i = 0;
    let mut j = 0;
    let mut min_len = usize::MAX;
    let mut sum = nums[0];

    while i < nums.len() && j < nums.len() {
        let len = j - i + 1;

        if sum < target && j != nums.len() - 1 {
            j += 1;
            sum += nums[j];
        } else if sum >= target && min_len > len {
            min_len = len;
            sum -= nums[i];
            i += 1;
        } else {
            sum -= nums[i];
            i += 1;
        }
    }

    if min_len == usize::MAX {
        println!("0");
    } else {
        println!("{}", min_len);
    }
}