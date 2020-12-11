use regex::Regex;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::prelude::*;

#[derive(Eq)]
struct Bag {
    color: String,
    owned_by: Vec<String>,
    contains: HashMap<String, usize>,
}

impl Hash for Bag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.color.hash(state);
    }
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        return self.color == other.color;
    }
}

impl Bag {
    fn new(color: &str) -> Self {
        Bag {
            color: color.to_string(),
            owned_by: Vec::new(),
            contains: HashMap::new(),
        }
    }

    fn add_owner(&mut self, owner: &str) {
        self.owned_by.push(owner.to_string());
    }

    fn add_content(&mut self, target: &str, amount: usize) {
        self.contains.insert(target.to_string(), amount);
    }
}

struct BagCollection {
    bags: HashMap<String, Bag>,
}

impl BagCollection {
    fn new() -> Self {
        BagCollection {
            bags: HashMap::new(),
        }
    }

    fn get(&self, color: &str) -> Option<&Bag> {
        return self.bags.get(color);
    }

    fn get_mut(&mut self, color: &str) -> Option<&mut Bag> {
        return self.bags.get_mut(color);
    }

    fn add(&mut self, color: &str) -> Option<&Bag> {
        let bag = Bag::new(color);
        self.bags.insert(color.to_string(), bag);
        return self.get(color);
    }

    fn has(&self, color: &str) -> bool {
        return self.bags.contains_key(color);
    }

    fn ensure_exists(&mut self, color: &str) {
        if !self.has(color) {
            self.add(color);
        }
    }

    fn add_rule(&mut self, owner_color: &str, target_color: &str, amount: usize) {
        self.ensure_exists(owner_color);
        self.ensure_exists(target_color);

        if let Some(ref mut owner) = self.get_mut(owner_color) {
            owner.add_content(target_color, amount.clone());
        }
        if let Some(ref mut target) = self.get_mut(target_color) {
            target.add_owner(owner_color);
        }
    }

    fn get_all_parents(&self, color: &str) -> Vec<&str> {
        let mut res: Vec<&str> = Vec::new();
        if let Some(root) = self.get(color) {
            let mut stack: Vec<&str> = root.owned_by.iter().map(|s| s as &str).collect();

            while !stack.is_empty() {
                let color = stack.pop().unwrap();
                res.push(color);
                if let Some(bag) = self.get(color) {
                    for child in &bag.owned_by {
                        if !res.contains(&child.as_str()) && !stack.contains(&child.as_str()) {
                            stack.push(child);
                        }
                    }
                }
            }
        }
        return res;
    }

    fn get_content_size(&self, root_color: &str) -> usize {
        let mut res = 0;
        let mut stack: Vec<(&str, usize)> = vec![(root_color, 1)];

        while !stack.is_empty() {
            let (color, amount) = stack.pop().unwrap();
            res += amount;
            if let Some(bag) = self.get(color) {
                for (child, child_amount) in &bag.contains {
                    stack.push((child, child_amount * amount));
                }
            }
        }

        return res -1;
    }
}

fn get_input() -> BagCollection {
    let mut collection = BagCollection::new();

    let stdin = io::stdin();
    let line_parse = Regex::new(r"^(?P<color>.*) bags contain (?P<rules>.*).").unwrap();
    let rule_parse = Regex::new(r"(?P<amount>\d+) (?P<color>.*) bag").unwrap();
    for line in stdin.lock().lines() {
        if let Some(input) = line_parse.captures(&line.unwrap()) {
            let owner = input.name("color").unwrap().as_str();

            let ruleline: &str = input.name("rules").unwrap().as_str();

            for rulestr in ruleline.split(',').collect::<Vec<&str>>() {
                if let Some(rule) = rule_parse.captures(rulestr) {
                    let amount: usize = rule.name("amount").unwrap().as_str().parse().unwrap();
                    let target: &str = rule.name("color").unwrap().as_str();

                    collection.add_rule(&owner, &target, amount);
                }
            }
        } else {
            break;
        }
    }

    return collection;
}

fn part1(collection: &BagCollection) {
    let parent_count = collection.get_all_parents("shiny gold").len();
    println!("Shiny gold has {} parents", parent_count);
}

fn part2(collection: &BagCollection) {
    let top_bag = "shiny gold";
    let bag_count = collection.get_content_size(top_bag);

    println!("A {} bag contains {} bags", top_bag, bag_count);
}

fn main() {
    println!("Enter bag graph...");
    let collection = get_input();
    //part1(&collection);
    part2(&collection);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bag() {
        let mut b = Bag::new("t");
        b.add_owner("a");
        b.add_owner("b");
        assert_eq!(b.owned_by, vec!["a", "b"]);
    }

    #[test]
    fn collection() {
        let mut c = BagCollection::new();
        c.add("a");
        c.add("b");
        c.add("c");
        c.add_rule("a", "b", 1);
        c.add_rule("b", "c", 2);
        if let Some(bag) = c.get("b") {
            assert_eq!(bag.color, "b");
            assert_eq!(bag.owned_by, vec!["a"]);
            assert_eq!(bag.contains[&"c"], 2);
        } else {
            unreachable!()
        }
        assert_eq!(c.get_all_parents("c"), vec!["b", "a"]);
    }
}
