use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[macro_use]
extern crate bitflags;

bitflags! {
    struct Fields: u8 {
        const BIRTH_YEAR = 1u8 << 0;
        const ISSUE_YEAR = 1u8 << 1;
        const EXPIRATION_YEAR = 1u8 << 2;
        const HEIGHT = 1u8 << 3;
        const HAIR_COLOR = 1u8 << 4;
        const EYE_COLOR = 1u8 << 5;
        const PASSPORT_ID = 1u8 << 6;
        const REQUIRED = 0b01111111;
    }
}

struct PassportParser {
    validate_values: bool,
    fields: Fields,
}

fn number_is_valid(value: &str, min: i32, max: i32) -> bool {
    match value.parse::<i32>() {
        Ok(number) => number >= min && number <= max,
        Err(_) => false,
    }
}

impl PassportParser {
    fn new(validate_values: bool) -> Self {
        Self {
            validate_values,
            fields: Fields::empty(),
        }
    }

    fn birth_year_if_valid(&self, value: &str) -> Fields {
        if !self.validate_values || number_is_valid(value, 1920, 2002) {
            Fields::BIRTH_YEAR
        } else {
            Fields::empty()
        }
    }

    fn issue_year_if_valid(&self, value: &str) -> Fields {
        if !self.validate_values || number_is_valid(value, 2010, 2020) {
            Fields::ISSUE_YEAR
        } else {
            Fields::empty()
        }
    }

    fn expiration_year_if_valid(&self, value: &str) -> Fields {
        if !self.validate_values || number_is_valid(value, 2020, 2030) {
            Fields::EXPIRATION_YEAR
        } else {
            Fields::empty()
        }
    }

    fn height_if_valid(&self, value: &str) -> Fields {
        if !self.validate_values {
            return Fields::HEIGHT;
        }

        let bytes = value.as_bytes();
        if match &bytes[bytes.len() - 2..] {
            b"cm" => number_is_valid(
                value.strip_suffix("cm").expect("Failed to strip cm suffix"),
                150,
                193,
            ),
            b"in" => number_is_valid(
                value.strip_suffix("in").expect("Failed to strip in suffix"),
                59,
                76,
            ),
            _ => false,
        } {
            Fields::HEIGHT
        } else {
            Fields::empty()
        }
    }

    fn hair_color_if_valid(&self, value: &str) -> Fields {
        let bytes = value.as_bytes();
        if !self.validate_values
            || bytes.len() == 7
                && bytes[0] == b'#'
                && bytes[1..]
                    .into_iter()
                    .all(|c| *c >= b'0' && *c <= b'9' || *c >= b'a' && *c <= b'f')
        {
            Fields::HAIR_COLOR
        } else {
            Fields::empty()
        }
    }

    fn eye_color_if_valid(&self, value: &str) -> Fields {
        if !self.validate_values {
            return Fields::EYE_COLOR;
        }

        match value {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Fields::EYE_COLOR,
            _ => Fields::empty(),
        }
    }

    fn passport_id_if_valid(&self, value: &str) -> Fields {
        let bytes = value.as_bytes();
        if !self.validate_values
            || bytes.len() == 9 && bytes.into_iter().all(|b| *b >= b'0' && *b <= b'9')
        {
            Fields::PASSPORT_ID
        } else {
            Fields::empty()
        }
    }

    fn parse_fields(&self, line: &str) -> Fields {
        let mut fields = Fields::empty();
        for token in line.trim().split_ascii_whitespace() {
            let split: Vec<&str> = token.split(":").collect();
            assert!(
                split.len() == 2,
                format!("Expected two fields when splitting [{}]", token)
            );

            fields |= match split[0] {
                "byr" => self.birth_year_if_valid(split[1]),
                "iyr" => self.issue_year_if_valid(split[1]),
                "eyr" => self.expiration_year_if_valid(split[1]),
                "hgt" => self.height_if_valid(split[1]),
                "hcl" => self.hair_color_if_valid(split[1]),
                "ecl" => self.eye_color_if_valid(split[1]),
                "pid" => self.passport_id_if_valid(split[1]),
                "cid" => Fields::empty(),
                _ => panic!("Unexpected field {}", split[0]),
            }
        }

        fields
    }

    fn add_line(&mut self, line: &str) -> Option<Fields> {
        if line.trim().is_empty() {
            let result = Some(self.fields);
            self.fields = Fields::empty();
            return result;
        }

        self.fields |= self.parse_fields(line);
        None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).expect(format!("Failed to open file {}", filename).as_str());
    let mut reader = BufReader::new(file);

    let validate_values = args.len() == 3 && args[2] == "validate";
    let mut parser = PassportParser::new(validate_values);
    let mut valid_passports = 0usize;

    let mut line = String::new();
    loop {
        let bytes = reader.read_line(&mut line).expect("Failed to read line");
        if bytes == 0 {
            break;
        }

        if let Some(fields) = parser.add_line(&line) {
            if fields == Fields::REQUIRED {
                valid_passports += 1;
            }
        }

        line.clear();
    }

    if parser.add_line("").expect("Failed to find last record") == Fields::REQUIRED {
        valid_passports += 1;
    }

    println!("Valid passports: {}", valid_passports);
}
