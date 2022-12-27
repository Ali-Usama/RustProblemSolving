

fn main() {
    println!("Hello, world!");
    let valleys = counting_valleys(12, "DDUUDDUDUUUD");
    println!("valleys: {}", valleys);
}

fn counting_valleys(steps: i32, path: &str) -> i32 {
    let mut level = 0;
    let mut valleys = 0;
    let mut previous_level = 0;

    for step in path.chars() {
        if step == 'U' {
            level += 1;
        } else {
            level -= 1;
        }
        // To correctly count the number of valleys, we need to track the current elevation level
        // and check if it is below sea level (i.e. negative).
        if level < 0 && previous_level >= 0 {
            valleys += 1;
        }

        previous_level = level;
    }


    valleys
}

#[cfg(test)]
mod tests {
    use super::counting_valleys;

    fn test_counting_valleys() {
        assert_eq!(counting_valleys(8, "UDDDUDUU"), 1);
        assert_eq!(counting_valleys(12, "DDUUDDUDUUUD"), 4);
    }
}