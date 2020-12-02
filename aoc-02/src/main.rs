use std::io;
use std::io::prelude::*;
use regex::Regex;

struct Rule {
    min : usize,
    max : usize,
    letter : char,
}

impl Rule {
    fn new(min : usize, max : usize, letter : char) -> Rule {
        Rule {
            min: min,
            max: max,
            letter: letter,
        }
    }

    fn is_valid(&self, pwd : String) -> bool {
        let count = pwd.chars().filter(|x| x == &self.letter).count();

        return self.min <= count && count <= self.max;
    }

    fn is_valid_pt2(&self, pwd : String) -> bool {
        let first = pwd.chars().nth(self.min-1).unwrap() == self.letter;
        let second = pwd.chars().nth(self.max-1).unwrap() == self.letter;

        return first ^ second;
    }
}


fn parse_line<'a>(input : &str) -> (Rule, String) {
    let parser = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w+): (?P<pwd>\w+)$").unwrap();
    match parser.captures(input) {
        Some(x) => {
            let min : usize = x.name("min").unwrap().as_str().parse().unwrap();
            let max : usize = x.name("max").unwrap().as_str().parse().unwrap();
            let letter : char = x.name("letter").unwrap().as_str().chars().next().unwrap();
            let pwd : String = x.name("pwd").unwrap().as_str().to_string();
            return (Rule::new(min, max, letter), pwd);
        },
        None => unreachable!(),
    }
}


fn part1() {
    let mut count = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let (r, s) : (Rule, String) = parse_line(&line.unwrap());
        if r.is_valid(s) {
            count += 1;
            println!("Valid: {}", count);
        }
        else {
            println!("Miss: {}", count);
        }
    }
}


fn part2() {
    let mut count = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let (r, s) : (Rule, String) = parse_line(&line.unwrap());
        if r.is_valid_pt2(s) {
            count += 1;
            println!("Valid: {}", count);
        }
        else {
            println!("Miss: {}", count);
        }
    }
}


fn main() {
    println!("Enter passwords to verify...");
    //part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let r1 = Rule::new(1, 3, 'a');
        assert_eq!(r1.is_valid("aa".to_string()), true);
        assert_eq!(r1.is_valid("basdaj".to_string()), true);
        assert_eq!(r1.is_valid("ab".to_string()), true);
        let r2 = Rule::new(1, 3, 'b');
        assert_eq!(r2.is_valid("asdjb".to_string()), true);
        let r3 = Rule::new(2, 9, 'c');
        assert_eq!(r3.is_valid("ccccccccc".to_string()), true);
    }

    #[test]
    fn test_invalid() {
        let r1 = Rule::new(1, 3, 'a');
        assert_eq!(r1.is_valid("bbbbc".to_string()), false);
        assert_eq!(r1.is_valid("aaaaj".to_string()), false);
        assert_eq!(r1.is_valid("asdsdaasdnka".to_string()), false);
    }

    #[test]
    fn test_line_parse() {
        let (r1, s1) = parse_line("1-3 a: abcde");
        assert_eq!(s1, "abcde");
        assert_eq!(r1.min, 1);
        assert_eq!(r1.max, 3);
        assert_eq!(r1.letter, 'a');
        let (r2, s2) = parse_line("0-10 b: abcde");
        assert_eq!(s2, "abcde");
        assert_eq!(r2.min, 0);
        assert_eq!(r2.max, 10);
        assert_eq!(r2.letter, 'b');
        let (r3, s3) = parse_line("1-3 c: aabbsssaaabsdkajshd");
        assert_eq!(s3, "aabbsssaaabsdkajshd");
        assert_eq!(r3.min, 1);
        assert_eq!(r3.max, 3);
        assert_eq!(r3.letter, 'c');
    }

    #[test]
    fn test_valid_pt2() {
        let r1 = Rule::new(1, 3, 'a');
        assert_eq!(r1.is_valid_pt2("aabbbaa".to_string()), true);
        assert_eq!(r1.is_valid_pt2("bbaa".to_string()), true);
        assert_eq!(r1.is_valid_pt2("abb".to_string()), true);
        let r2 = Rule::new(1, 3, 'b');
        assert_eq!(r2.is_valid_pt2("asbjb".to_string()), true);
        let r3 = Rule::new(2, 9, 'c');
        assert_eq!(r3.is_valid_pt2("jjjjjjccc".to_string()), true);
    }

    #[test]
    fn test_invalid_pt2() {
        let r1 = Rule::new(1, 3, 'a');
        assert_eq!(r1.is_valid_pt2("bbbbc".to_string()), false);
        assert_eq!(r1.is_valid_pt2("aaaaj".to_string()), false);
    }
}
