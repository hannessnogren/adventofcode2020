use std::io;
use std::io::prelude::*;


fn get_id(ticket: &str) -> usize {
    let mut res : usize = 0;
    //let len = ticket.len();
    for (i, c) in ticket.chars().rev().enumerate() {
        if c == 'B' || c == 'R' {
            res += 2_usize.pow(i as u32);
        }
        //println!("{}: {} -> {}", i, c, res);
    }
    //println!("{}: {}", ticket, res);
    return res;
}

fn get_input() -> Vec<String> {
    let mut res : Vec<String> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.len() == 0 {
            break;
        }
        res.push(l);
    }

    return res;
}

fn part1() {
    let tickets = get_input();
    let mut max = 0;

    for t in tickets {
        let res = get_id(&t);
        if res > max {
            max = res;
        }
    }
    println!("Max: {}", max);
}

fn part2() {
    let tickets = get_input();
    let mut ids : Vec<usize> = Vec::new();

    for t in tickets {
        ids.push(get_id(&t));
    }
    ids.sort();
    for (i, id) in ids.iter().enumerate() {
        if (ids[i+1] - id) == 2 {
            println!("Lonely seat: {}", id+1);
            break;
        }
    }
}

fn main() {
    println!("Enter tickets...");
    //part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row() {
        assert_eq!(get_id("FBFBBFFRLR"), 357);
        assert_eq!(get_id("BFFFBBFRRR"), 567);
        assert_eq!(get_id("FFFBBBFRRR"), 119);
        assert_eq!(get_id("BBFFBBFRLL"), 820);
    }
}

