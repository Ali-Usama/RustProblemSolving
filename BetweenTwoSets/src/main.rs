fn main() {
    let factors = get_total_x(&[2, 6], &[24, 36]);
    println!("Factors: {}", factors);
}


fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let mut common_factors: Vec<i32> = vec![];

    for i in 1..b[b.len() -1]+1 {
        let mut is_common_factor = true;

        // check if each element in `a` is factor of the element being considered
        for elem in a.iter() {
            if !is_factor(i, *elem) {
                is_common_factor = false;
                break;
            }
        }

        // check if the element is the factor of each element in `b`
        if is_common_factor {
            for elem in b.iter() {
                if !is_factor(*elem, i) {
                    is_common_factor = false;
                    break;
                }
            }
        }

        if is_common_factor {
            common_factors.push(i)
        }
    }

    common_factors.len() as i32
}

fn is_factor(target: i32, number: i32) -> bool {

    if target % number == 0 {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::get_total_x;

    #[test]
    fn test_get_total_x() {
        assert_eq!(get_total_x(&[2, 6], &[24,36]), 2);
        assert_eq!(get_total_x(&[2, 4], &[16, 32, 96]), 3);
        assert_eq!(get_total_x(&[1], &[100]), 9);
    }
}