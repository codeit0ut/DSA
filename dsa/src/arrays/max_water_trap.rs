pub fn max_water_trap() {
    let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    let n = height.len();
    let mut i: usize = 0;
    let mut j: usize = i + 1;
    let mut max_water = 0;

    while i < n && j < n {
        println!("i: {}, j: {}", i, j);
        if j - i == 1 && height[i] < height[j] {
            i += 1;
            j += 1;
        } else if height[i] > height[j] {
            j += 1;
        } else if height[i] <= height[j] {
            let temp = i;
            while i < j {
                max_water += height[temp] - height[i];
                i += 1;
            }
            j += 1;
        }
        if j == n {
            i += 1;
            j = i + 1;
        }
    }
    println!("{}", max_water);
}