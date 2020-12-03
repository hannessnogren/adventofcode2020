use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::ops::Add;

#[derive(Eq)]
struct Point {
    x : usize,
    y : usize,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other : Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other : &Self) -> Ordering {
        if self.x == other.x {
            return self.y.cmp(&other.y);
        }
        else {
            return self.x.cmp(&other.x);
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other : &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other : &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

struct Map {
    map : Vec<Point>,
    width : usize,
    height : usize,
}

impl Map {
    fn new() -> Map {
        let map = Vec::new();
        Map {
            map: map,
            width: 1,
            height: 1,
        }
    }

    fn add_tree(&mut self, x : usize, y : usize) {
        let p = Point { x: x, y: y };
        self.map.push(p);
    }

    fn sort(&mut self) {
        self.map.sort();
    }

    fn set_dim(&mut self, width : usize, height : usize) {
        self.width = width;
        self.height = height;
    }

    fn has_tree(&self, x : usize, y : usize) -> bool {
        let p = Point { x: x % self.width, y: y };
        match self.map.binary_search(&p) {
            Ok(m) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}

fn get_input() -> Map {
    let stdin = io::stdin();
    let mut map = Map::new();
    let mut width = 0;
    let mut height = 0;

    for (i, line) in stdin.lock().lines().enumerate() {
        let l = line.unwrap();
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                map.add_tree(j, i);
            }
        }

        if l.len() > width {
            width = l.len();
        } else if l.len() == 0 {
            break;
        }

        height += 1;
    }

    map.set_dim(width, height);
    map.sort();

    return map;
}

fn find_crashes(map : &Map, start : (usize, usize), step : (usize, usize)) -> i64 {
    let mut p = Point { x: start.0, y: start.1 };
    let p_inc = Point { x: step.0, y: step.1 };
    let mut res = 0;
    while p.y <= map.height {
        if map.has_tree(p.x, p.y) {
            res += 1;
        }
        p.x += p_inc.x;
        p.y += p_inc.y;
    }
    return res;
}

fn part1() {
    let map = get_input();
    let start = (0, 0);
    let step = (3, 1);
    let res = find_crashes(&map, start, step);
    println!("Crashes: {}", res);
}

fn part2() {
    let map = get_input();
    let start = (0, 0);

    let step1 = (1, 1);
    let res1 = find_crashes(&map, start, step1);
    println!("Crashes 1: {}", res1);
    let step2 = (3, 1);
    let res2 = find_crashes(&map, start, step2);
    println!("Crashes 2: {}", res2);
    let step3 = (5, 1);
    let res3 = find_crashes(&map, start, step3);
    println!("Crashes 3: {}", res3);
    let step4 = (7, 1);
    let res4 = find_crashes(&map, start, step4);
    println!("Crashes 4: {}", res4);
    let step5 = (1, 2);
    let res5 = find_crashes(&map, start, step5);
    println!("Crashes 5: {}", res5);

    let prod = res1*res2*res3*res4*res5;
    println!("{}*{}*{}*{}*{} = {}", res1, res2, res3, res4, res5, prod);
}

fn main() {
    println!("Enter map...");
    //part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let mut m = Map::new();
        m.add_tree(1, 2);
        m.add_tree(5, 123);
        m.add_tree(0, 3);
        m.set_dim(9, 100);
        m.sort();
        assert_eq!(m.has_tree(1, 2), true);
        assert_eq!(m.has_tree(5, 123), true);
        assert_eq!(m.has_tree(4, 5), false);
        assert_eq!(m.has_tree(0, 3), true);
    }

    #[test]
    fn test_tree_extend() {
        let mut m = Map::new();
        m.add_tree(1, 2);
        m.set_dim(3, 5);
        assert_eq!(m.has_tree(1, 2), true);
        assert_eq!(m.has_tree(4, 2), true);
        assert_eq!(m.has_tree(7, 2), true);
    }
}
