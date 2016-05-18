extern crate core;

use std::env;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use core::fmt;
use std::cmp;

fn load_file<T: AsRef<Path> + fmt::Display >(filename: T) -> String {
    let mut file = match File::open(&filename) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", filename,
                                                   Error::description(&why)),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
       Err(why) => panic!("couldn't read {}: {}", filename,
                                                  Error::description(&why)),
       Ok(_) => (),
   };
   s
}


fn day1_first(s :&str) -> i32 {
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

fn day1_second(s :&str) -> i32 {
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

struct Cuboid {
    l : u32,
    w : u32,
    h : u32,
}

impl Cuboid {
    fn new(v : &Vec<u32>) -> Cuboid {
        Cuboid {l : v[0], w : v[1], h : v[2] }
    }

    fn surface(&self) -> u32 {
        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.l * self.h
    }

    fn smallest_area(&self) -> u32 {
        cmp::min(self.l * self.w , cmp::min(self.w * self.h, self.l * self.h))
    }

    fn total_wrapping(&self) -> u32 {
        self.surface() + self.smallest_area()
    }

    fn shortest_perimeter(&self) -> u32 {
        cmp::min(2*(self.l + self.w), cmp::min(2*(self.w + self.h), 2*(self.l + self.h)))
    }

    fn volume(&self) -> u32 {
        self.l * self.w * self.h
    }

    fn ribbon_size(&self) -> u32 {
        self.shortest_perimeter() + self.volume()
    }
}

fn day2_first(s : &str) -> u32 {
    let mut wrapping_paper = 0;

    for line in s.lines() {
        let dims = line.split("x").map(|v| v.parse::<u32>().unwrap()).collect();
        let cube = Cuboid::new(&dims);
        wrapping_paper += cube.total_wrapping();
    };
    println!("Total surface of wrapping paper is {}.", wrapping_paper);
    wrapping_paper
}

fn day2_second(s : &str) -> u32 {
    let mut ribbon_size = 0;

    for line in s.lines() {
        let dims = line.split("x").map(|v| v.parse::<u32>().unwrap()).collect();
        let cube = Cuboid::new(&dims);
        ribbon_size += cube.ribbon_size();
    };
    println!("Total size of ribbons is {}.", ribbon_size);
    ribbon_size
}

#[cfg(test)]
mod tests {
    use super::{day1_first, day1_second, day2_first, day2_second};

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

    #[test]
    fn day2_first_test() {
        assert_eq!(day2_first("2x3x4"), 58);
        assert_eq!(day2_first("1x1x10"), 43);
    }

    #[test]
    fn day2_second_test() {
        assert_eq!(day2_second("2x3x4"), 34);
        assert_eq!(day2_second("1x1x10"), 14);
    }

}

fn main() {
    let args : Vec<String> = env::args().collect();

    let file_str = load_file(&args[1]);

    //day1_first(&file_str);
    //day1_second(&file_str);
    day2_first(&file_str);
    day2_second(&file_str);

    println!("Done!");
}
