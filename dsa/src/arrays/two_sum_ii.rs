pub fn two_sum() {
    let nums: Vec<i32> = vec![-1,0];
    let target: i32 = -1;

    let mut i: usize = 0;
    let mut j: usize = nums.len() - 1;

    while i <= j {
        let sum = nums[i] + nums[j];
        if sum == target {
            println!("i = {}, j = {}", i+1, j+1);
            break;
        } else if sum > target {
            j -= 1;
        } else if sum < target {
            i += 1;
        }
    }
}