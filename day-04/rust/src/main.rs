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
}

fn part_one_validator(passport: &Passport) -> bool {
    passport.byr.is_some()
        && passport.iyr.is_some()
        && passport.eyr.is_some()
        && passport.hgt.is_some()
        && passport.hcl.is_some()
        && passport.ecl.is_some()
        && passport.pid.is_some()
}

fn part_two_validator(passport: &Passport) -> bool {
    part_one_validator(passport)
        && passport.byr_valid()
        && passport.iyr_valid()
        && passport.eyr_valid()
        && passport.hgt_valid()
        && passport.hcl_valid()
        && passport.ecl_valid()
        && passport.pid_valid()
}

fn sum_valid_passports(passports: &Vec<Passport>, validator: fn(&Passport) -> bool) -> usize {
    passports.iter().filter(|x| validator(x)).count()
}

fn main() {
    let passports = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
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
        sum_valid_passports(&passports, part_one_validator)
    );
    println!(
        "Part Two: {}",
        sum_valid_passports(&passports, part_two_validator)
    );
}
