fn main() {
    println!("Hello, world!");
}

fn angry_professor(k: i32, a: &[i32]) -> String {
    let mut late_attendees = a.iter().filter(|&x| *x > 0).count();

    if late_attendees < k as usize {
        return String::from("YES")
    }

    String::from("NO")
}

#[cfg(test)]
mod tests {
    use super::angry_professor;

    #[test]
    fn test_angry_professor() {
        assert_eq!(angry_professor(3, &[-1, -3, 4, 2]), "YES".to_string());
        assert_eq!(angry_professor(2, &[0, -1, 2, 1]), "NO".to_string());
        assert_eq!(angry_professor(4, &[-93, -86, 49, -62, -90, -63, -40, 72, 11, 67]), "NO".to_string());
        assert_eq!(angry_professor(10, &[23, -35, -2, 58, -67, -56, -42, -73, -19, 37]), "YES".to_string());
    }
}
