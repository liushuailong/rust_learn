#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_return_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_return_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn test_pravite_add_one() {
        assert_eq!(5, pravite_add_one(4));
    }
}

pub fn prints_and_return_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

fn pravite_add_one(num: i32) -> i32 {
    num + 1
}

