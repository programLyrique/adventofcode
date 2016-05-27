static VOWELS : &'static str = "aeiou";
static FORBIDDEN : [&'static str;4] = ["ab", "cd", "pq", "xy"];


fn is_nice(s : &str) -> bool {
    let mut vowel_counter = 0;

    let mut letter_contig = 0;

    let mut it = s.chars().peekable();


    while let Some(c)  = it.next() {
        //Count vowels
        if VOWELS.contains(c) {
            vowel_counter += 1;
        }

        if let Some(&next_c) = it.peek() {
            //A letter appears twice in a row
            if c == next_c {
                letter_contig += 1
            }

            //Test if there is the forbidden pattern. Not allocation (even of an iterator)
            let mut duet = String::with_capacity(2);
            duet.push(c);
            duet.push(next_c);
            if FORBIDDEN.contains(&&*duet) {
                return false
            }
        }
    };
    vowel_counter >= 3 && letter_contig > 0
}

pub fn day5_first(s : &str) -> u32 {
    let nb_nice = s.lines().filter(|&v| is_nice(v) ).count() as u32;
    println!("There are {} nice strings (version 1)",  nb_nice);
    nb_nice
}

pub fn day5_second(s : &str) -> u32 {
    let nb_nice = s.lines().filter(|&v| is_nice2(v) ).count() as u32;
    println!("There are {} nice strings (version 2)",  nb_nice);
    nb_nice
}

pub fn is_nice2(s : &str) -> bool {
    //Two passes for each nice property

    // Check for nice triplets
    let nice_triplet = (0..s.len()).map(|i| s.chars().skip(i).take(3))
        .any(|mut triplet| {let c1 = triplet.next().unwrap(); triplet.nth(1).map_or(false, |c2| c1 == c2)});

    //Check for similar duets -- slice solution
    let mut nice_duet = false;
    //i = index of the second char of duet
    for i in 1..s.len()  {
        let duet1 = &s[i-1 .. i + 1];
        for j in i + 1..s.len() - 1 {
            let duet2 = &s[j .. j + 2];
            if duet1 == duet2 {
                nice_duet = true;
                break;
            }
        }
    };
    //println!("Nice triplets: {}; nice duets: {}", nice_triplet, nice_duet);
    nice_triplet && nice_duet
}


#[cfg(test)]
mod tests {
    use super::{is_nice, is_nice2};

    #[test]
    fn is_nice_test() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn is_nice2_test(){
        assert!(is_nice2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice2("xxyxx"));
        assert!(!is_nice2("uurcxstgmygtbstg"));
        assert!(!is_nice2("ieodomkazucvgmuy"));
    }
}
