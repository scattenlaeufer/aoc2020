use std::fs::File;
use std::io::prelude::*;

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
            "byr" => self.birth_year = Some(value.to_string().parse::<u16>().unwrap()),
            "iyr" => self.issue_year = Some(value.to_string().parse::<u16>().unwrap()),
            "eyr" => self.expiration_year = Some(value.to_string().parse::<u16>().unwrap()),
            "hgt" => self.height = Some(value.to_string()),
            "hcl" => self.hair_color = Some(value.to_string()),
            "ecl" => self.eye_color = Some(value.to_string()),
            "pid" => self.passport_id = Some(value.to_string()),
            "cid" => self.country_id = Some(value.to_string()),
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
        println!("{:?}", passport_str);
        let mut passport = Passport::new();
        for line in passport_str.split("\n") {
            for data_str in line.split(" ") {
                println!("{:?}", data_str);
                let data = data_str.split(":").collect::<Vec<&str>>();
                if data.len() > 1 {
                    passport.add_value(data[0], data[1]);
                }
            }
        }
        //println!("{:#?}", passport);
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
