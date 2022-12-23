use std::fmt::format;

fn main() {
    println!("Hello, world!");
}

fn day_of_programmer(year: i32) -> String {
    let mut date = String::new();

    if is_leap_year(year) {
        date = format!("12.09.{year}")
    } else {
        date = format!("13.09.{year}")
    }

    if year == 1918 {
        date = format!("26.09.{year}");
    }

    date

}

fn is_leap_year(year: i32) -> bool {
    if year%4 != 0 {
        return false
    }
    if year > 1918 && year%100 == 0 && year%400 != 0 {
        return false
    }

    return true;
}


#[cfg(test)]
mod tests {
    use super::day_of_programmer;

    #[test]
    fn test_day_of_programmer() {
        assert_eq!(day_of_programmer(2016), "12.09.2016");
        assert_eq!(day_of_programmer(1800), "12.09.1800");
        assert_eq!(day_of_programmer(1918), "26.09.1918");
    }
}