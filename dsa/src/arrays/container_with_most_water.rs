pub fn most_water() {
    let height = vec![1,8,6,2,5,4,8,3,7];

    let mut i: usize = 0;
    let mut j: usize = height.len() - 1;
    let mut max: i32 = 0;

    while i < j {
        let min_height = height[i].min(height[j]);
        let water = min_height * (j - i) as i32;
        if max < water {
            max = water;
        }
        if height[i] < height[j] {
            i += 1;
        } else {
            j -= 1;
        }
    }
    println!("{}", max);
}