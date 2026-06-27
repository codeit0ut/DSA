pub fn max_avg_subarray() {
    let nums: Vec<i32> = vec![1,12,-5,-6,50,3];
    let k = 4;

    let mut sum = 0;

    for i in 0..k {
        sum += nums[i];
    }

    let mut i = 1;
    let mut max = sum;

    while i + k <= nums.len() {
        sum = sum - nums[i-1] + nums[i+k-1];

        if sum > max {
            max = sum;
        }
        i += 1;
    }

    println!("{}", max/k as i32);
}