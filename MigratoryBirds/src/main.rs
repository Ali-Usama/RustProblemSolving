use std::cmp::max;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
    migratory_birds(&[1, 1, 2, 2, 3]);
}

fn migratory_birds(arr: &[i32]) -> i32 {
    let mut count_arr: HashMap<i32, i32> = HashMap::new();

    for i in arr {
        *count_arr.entry(*i).or_insert(0) += 1;
    }

    println!("ARR: {:?}", count_arr.clone());

    let mut max_count = 0;
    let mut lowest = 0;

    for (key, value) in count_arr {
        if value > max_count || (value == max_count && key < lowest) {
            max_count = value;
            lowest = key;
        }
    }
    lowest
}



#[cfg(test)]
mod tests {
    use super::migratory_birds;

    #[test]
    fn test_migratory_birds() {
        assert_eq!(migratory_birds(&[1, 1, 2, 2, 3]), 1);
        assert_eq!(migratory_birds(&[1, 4, 4, 4, 5, 3]), 4);
    }
}