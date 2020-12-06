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

fn sum_valid_passports(passports: &[Passport], validator: fn(&Passport) -> bool) -> usize {
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
        for field_data in passport_fields.split_ascii_whitespace() {
            let mut field = field_data.split(':');
            let field_key = field.next().unwrap();
            let field_value = Some(field.next().unwrap().to_string());

            match field_key {
                "byr" => passport.byr = field_value,
                "iyr" => passport.iyr = field_value,
                "eyr" => passport.eyr = field_value,
                "hgt" => passport.hgt = field_value,
                "hcl" => passport.hcl = field_value,
                "ecl" => passport.ecl = field_value,
                "pid" => passport.pid = field_value,
                "cid" => passport.cid = field_value,
                other => panic!("Unrecognized passport field: '{}'", other),
            }
        }
        passport
    })
    .collect::<Vec<Passport>>();

    let part_one_solution = sum_valid_passports(&passports, part_one_validator);
    let part_two_solution = sum_valid_passports(&passports, part_two_validator);

    println!("Part One: {}", part_one_solution);
    println!("Part Two: {}", part_two_solution);

    assert_eq!(part_one_solution, 233);
    assert_eq!(part_two_solution, 111);
}
