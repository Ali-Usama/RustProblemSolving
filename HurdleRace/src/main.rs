fn main() {
    println!("Hello, world!");
}


fn hurdle_race(height: &[i32], k:i32) -> i32 {
    let mut potion_count = 0;
    // sort the array
    let mut copied_arr = height.to_vec();
    copied_arr.sort();

    let last_element = copied_arr[copied_arr.len() -1];

    if last_element > k {
        potion_count = last_element - k;
    }

    potion_count
}

#[cfg(test)]
mod tests {
    use super::hurdle_race;

    #[test]
    fn test_hurdle_race() {
        assert_eq!(hurdle_race(&[1, 2, 3, 3, 2], 1), 2);
        assert_eq!(hurdle_race(&[1, 6, 3, 5, 2], 4), 2);
    }
}