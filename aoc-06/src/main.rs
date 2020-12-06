use std::io;
use std::io::prelude::*;

fn get_input() -> Vec<Vec<char>> {
    let mut res = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut answers = Vec::new();
        for c in line.unwrap().chars() {
            answers.push(c);
        }
        if answers.len() == 0 {
            break;
        } else {
            res.push(answers);
        }
    }
    return res;
}

fn get_unique(group : &Vec<Vec<char>>) -> usize {
    let mut unique_answers = Vec::new();

    for g in group {
        for c in g {
            if !unique_answers.contains(&c) {
                unique_answers.push(c);
            }
        }
    }

    return unique_answers.len();
}

fn get_common(group : &Vec<Vec<char>>) -> usize {
    let mut common_answers = group.first().unwrap().clone();

    for g in group {
        if common_answers.len() == 0 {
            return 0;
        } else {
            let mut cur_answers = Vec::new();
            for a in common_answers {
                if g.contains(&a) {
                    cur_answers.push(a);
                }
            }
            common_answers = cur_answers;
        }
    }

    return common_answers.len();
}

fn part1() {
    let mut sum = 0;

    loop {
        let input = get_input();
        sum += get_unique(&input);

        println!("Sum: {}", sum);
    }
}

fn part2() {
    let mut sum = 0;

    loop {
        let input = get_input();
        sum += get_common(&input);

        println!("Sum: {}", sum);
    }
}

fn main() {
    println!("Enter declarations...");
    //part1();
    part2();
}
