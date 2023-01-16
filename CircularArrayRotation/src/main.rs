fn main() {
    println!("Hello, world!");
}

fn circular_array_rotation(a: &[i32], k: i32, queries: &[i32]) -> Vec<i32> {
    let mut indices: Vec<i32> = Vec::new();
    let mut new_arr: Vec<i32> = a.to_vec();
    let rotations = k as usize % new_arr.len();
    new_arr.rotate_right(rotations);

    for &query in queries {
        indices.push(new_arr[query as usize]);
    }
    indices
}

#[cfg(test)]
mod tests {
    use super::circular_array_rotation;

    #[test]
    fn test_array_rotation() {
        assert_eq!(circular_array_rotation(&[3, 4, 5], 2, &[1, 2]), [5, 3]);
    }
}