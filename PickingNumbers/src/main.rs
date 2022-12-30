fn main() {
    println!("Hello, world!");
}

fn picking_numbers(arr: &[i32]) -> i32 {
    // making a copy of the array so that it can be sorted in order to use the sliding window technique
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    let mut max_length = 0;
    let mut start = 0;
    let mut end = 0;

    while end < sorted_arr.len() {
        if (sorted_arr[start] - sorted_arr[end]).abs() <= 1 {
            max_length = std::cmp::max(max_length, end - start + 1);
            end += 1;
        } else {
            start += 1;
            end = start;
        }
    }

    max_length as i32
}


#[cfg(test)]
mod tests {
    use super::picking_numbers;

    #[test]
    fn test_picking_numbers() {
        // assert_eq!(picking_numbers(&[1, 1, 2, 2, 4 ,4, 5, 5, 5]), 5);
        assert_eq!(picking_numbers(&[1, 2, 2, 3, 3, 3, 4, 4, 5]), 5);
        assert_eq!(picking_numbers(&[4, 6, 5, 3, 3, 1]), 3);
    }
}