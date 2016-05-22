pub fn day1_first(s :&str) -> i32 {
    let mut floor = 0;

    for ch in s.chars() {
        match ch {
            '(' => floor = floor + 1,
            ')' => floor = floor - 1,
            _ => panic!("Unexpected character")
        }
    };
    println!("Santa is on floor {}", floor);
    floor
}

pub fn day1_second(s :&str) -> i32 {
    let mut floor = 0;
    let mut pos=0;

    for ch in s.chars() {
        match ch {
            '(' => floor = floor + 1,
            ')' => floor = floor - 1,
            _ => panic!("Unexpected character")
        };
        pos = pos + 1;
        if floor < 0 {
            break
        }
    };
    println!("Santa arrived in basement at position {}", pos);
    pos
}

#[cfg(test)]
mod tests {
    use super::{day1_first, day1_second};

    #[test]
    fn day1_first_test() {
        assert_eq!(day1_first("(())"), 0);
        assert_eq!(day1_first("()()"), 0);

        assert_eq!(day1_first("((("), 3);
        assert_eq!(day1_first("(()(()("), 3);

        assert_eq!(day1_first("))((((("), 3);

        assert_eq!(day1_first("())"), -1);
        assert_eq!(day1_first("))("), -1);

        assert_eq!(day1_first(")))"), -3);
        assert_eq!(day1_first(")())())"), -3);
    }

    #[test]
    fn day1_second_test() {
        assert_eq!(day1_second(")"), 1);
        assert_eq!(day1_second("()())"), 5);
    }
}
