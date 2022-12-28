fn main() {
    println!("Hello, world!");
    count_apples_and_oranges(2, 3, 1, 5, &[2], &[-2]);
}

fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let mut apple_count = 0;
    let mut orange_count = 0;

    for &i in apples {
        let distance = a + i;
        if distance >= s && distance <= t {
            println!("distance: {} {}", distance, i);
            apple_count += 1;
        }
    }

    for &i in oranges {
        let distance = b + i;
        if distance >= s && distance <= t {
            orange_count += 1
        }
    }

    println!("{apple_count}");
    println!("{orange_count}");
}
