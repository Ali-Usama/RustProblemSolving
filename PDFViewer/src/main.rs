fn main() {
    println!("Hello, world!");
}

fn pdf_viewer(h: &[i32], word: &str) -> i32 {
    let mut tallest_letter = h[0];

    for char in word.chars() {
        let index = char as usize - 'a' as usize;
        if h[index] > tallest_letter {
            tallest_letter = h[index];
        }
    }

    tallest_letter * word.len() as i32
}

#[cfg(test)]
mod tests {
    use super::pdf_viewer;

    #[test]
    fn test_pdf_viewer() {
        assert_eq!(pdf_viewer(&[1, 3, 1, 3, 1, 4, 1, 3, 2, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], "abc"), 9);
    }
}
