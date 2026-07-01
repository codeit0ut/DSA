pub fn binary_search() {
    let nums = vec![-1,0,3,5,9,12];
    let target = 9;

    let mut j: usize = nums.len() - 1;
    let mut i: usize = 0;
    let mut ans = -1;

    while i <= j {
        let mid = (i + j) / 2;

        if target == nums[mid] {
            ans = mid as i32;
            break;
        } else if target < nums[mid] {
            j = mid - 1;
        } else {
            i = mid + 1;
        }
    }

    println!("{}", ans);
}