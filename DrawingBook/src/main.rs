fn main() {
    println!("Hello, world!");
}

fn page_count(n: i32, p: i32) -> i32 {
    let from_front = p/2;    // pages to be turned from the beginning of the book
    let from_back = n/2 - p/2;      // pages to be turned from the end of the book

    // return the minimum of two
    std::cmp::min(from_front, from_back)
}

#[cfg(test)]
mod tests {
    use super::page_count;

    #[test]
    fn test_page_count() {
        assert_eq!(page_count(5, 3), 1);
        assert_eq!(page_count(6, 2), 1);
        assert_eq!(page_count(5, 4), 0);
    }
}
