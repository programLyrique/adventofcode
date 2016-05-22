/*
 * First idea: using big matrix with booleans and then count. To init the bit matrix: max dim = size of file
 * (roughly...)
 * 2nd idead, smarter: store segments, and then do a scan and count once every point
 * where a segment goes through (i.e. if there are several intersections, it counts for 1)
 */


pub fn day3_first(s : &str) -> u32 {
    let max_dim = 2 * s.len();
    // Build 2 dim array
    let mut grid_raw = vec![false; max_dim * max_dim];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(max_dim).collect();
    let mut grid: &mut [&mut [_]] = grid_base.as_mut_slice();

    let mut x = max_dim / 2;
    let mut y = max_dim / 2;

    for ch in s.chars() {
        grid[x][y] = true;
        match ch  {
            '^' => y = y + 1,
            'v' => y = y - 1,
            '<' => x = x + 1,
            '>' => x = x - 1,
            _ => panic!("Unexpected character")
        };
    };
    grid[x][y] = true;

    //Count points which are true
    let nb_houses = grid.iter().fold(0, |acc1, row| acc1 + row.iter().fold(0, |acc2, &v| acc2 + if v {1} else {0}));
    println!("Santa has given presents to {} houses.", nb_houses);
    nb_houses
}

#[derive(Debug)]
struct Position {
    x : usize,
    y : usize,
}

pub fn day3_second(s : &str) -> u32 {
    let max_dim = 2 * s.len();
    // Build 2 dim array
    let mut grid_raw = vec![false; max_dim * max_dim];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(max_dim).collect();
    let mut grid: &mut [&mut [_]] = grid_base.as_mut_slice();

    let mut santa = Position {x : max_dim /2, y : max_dim / 2};
    let mut robot = Position {x : max_dim /2, y : max_dim / 2};
    grid[santa.x][santa.y] = true;
    //println!("Initial positions {:?}", santa);

    let mut santa_turn= true;
    for ch in s.chars() {
        let  mut p= if santa_turn {&mut santa} else {&mut robot};

        match ch  {
            '^' => p.y = p.y + 1,
            'v' => p.y = p.y - 1,
            '<' => p.x = p.x - 1,
            '>' => p.x = p.x + 1,
            _ => panic!("Unexpected character")
        };
        grid[p.x][p.y] = true;

        //println!("{}: {:?}", if santa_turn {"santa"} else {"robot santa"}, p);

        //Now,, it is the other one's turn
        santa_turn = !santa_turn;
    };


    //Count points which are true
    let nb_houses = grid.iter().fold(0, |acc1, row| acc1 + row.iter().fold(0, |acc2, &v| acc2 + if v {1} else {0}));
    println!("{} houses have received presents", nb_houses);
    nb_houses
}

#[cfg(test)]
mod tests {
    use super::{day3_first, day3_second};


    #[test]
    fn day3_first_test() {
        assert_eq!(day3_first(">"), 2);
        assert_eq!(day3_first("^>v<"), 4);
        assert_eq!(day3_first("^v^v^v^v^v"), 2);
    }

    #[test]
    fn day3_second_test() {
        assert_eq!(day3_second("^v"), 3);
        assert_eq!(day3_second("^>v<"), 3);
        assert_eq!(day3_second("^v^v^v^v^v"), 11);
    }

}
