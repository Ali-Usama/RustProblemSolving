fn main() {
    println!("Hello, world!");
}

fn save_prisoner(n: i32, m:i32, s:i32) -> i32 {
    let num = (s + m -1) % n;
    
    if num == 0 {
        return n;
    }
    
    num
}

#[cfg(test)]
mod tests {
    use super::save_prisoner;

    #[test]
    fn test_save_prisoner() {
        assert_eq!(save_prisoner(3, 7, 3), 3);
    }
}