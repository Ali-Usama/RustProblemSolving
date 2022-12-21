fn main() {
    println!("Hello, world!");
}


fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut lowest = scores[0];
    let mut highest = scores[0];

    let mut count_lowest = 0;
    let mut count_highest = 0;

    let mut count_records: Vec<i32> = vec![];
    for score in scores.iter() {
        if *score > highest {
            highest = score.clone();
            count_highest +=1 ;
        } else if *score < lowest {
            lowest = score.clone();
            count_lowest += 1;
        }
    }
    count_records.push(count_highest);
    count_records.push(count_lowest);
    count_records
}


#[cfg(test)]
mod tests {
    use super::breaking_records;

    #[test]
    fn test_breaking_records() {
        assert_eq!(breaking_records(&[10, 5, 20, 20, 4, 5, 2, 25, 1]), [2, 4]);
        assert_eq!(breaking_records(&[3, 4 ,21, 36, 10, 28, 35, 5, 24, 42]), [4, 0]);
    }
}