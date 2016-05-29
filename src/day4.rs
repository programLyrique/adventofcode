use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn day4(s : &str, start_pattern: &str) -> u32 {
    let st = s.as_bytes();

    let mut md5_hasher = Md5::new();
    let mut cnt = 1;
    loop {
        md5_hasher.input(st);
        md5_hasher.input(&cnt.to_string().as_bytes());
        let hash = md5_hasher.result_str();
        if hash.starts_with(start_pattern) {
            break;
        };
        md5_hasher.reset();
        cnt +=1
    };
    println!("First number which gives  starting pattern {} for seed {} is {}", start_pattern, s, cnt);
    cnt
}

#[cfg(test)]
mod tests {
    use super::day4;
    #[test]
    #[ignore]
    fn day4_first_test() {
        assert_eq!(day4("abcdef", "00000"), 609043);
        assert_eq!(day4("pqrstuv", "00000"), 1048970);
    }

}
