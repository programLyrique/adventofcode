use std::cmp;


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

pub fn day2_first(s : &str) -> u32 {
    let mut wrapping_paper = 0;

    for line in s.lines() {
        let dims = line.split("x").map(|v| v.parse::<u32>().unwrap()).collect();
        let cube = Cuboid::new(&dims);
        wrapping_paper += cube.total_wrapping();
    };
    println!("Total surface of wrapping paper is {}.", wrapping_paper);
    wrapping_paper
}

pub fn day2_second(s : &str) -> u32 {
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
    use super::{day2_first, day2_second};
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
