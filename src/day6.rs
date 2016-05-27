use std::cmp;

#[derive(PartialEq, Debug)]
struct Point {
    x : u32,
    y : u32
}

#[derive(PartialEq, Debug)]
enum SantaCommand {
    TurnOn,
    TurnOff,
    Toggle
}

#[derive(PartialEq, Debug)]
struct SantaInstr {
    pub command : SantaCommand,
    pub l_p : Point,
    pub r_p : Point
}

fn parse_instr(s : &str) -> SantaInstr {
    let mut it = s.split_whitespace();

    //FSM
    let command = match it.next().unwrap() {
        "turn" => match it.next().unwrap() {
                    "on" => SantaCommand::TurnOn,
                    "off" => SantaCommand::TurnOff,
                    _ => panic!("Unrecognized keyword"),
                },
        "toggle" => SantaCommand::Toggle,
        _ => panic!("Unrecognized keyword")
    };

    let mut it2 = it.next().unwrap().split(',');

    let l_p = Point {x : it2.next().unwrap().parse::<u32>().unwrap(), y : {it2.next().unwrap().parse::<u32>().unwrap()}};

    it.next().unwrap(); // through

    it2 = it.next().unwrap().split(',');
    let r_p = Point {x : it2.next().unwrap().parse::<u32>().unwrap(), y : {it2.next().unwrap().parse::<u32>().unwrap()}};

    SantaInstr{ command : command, l_p : l_p , r_p : r_p}
}

fn turn1(grid : &mut [[bool; 1000]; 1000], p1 : & Point, p2 : & Point , value : bool) {
    for i in p1.x..p2.x + 1 {
        for j in p1.y..p2.y+1 {
            grid[i as usize][j as usize] = value
        }
    }
}

fn toggle1(grid : &mut [[bool; 1000]; 1000], p1 : & Point, p2 : & Point) {
    for i in p1.x..p2.x + 1 {
        for j in p1.y..p2.y+1 {
            grid[i as usize][j as usize] = !grid[i as usize][j as usize]
        }
    }
}

pub fn day6_first(s : &str) -> u32 {
    let mut grid = Box::new([[false; 1000]; 1000]);//box because rust stack is only 2MB


    for line in s.lines() {
        let SantaInstr {command, l_p, r_p} = parse_instr(line);
        match command {
            SantaCommand::TurnOn => turn1(&mut grid, &l_p, &r_p, true),
            SantaCommand::TurnOff => turn1(&mut grid, &l_p, &r_p, false),
            SantaCommand::Toggle => toggle1(&mut grid, &l_p, &r_p),
        }
    };
    let res = grid.iter().fold(0, |acc1, row| acc1 + row.iter().fold(0, |acc2, &v| acc2 + if v {1} else {0}));
    println!("{} lights are turned on", res);
    res
}

fn change_brightness(grid : &mut [[u32; 1000]; 1000], p1 : & Point, p2 : & Point, value : i32) {
    for i in p1.x..p2.x + 1 {
        for j in p1.y..p2.y+1 {
            grid[i as usize][j as usize] = cmp::max(0, grid[i as usize][j as usize] as i32 + value) as u32
        }
    }
}


pub fn day6_second(s : &str) -> u32 {
    let mut grid = Box::new([[0; 1000]; 1000]);//box because rust stack is only 2MB


    for line in s.lines() {
        let SantaInstr {command, l_p, r_p} = parse_instr(line);
        match command {
            SantaCommand::TurnOn => change_brightness(&mut grid, &l_p, &r_p, 1),
            SantaCommand::TurnOff => change_brightness(&mut grid, &l_p, &r_p, -1),
            SantaCommand::Toggle => change_brightness(&mut grid, &l_p, &r_p, 2),
        }
    };
    let res = grid.iter().fold(0, |acc1, row| acc1 + row.iter().fold(0, |acc2, &v| acc2 + v));
    println!("Total brightness is {}", res);
    res
}

#[cfg(test)]
mod tests {
    use super::{parse_instr, SantaInstr, Point, SantaCommand};

    #[test]
    fn test_parse_instr() {
        assert_eq!(parse_instr("turn on 0,0 through 999,999"), SantaInstr {command : SantaCommand::TurnOn, l_p : Point {x : 0, y : 0}, r_p : Point {x : 999, y : 999}});
        assert_eq!(parse_instr("toggle 0,0 through 999,0"), SantaInstr {command : SantaCommand::Toggle, l_p : Point {x : 0, y : 0}, r_p : Point {x : 999, y : 0}});
        assert_eq!(parse_instr("turn off 499,499 through 500,500"), SantaInstr {command : SantaCommand::TurnOff, l_p : Point {x : 499, y : 499}, r_p : Point {x : 500, y : 500}});

    }
}
