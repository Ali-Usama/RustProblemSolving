use std::cmp::min;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn sock_merchant(n: i32, arr: &[i32]) -> i32 {
    let mut pairs = 0;
    let mut sock_count: HashMap<i32, i32> = HashMap::new();

    for item in arr {
        *sock_count.entry(*item).or_insert(0) += 1;
    }

    for (key, value) in sock_count {
        pairs += (value / 2) as i32
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::sock_merchant;

    #[test]
    fn test_sock_merchant() {
        assert_eq!(sock_merchant(7, &[1, 2, 1, 2, 1, 3, 2]), 2);
        assert_eq!(sock_merchant(7, &[1, 1, 3, 1, 2, 1, 3, 3, 3, 3,]), 4);
    }
}