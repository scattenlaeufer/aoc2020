use std::fs::File;
use std::io::prelude::*;

fn validate_year_range(year_str: &str, min_year: u16, max_year: u16) -> Option<u16> {
    let year = year_str.to_string().parse::<u16>().unwrap_or_default();
    if year >= min_year && year <= max_year {
        Some(year)
    } else {
        None
    }
}

fn validate_birth_year(birth_year: &str) -> Option<u16> {
    //! four digits; at least `1920` and at most `2002`
    validate_year_range(birth_year, 1920, 2002)
}

fn validate_issue_year(issue_year: &str) -> Option<u16> {
    //! four digits; at least `2010` and at most `2020`
    validate_year_range(issue_year, 2010, 2020)
}

fn validate_expiration_year(expiration_year: &str) -> Option<u16> {
    //! four digits; at least `2020` and at most `2030`
    validate_year_range(expiration_year, 2020, 2030)
}

fn validate_height(height: &str) -> Option<String> {
    //! a number followed by either `cm` or `in`:
    //! - If `cm`, the number must be at least `150` and at most `193`
    //! - If `in`, the number must be at least `59` and at most `76`
    if height.len() >= 4 && height.len() <= 5 {
        let unit = &height[height.len() - 2..];
        let value = &height[..height.len() - 2].parse::<u8>().unwrap_or_default();
        match unit {
            "cm" => {
                if value >= &150 && value <= &193 {
                    Some(height.to_string())
                } else {
                    None
                }
            }
            "in" => {
                if value >= &59 && value <= &76 {
                    Some(height.to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        None
    }
}

fn validate_hair_color(hair_color: &str) -> Option<String> {
    //! a `#` followed by exactly six characters `0`-`9` or `a`-`f`
    if hair_color.len() == 7 && hair_color.chars().nth(0) == Some('#') {
        for character in hair_color[1..].chars() {
            if !([
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
            ]
            .contains(&character))
            {
                return None;
            }
        }
        Some(hair_color.to_string())
    } else {
        None
    }
}

fn validate_eye_color(eye_color: &str) -> Option<String> {
    //! exactly one of: `amb` `blu` `brn` `gry` `grn` `hzl` `oth`
    if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&eye_color) {
        Some(eye_color.to_string())
    } else {
        None
    }
}

fn validate_passport_id(passport_id: &str) -> Option<String> {
    //! a nine-digit number, including leading zeroes
    if passport_id.len() == 9 {
        for character in passport_id.chars() {
            if !(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&character)) {
                return None;
            }
        }
        Some(passport_id.to_string())
    } else {
        None
    }
}

fn validate_country_id(country_id: &str) -> Option<String> {
    //! ignored, missing or not
    Some(country_id.to_string())
}

#[derive(Debug)]
struct Passport {
    birth_year: Option<u16>,
    issue_year: Option<u16>,
    expiration_year: Option<u16>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn new() -> Self {
        Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }
}

impl Passport {
    fn add_value(&mut self, key: &str, value: &str) {
        match key {
            "byr" => self.birth_year = validate_birth_year(value),
            "iyr" => self.issue_year = validate_issue_year(value),
            "eyr" => self.expiration_year = validate_expiration_year(value),
            "hgt" => self.height = validate_height(value),
            "hcl" => self.hair_color = validate_hair_color(value),
            "ecl" => self.eye_color = validate_eye_color(value),
            "pid" => self.passport_id = validate_passport_id(value),
            "cid" => self.country_id = validate_country_id(value),
            &_ => (),
        }
    }

    fn loosely_valid(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let string_vec = content.split("\n\n").collect::<Vec<&str>>();
    let mut passport_vec: Vec<Passport> = Vec::new();

    for passport_str in string_vec {
        let mut passport = Passport::new();
        for line in passport_str.split('\n') {
            for data_str in line.split(' ') {
                let data = data_str.split(':').collect::<Vec<&str>>();
                if data.len() > 1 {
                    passport.add_value(data[0], data[1]);
                }
            }
        }
        passport_vec.push(passport);
    }

    println!("number of passports {}", passport_vec.len());
    let mut loosely_valid_passports = 0;
    for passport in passport_vec {
        if passport.loosely_valid() {
            loosely_valid_passports += 1;
        }
    }

    println!("Loosely valid passports: {}", loosely_valid_passports);
}
