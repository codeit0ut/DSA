pub fn three_sum() {
    let mut nums = vec![-1,0,1,2,-1,-4];
    nums.sort();
    let n = nums.len();
    let mut result: Vec<Vec<i32>> = Vec::new();

    for k in 0..n {
        if k > 0 && nums[k] == nums[k - 1] {
            continue;
        }

        let mut i = k + 1;
        let mut j = n - 1;

        while i < j {
            let sum = nums[k] + nums[i] + nums[j];

            if sum == 0 {
                result.push(vec![nums[k], nums[i], nums[j]]);
                
                i += 1;
                j -= 1;

                while i < j && nums[i] == nums[i - 1] {
                    i += 1;
                }

                while i < j && nums[j] == nums[j + 1] {
                    j -= 1;
                }

            } else if sum < 0 {
                i += 1;
            } else {
                j -= 1;
            }
        }
    }
}