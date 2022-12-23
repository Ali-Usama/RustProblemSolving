fn main() {
    println!("Hello, world!");
    bon_appetite(&[3, 10, 2, 9], 1, 12);
}

fn bon_appetite(bill: &[i32], item_to_skip: i32, anna_paid: i32) {
    let total_bill = calculate_bill(bill, item_to_skip);
    let anna_share = total_bill / 2;
    if anna_share == anna_paid {
        println!("Bon Appetit")
    } else {
        println!("{}", anna_paid - anna_share);
    }

}

fn calculate_bill(arr: &[i32], index_to_skip: i32) -> i32 {
    let mut sum = 0;
    for (index, &item) in arr.iter().enumerate() {
        if index == index_to_skip as usize {
            continue;
        }
        sum += item
    }

    sum
}
