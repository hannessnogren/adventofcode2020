use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use regex::Regex;

struct Passport<'a> {
    fields: HashMap<String, String>,
    necessary_fields: Vec<&'a str>,
    optional_fields: Vec<&'a str>,
}

impl<'a> Passport<'a> {
    fn new() -> Self {
        Passport {
            fields: HashMap::new(),
            necessary_fields: ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].to_vec(),
            optional_fields: ["cid"].to_vec(),
        }
    }

    fn add_field(&mut self, f: &str, v: &str) {
        self.fields.insert(f.to_string(), v.to_string());
    }

    fn parse_line(&mut self, line: &str) {
        for entry in line.split(' ') {
            let e : Vec<&str> = entry.split(':').collect();

            self.add_field(e[0], e[1]);
        }
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
        } else if self.fields.len() < self.min_valid_len() {
            return false;
        } else {
            return self.missing() == self.missing_optional();
        }
    }

    fn validate_fields(&self) -> bool {
        for (k, v) in &self.fields {
            if k == "byr" {
                //byr (Birth Year) - four digits; at least 1920 and at most 2002.
                match v.parse::<i32>() {
                    Ok(n) => {
                        if n < 1920 || n > 2002 {
                            return false;
                        }
                    },
                    Err(_e) => { return false; },
                }
            } else if k == "iyr" {
                //iyr (Issue Year) - four digits; at least 2010 and at most 2020.
                match v.parse::<i32>() {
                    Ok(n) => {
                        if n < 2010 || n > 2020 {
                            return false;
                        }
                    },
                    Err(_e) => { return false; },
                }
            } else if k == "eyr" {
                //eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
                match v.parse::<i32>() {
                    Ok(n) => {
                        if n < 2020 || n > 2030 {
                            return false;
                        }
                    },
                    Err(_e) => { return false; },
                }
            } else if k == "hgt" {
                //hgt (Height) - a number followed by either cm or in:
                //    If cm, the number must be at least 150 and at most 193.
                //    If in, the number must be at least 59 and at most 76.
                let r_cm = Regex::new(r"^(?P<height>[0-9]{3})cm").unwrap();
                let r_in = Regex::new(r"^(?P<height>[0-9]{2})in").unwrap();
                if let Some(m) = r_cm.captures(v) {
                    let h : i32 = m.name("height").unwrap().as_str().parse().unwrap();
                    if h < 150 || h > 193 {
                        return false;
                    }
                } else if let Some(m) = r_in.captures(v) {
                    let h : i32 = m.name("height").unwrap().as_str().parse().unwrap();
                    if h < 59 || h > 76 {
                        return false;
                    }
                } else {
                    return false;
                }
            } else if k == "hcl" {
                //hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
                let r = Regex::new(r"#[0-9a-f]{6}$").unwrap();
                if !r.is_match(v) {
                    return false;
                }
            } else if k == "ecl" {
                //ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
                if !["amb".to_string(), "blu".to_string(), "brn".to_string(), "gry".to_string(), "grn".to_string(), "hzl".to_string(), "oth".to_string()].contains(&v) {
                    return false;
                }
            } else if k == "pid" {
                //pid (Passport ID) - a nine-digit number, including leading zeroes.
                let r = Regex::new(r"^[0-9]{9}$").unwrap();
                if !r.is_match(v) {
                    return false;
                }
            } else if k == "cid" {
                //cid (Country ID) - ignored, missing or not.
            } else {
                unreachable!();
            }
        }
        return true;
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
        passport.parse_line(&l);
    }
    return passport;
}

