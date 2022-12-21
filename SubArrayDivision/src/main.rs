fn main() {
    println!("Hello, world!");
}


fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let mut  count = 0;

    for i in 0..s.len() {
        for j in i+1..s.len()+1 {
            let segment = &s[i..j];
            println!("segment: {:?}", segment);
            if segment.len() as i32 == m && segment.iter().sum::<i32>() == d {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::birthday;

    #[test]
    fn test_birthday() {
        assert_eq!(birthday(&[2, 2, 1, 3, 2], 4, 2), 2);
        assert_eq!(birthday(&[1, 2, 1, 3, 2], 3, 2), 2);
    }
}