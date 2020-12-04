use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

struct Passport<'a> {
    fields: HashMap<String, String>,
    necessary_fields: Vec<&'a str>,
    optional_fields: Vec<&'a str>,
}

impl Passport<'_> {
    fn new<'a>() -> Passport<'a> {
        Passport {
            fields: HashMap::new(),
            necessary_fields: ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].to_vec(),
            optional_fields: ["cid"].to_vec(),
        }
    }

    fn add_field(&mut self, f: &str, v: &str) {
        self.fields.insert(f.to_string(), v.to_string());
    }

    fn min_valid_len(&self) -> usize {
        return self.necessary_fields.len();
    }

    fn field_count(&self) -> usize {
        return self.necessary_fields.len() + self.optional_fields.len();
    }

    fn missing(&self) -> usize {
        return self.field_count() - self.fields.len();
    }

    fn missing_optional(&self) -> usize {
        let mut missing = 0;
        for f in &self.optional_fields {
            if !self.fields.get(*f).is_some() {
                missing += 1;
            }
        }
        return missing;
    }

    fn is_valid(&self) -> bool {
        if self.fields.len() == self.field_count() {
            return true;
        }
        else if self.fields.len() < self.min_valid_len() {
            return false;
        }
        else {
            return self.missing() == self.missing_optional();
        }
    }

    fn validate_fields(&self) -> bool {
        match self.fields.get("byr").unwrap().parse() {
            Ok(m) => println!("{}", m);
            Err(_) => return false;
        }
        return true;

/*
    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.
*/


    }

    fn is_valid_pt2(&self) -> bool {
        return self.is_valid() && self.validate_fields();
    }
}

fn get_input<'a>() -> Passport<'a> {
    let mut passport = Passport::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.len() == 0 {
            break;
        }
        for entry in l.split(' ') {
            let e : Vec<&str> = entry.split(':').collect();

            passport.add_field(e[0], e[1]);
        }
    }
    return passport;
}

fn main() {
    println!("Enter passport");
    let mut count_p1 = 0;
    let mut count_p2 = 0;
    loop {
        let p = get_input();
        count_p1 += if p.is_valid() {1} else {0};
        count_p2 += if p.is_valid_pt2() {1} else {0};
        println!("{}, {}", count_p1, count_p2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_Passport() {
        let mut p = Passport::new();
        p.add_field("byr", "a");
        p.add_field("iyr", "a");
        p.add_field("eyr", "a");
        p.add_field("hgt", "a");
        p.add_field("hcl", "a");
        p.add_field("ecl", "a");
        p.add_field("pid", "a");
        assert_eq!(p.is_valid(), true);
        p.add_field("cid", "a");
        assert_eq!(p.is_valid(), true);
    }

    #[test]
    fn invalid_Passport() {
        let mut p = Passport::new();
        p.add_field("byr", "a");
        p.add_field("iyr", "a");
        p.add_field("eyr", "a");
        p.add_field("hgt", "a");
        p.add_field("hcl", "a");
        assert_eq!(p.is_valid(), false);
        p.add_field("ecl", "a");
        //p.add_field("pid", "a");
        p.add_field("cid", "a");
        assert_eq!(p.is_valid(), false);

    }
}
