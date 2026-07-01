pub fn first_last_pos() {
    let nums = vec![5,7,7,8,8,10];
    let target = 6;

    let mut i = 0;
    let mut j = nums.len() as i32 - 1;
    let mut ans = vec![-1, -1];
    let mut mid = -1;
    let mut found = false;

    while i <= j {
        mid = i + (j - i) / 2;

        if nums[mid as usize] == target {
            found = true;
            break;
        } else if target < nums[mid as usize] {
            j = mid - 1;
        } else {
            i = mid + 1;
        }
    }

    if found {
        let mut k = mid;
        while k >= 0 && nums[k as usize] == target {
            ans[0] = i;
            k -= 1;
        }
        let mut l = mid;
        while l < nums.len() as i32 && nums[l as usize] == target {
            ans[1] = l;
            l += 1;
        }
    }

    println!("{}, {}", ans[0], ans[1]);
}