use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use regex::Regex;

fn contains_bag(
    enclosing_bag_color: &str,
    contained_bag_color: &str,
    bag_rules: &HashMap<String, HashMap<String, u32>>,
) -> bool {
    bag_rules
        .get(enclosing_bag_color)
        .unwrap()
        .keys()
        .any(|color| {
            contained_bag_color == color || contains_bag(&color, &contained_bag_color, &bag_rules)
        })
}

fn get_bag_count(bag_color: &str, bag_rules: &HashMap<String, HashMap<String, u32>>) -> u32 {
    bag_rules
        .get(bag_color)
        .iter()
        .fold(0, |total_bag_count, bag_color_rules| {
            total_bag_count
                + bag_color_rules.iter().fold(0, |bag_count, items| {
                    let (color, count) = items;
                    bag_count + count + count * get_bag_count(color, &bag_rules)
                })
        })
}

fn main() {
    let input = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap();

    let mut bag_rules = HashMap::new();
    for line in input.lines() {
        let rule = line.parse::<String>().unwrap();
        let (bag_color, rule_data) = rule.split(" bags contain ").collect_tuple().unwrap();

        let mut bag_color_rules = HashMap::new();
        for bag_rule in rule_data.split(", ") {
            let bag_rule = bag_rule.strip_suffix(".").unwrap_or(bag_rule);
            if bag_rule == "no other bags" {
                break;
            }

            let matches = Regex::new(r"(\d+) (.*) bags?")
                .unwrap()
                .captures(&bag_rule)
                .unwrap();
            bag_color_rules.insert(
                matches[2].parse::<String>().unwrap(),
                matches[1].parse::<u32>().unwrap(),
            );
        }
        bag_rules.insert(bag_color.to_string(), bag_color_rules);
    }

    let part_one_solution = bag_rules
        .keys()
        .filter(|bag_color| contains_bag(bag_color, "shiny gold", &bag_rules))
        .count();
    let part_two_solution = get_bag_count("shiny gold", &bag_rules);

    println!("Part One: {}", &part_one_solution);
    println!("Part Two: {}", &part_two_solution);

    assert_eq!(part_one_solution, 289);
    assert_eq!(part_two_solution, 30055);
}
