use std::ops::Range;

fn main() {
    println!("Hello, world!");
    beautiful_days(13, 45, 3);
}

fn beautiful_days(i: i32, j: i32, k: i32) -> i32 {
    let range: Range<i32> = i..j+1;
    let mut days = 0;
    for day in range {
        let reverse_day = reverse_num(day);
        println!("Day: {}: {day}", reverse_day);
        if (day-reverse_day).abs() % k == 0 {
            days += 1;
        }
        // println!("{day}")
    }

    days

}

fn reverse_num(num: i32) -> i32 {
    let reverse_string = num.to_string().chars().rev().collect::<String>();

    reverse_string.parse::<i32>().unwrap()
}


#[cfg(test)]
mod tests {
    use super::beautiful_days;

    #[test]
    fn test_beautiful_days() {
        assert_eq!(beautiful_days(20, 23, 6), 2);
        assert_eq!(beautiful_days(13, 45, 3), 33);
    }
}