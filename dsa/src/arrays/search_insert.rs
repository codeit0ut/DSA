pub fn search_insert() {
    let nums = vec![1,3,5,6];
    let target = 2;

    let mut i = 0;
    let mut j = nums.len() as i32 - 1;

    while i <= j {

        let mid = (i + j)/2;

        if target == nums[mid as usize] {
            println!("{}", mid);
            break;
        } else if target < nums[mid as usize] {
            j = mid - 1;
        } else {
            i = mid + 1;
        }
    }

    println!("{}", i);
}