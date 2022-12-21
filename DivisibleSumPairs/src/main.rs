fn main() {
    println!("Hello, world!");
}

fn divisible_sum_pairs(ar: &[i32], n: i32, k: i32) -> i32 {
    let mut count = 0;

    for i in 0..ar.len() {
        for j in i+1..ar.len() {
            let sum = ar[i] + ar[j];

            if sum % k == 0 {
                println!("Sum: {sum}");
                count += 1
            }
        }
    }
    count
}


#[cfg(test)]
mod tests {
    use super::divisible_sum_pairs;

    #[test]
    fn test_divisible_sum_pairs() {
        assert_eq!(divisible_sum_pairs(&[1,2, 3, 4, 5, 6], 6, 5), 3);
        assert_eq!(divisible_sum_pairs(&[1, 3, 2, 6, 1, 2], 6, 3), 5);
    }
}
