use std::io;
use std::io::prelude::*;
use core::num::ParseIntError;

fn get_input() -> Vec<i32> {
    let stdin = io::stdin();
    let mut res: Vec<i32> = Vec::new();

    for line in stdin.lock().lines() {
        let number_res : Result<i32, ParseIntError> = line.unwrap().parse();
        if number_res.is_ok() {
            let number = number_res.unwrap();
            res.push(number);
        }
        else {
            break;
        }
    }

    return res;
}

fn print_vector(numbers : &Vec<i32>) {
    print!("[");
    for (idx, n) in numbers.iter().enumerate() {
        if n == numbers.last().unwrap() {
            println!("{} ({})]", n, idx);
        }
        else {
            print!("{} ({}), ", n, idx);
        }
    }
}

fn find_closest_up(numbers : &Vec<i32>, n : i32) -> usize {
    let res = numbers.binary_search(&n);

    if res.is_ok() {
        return res.unwrap();
    } else {
        let idx = res.unwrap_or_else(|x| x);
        return idx - 1;
    }
}

fn find_closest_down(numbers : &Vec<i32>, n : i32) -> usize {
    let res = numbers.binary_search(&n);

    if res.is_ok() {
        return res.unwrap();
    } else {
        let idx = res.unwrap_or_else(|x| x);
        return idx;
    }
}

fn find_exact(numbers : &Vec<i32>, n : i32) -> Result<usize, usize> {
    return numbers.binary_search(&n);
}

fn part1(numbers: &Vec<i32>, target : &i32) {
    let mut lower = 0;
    let mut upper = numbers.len() - 1;

    let mut iteration = 0;

    while numbers[upper] + numbers[lower] != *target {
        lower = find_closest_down(numbers, target - numbers[upper]);
        upper = find_closest_up(numbers, target - numbers[lower]);
        println!("{}: {}({}) {}({})", iteration, lower, numbers[lower], upper, numbers[upper]);
        iteration += 1;
    }

    println!("{}*{} = {}", numbers[upper], numbers[lower], numbers[upper]*numbers[lower]);
}


fn part2(numbers: &Vec<i32>, target : &i32) {
    let mut iteration = 0;
    let mut cur_numbers = numbers.clone();

    for (idx, cur_number) in numbers.iter().enumerate() {
        cur_numbers.remove(idx);
        let cur_target = target - *cur_number;

        println!("Target: {}/{}", cur_target, target);

        let mut lower = 0;
        let mut upper = cur_numbers.len() - 1;

        while upper > lower {
            lower = find_closest_down(&cur_numbers, cur_target - cur_numbers[upper]);
            upper = find_closest_up(&cur_numbers, cur_target - cur_numbers[lower]);

            println!("{}: {}({}) {}({}) {}({})", iteration,
                lower, cur_numbers[lower], upper, cur_numbers[upper],
                idx, cur_number);
            println!("Sum: {}", cur_numbers[lower] + cur_numbers[upper] + cur_number);

            if cur_numbers[lower] + cur_numbers[upper] == cur_target {
                println!("{}*{}*{} = {}", cur_numbers[upper], cur_numbers[lower], cur_number,
                cur_numbers[upper]*cur_numbers[lower]*cur_number);
                return;
            }
            upper -= 1;

           iteration += 1;
        }

        cur_numbers.insert(idx, *cur_number);
    }
}


fn main() {
    println!("Enter numbers...");

    let mut input = get_input();
    input.sort();
    print_vector(&input);

    let target_sum = 2020;

    //part1(&input, &target_sum);
    part2(&input, &target_sum);

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_down() {
        let array : Vec<i32> = vec![0, 1, 2, 3, 5, 6];
        let num = 4;
        let res : usize = 4;
        println!("{} ({})", array[res], res);
        assert_eq!(find_closest_down(&array, num), res);
    }

    #[test]
    fn test_find_up() {
        let array : Vec<i32> = vec![0, 1, 2, 3, 5, 6];
        let num = 4;
        let res : usize = 3;
        println!("{} ({})", array[res], res);
        assert_eq!(find_closest_up(&array, num), res);
    }
}
