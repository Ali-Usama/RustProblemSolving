fn main() {
    println!("Hello, world!");
}


fn utopian_tree(n: i32) -> i32 {
    let mut height = 0;

    for i in 0..n+1 {
        if i%2 == 0 {
            height += 1;
        } else {
            height *= 2;
        }
    }
    height
}

#[cfg(test)]
mod tests {
    use super::utopian_tree;

    #[test]
    fn test_utopian_tree() {
        assert_eq!(utopian_tree(5), 14);
        assert_eq!(utopian_tree(0), 1);
        assert_eq!(utopian_tree(1), 2);
        assert_eq!(utopian_tree(4), 7);
    }
}