fn main() {
    println!("Enter passport");
    let mut count_p1 = 0;
    let mut count_p2 = 0;
    loop {
        let p = get_input();
        count_p1 += if p.is_valid() { 1 } else { 0 };
        count_p2 += if p.is_valid_pt2() { 1 } else { 0 };
        println!("{}, {}", count_p1, count_p2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_passport() {
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
    fn invalid_passport() {
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

    #[test]
    fn test_field_validation() {
        let mut p1 = Passport::new();
        p1.parse_line("eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926");
        assert_eq!(p1.validate_fields(), false);
        let mut p2 = Passport::new();
        p2.parse_line("iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946");
        assert_eq!(p2.validate_fields(), false);
        let mut p3 = Passport::new();
        p3.parse_line("hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277");
        assert_eq!(p3.validate_fields(), false);
        let mut p4 = Passport::new();
        p4.parse_line("hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007");
        assert_eq!(p4.validate_fields(), false);

        let mut p5 = Passport::new();
        p5.parse_line("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f");
        assert_eq!(p5.validate_fields(), true);
        let mut p6 = Passport::new();
        p6.parse_line("eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm");
        assert_eq!(p6.validate_fields(), true);
        let mut p7 = Passport::new();
        p7.parse_line("hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022");
        assert_eq!(p7.validate_fields(), true);
        let mut p8 = Passport::new();
        p8.parse_line("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719");
        assert_eq!(p8.validate_fields(), true);
    }

    #[test]
    fn fields_byr() {
        let mut p = Passport::new();
        p.add_field("byr", "1920");
        assert_eq!(p.validate_fields(), true);
        p.add_field("byr", "2002");
        assert_eq!(p.validate_fields(), true);
        p.add_field("byr", "2020");
        assert_eq!(p.validate_fields(), false);
    }

    #[test]
    fn fields_iyr() {
        let mut p = Passport::new();
        p.add_field("iyr", "2010");
        assert_eq!(p.validate_fields(), true);
        p.add_field("iyr", "2020");
        assert_eq!(p.validate_fields(), true);
        p.add_field("iyr", "1958");
        assert_eq!(p.validate_fields(), false);
        p.add_field("iyr", "201020");
        assert_eq!(p.validate_fields(), false);
        p.add_field("iyr", "5");
        assert_eq!(p.validate_fields(), false);
    }

    #[test]
    fn fields_eyr() {
        let mut p = Passport::new();
        p.add_field("eyr", "2020");
        assert_eq!(p.validate_fields(), true);
        p.add_field("eyr", "2030");
        assert_eq!(p.validate_fields(), true);
        p.add_field("eyr", "2018");
        assert_eq!(p.validate_fields(), false);
        p.add_field("eyr", "2019.5");
        assert_eq!(p.validate_fields(), false);
        p.add_field("eyr", "202020");
        assert_eq!(p.validate_fields(), false);
    }

    #[test]
    fn fields_hgt() {
        let mut p = Passport::new();
        p.add_field("hgt", "120cm");
        assert_eq!(p.validate_fields(), false);
        p.add_field("hgt", "150cm");
        assert_eq!(p.validate_fields(), true);
        p.add_field("hgt", "193cm");
        assert_eq!(p.validate_fields(), true);
        p.add_field("hgt", "1193cm");
        assert_eq!(p.validate_fields(), false);
        p.add_field("hgt", "70in");
        assert_eq!(p.validate_fields(), true);
        p.add_field("hgt", "7in");
        assert_eq!(p.validate_fields(), false);
        p.add_field("hgt", "700in");
        assert_eq!(p.validate_fields(), false);
        p.add_field("hgt", "80in");
        assert_eq!(p.validate_fields(), false);
        p.add_field("hgt", "Heyoin!");
        assert_eq!(p.validate_fields(), false);
    }

    #[test]
    fn fields_hcl() {
        let mut p = Passport::new();
        p.add_field("hcl", "#12a5f5");
        assert_eq!(p.validate_fields(), true);
        p.add_field("hcl", "#abcdef");
        assert_eq!(p.validate_fields(), true);
        p.add_field("hcl", "#12345p");
        assert_eq!(p.validate_fields(), false);
        p.add_field("hcl", "#12a5f5126");
        assert_eq!(p.validate_fields(), false);
        p.add_field("hcl", "aaa");
        assert_eq!(p.validate_fields(), false);
    }

    #[test]
    fn fields_ecl() {
        let mut p = Passport::new();
        p.add_field("ecl", "amb");
        assert_eq!(p.validate_fields(), true);
        p.add_field("ecl", "blu");
        assert_eq!(p.validate_fields(), true);
        p.add_field("ecl", "brn");
        assert_eq!(p.validate_fields(), true);
        p.add_field("ecl", "gry");
        assert_eq!(p.validate_fields(), true);
        p.add_field("ecl", "grn");
        assert_eq!(p.validate_fields(), true);
        p.add_field("ecl", "hzl");
        assert_eq!(p.validate_fields(), true);
        p.add_field("ecl", "oth");
        assert_eq!(p.validate_fields(), true);
        p.add_field("ecl", "aaa");
        assert_eq!(p.validate_fields(), false);
    }

    #[test]
    fn fields_pid() {
        let mut p = Passport::new();
        p.add_field("pid", "001524658");
        assert_eq!(p.validate_fields(), true);
        p.add_field("pid", "aaa");
        assert_eq!(p.validate_fields(), false);
        p.add_field("pid", "1526");
        assert_eq!(p.validate_fields(), false);
        p.add_field("pid", "152654852345");
        assert_eq!(p.validate_fields(), false);
    }
}
