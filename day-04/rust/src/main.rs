use std::env;
use std::fs;
use std::path::Path;

extern crate regex;
use regex::Regex;

#[derive(Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}
impl Passport {
    fn validate_year(year: Option<&String>, min: i64, max: i64) -> bool {
        let year: i64 = year.unwrap().parse().unwrap_or(0);
        year >= min && year <= max
    }
    fn byr_valid(&self) -> bool {
        Passport::validate_year(self.byr.as_ref(), 1920, 2002)
    }
    fn iyr_valid(&self) -> bool {
        Passport::validate_year(self.iyr.as_ref(), 2010, 2020)
    }
    fn eyr_valid(&self) -> bool {
        Passport::validate_year(self.eyr.as_ref(), 2020, 2030)
    }
    fn hcl_valid(&self) -> bool {
        Regex::new(r"^#[a-f0-9]{6}$")
            .unwrap()
            .is_match(&self.hcl.as_ref().unwrap())
    }
    fn ecl_valid(&self) -> bool {
        Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$")
            .unwrap()
            .is_match(&self.ecl.as_ref().unwrap())
    }
    fn pid_valid(&self) -> bool {
        Regex::new(r"^\d{9}$")
            .unwrap()
            .is_match(&self.pid.as_ref().unwrap())
    }
    fn hgt_valid(&self) -> bool {
        let matches = Regex::new(r"^(\d+)(\w+)$")
            .unwrap()
            .captures(&self.hgt.as_ref().unwrap())
            .unwrap();
        let hgt_value: i64 = matches[1].parse().unwrap();
        let hgt_unit: String = matches[2].parse().unwrap();
        match hgt_unit.as_str() {
            "cm" => hgt_value >= 150 && hgt_value <= 193,
            "in" => hgt_value >= 59 && hgt_value <= 76,
            _ => false,
        }
    }
    fn part_one_validator(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }
    fn part_two_validator(&self) -> bool {
        self.part_one_validator()
            && self.byr_valid()
            && self.iyr_valid()
            && self.eyr_valid()
            && self.hgt_valid()
            && self.hcl_valid()
            && self.ecl_valid()
            && self.pid_valid()
    }
}

fn main() {
    let input_filepath = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("input.txt");

    let passports = fs::read_to_string(input_filepath)
        .unwrap()
        .split("\n\n")
        .map(|passport_fields| {
            let mut passport = Passport::default();
            for field in passport_fields.split_ascii_whitespace() {
                let mut key_val = field.split(':');
                match key_val.next().unwrap() {
                    "byr" => passport.byr = Some(key_val.next().unwrap().to_string()),
                    "iyr" => passport.iyr = Some(key_val.next().unwrap().to_string()),
                    "eyr" => passport.eyr = Some(key_val.next().unwrap().to_string()),
                    "hgt" => passport.hgt = Some(key_val.next().unwrap().to_string()),
                    "hcl" => passport.hcl = Some(key_val.next().unwrap().to_string()),
                    "ecl" => passport.ecl = Some(key_val.next().unwrap().to_string()),
                    "pid" => passport.pid = Some(key_val.next().unwrap().to_string()),
                    "cid" => passport.cid = Some(key_val.next().unwrap().to_string()),
                    other => panic!("Unknown passport field: {}", other),
                }
            }
            passport
        })
        .collect::<Vec<Passport>>();

    println!(
        "Part One: {}",
        passports.iter().filter(|x| x.part_one_validator()).count()
    );
    println!(
        "Part One: {}",
        passports.iter().filter(|x| x.part_two_validator()).count()
    );
}
