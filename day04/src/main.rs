fn main() {
    let input = aoc2020::get_input_string_from_file().unwrap();

    let number_of_valid_passports_part1 = run_part1(&input);
    println!("Day 04 Part 1: {}", number_of_valid_passports_part1);
    let number_of_valid_passports_part2 = run_part2(&input);
    println!("Day 04 Part 2: {}", number_of_valid_passports_part2);
}

fn run_part1(input: &str) -> usize {
    let passports = parse_passports(&input);
    let valid_passports = passports
        .iter()
        .filter(|p| p.is_valid())
        .collect::<Vec<_>>();
    valid_passports.len()
}

fn run_part2(input: &str) -> usize {
    let passports = parse_passports(&input);
    let valid_passports = passports
        .iter()
        .filter(|p| p.is_valid_part2())
        .collect::<Vec<_>>();
    valid_passports.len()
}

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
    fn default() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_part2(&self) -> bool {
        self.is_byr_valid()
            && self.is_iyr_valid()
            && self.is_eyr_valid()
            && self.is_hgt_valid()
            && self.is_hcl_valid()
            && self.is_ecl_valid()
            && self.is_pid_valid()
    }

    fn is_byr_valid(&self) -> bool {
        match &self.byr {
            Some(byr) => {
                let byr_num = byr.parse::<i32>();
                if byr_num.is_ok() {
                    let num = byr_num.unwrap();
                    num >= 1920 && num <= 2002
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn is_iyr_valid(&self) -> bool {
        match &self.iyr {
            Some(val) => {
                let val_num = val.parse::<i32>();
                if val_num.is_ok() {
                    let num = val_num.unwrap();
                    num >= 2010 && num <= 2020
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn is_eyr_valid(&self) -> bool {
        match &self.eyr {
            Some(val) => {
                let val_num = val.parse::<i32>();
                if val_num.is_ok() {
                    let num = val_num.unwrap();
                    num >= 2020 && num <= 2030
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn is_hgt_valid(&self) -> bool {
        if let Some(val) = &self.hgt {
            if val.ends_with("cm") || val.ends_with("in") {
                let height = val[0..val.len() - 2].parse::<i32>();
                if height.is_ok() {
                    let height = height.unwrap();
                    if val.ends_with("cm") {
                        height >= 150 && height <= 193
                    } else {
                        height >= 59 && height <= 76
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    fn is_hcl_valid(&self) -> bool {
        if let Some(val) = &self.hcl {
            val.chars().count() == 7
                && val.starts_with("#")
                && val[1..]
                    .chars()
                    .all(|c| (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'))
        } else {
            false
        }
    }

    fn is_ecl_valid(&self) -> bool {
        if let Some(val) = &self.ecl {
            val == "amb"
                || val == "blu"
                || val == "brn"
                || val == "gry"
                || val == "grn"
                || val == "hzl"
                || val == "oth"
        } else {
            false
        }
    }

    fn is_pid_valid(&self) -> bool {
        if let Some(val) = &self.pid {
            val.chars().count() == 9
        } else {
            false
        }
    }
}

impl From<&str> for Passport {
    fn from(item: &str) -> Self {
        let mut passport = Passport::default();

        let fields = item.split_ascii_whitespace().collect::<Vec<_>>();

        for field in fields {
            let key_val: Vec<&str> = field.split(":").collect();
            let val = Some(String::from(key_val[1]));
            match key_val[0] {
                "byr" => passport.byr = val,
                "iyr" => passport.iyr = val,
                "eyr" => passport.eyr = val,
                "hgt" => passport.hgt = val,
                "hcl" => passport.hcl = val,
                "ecl" => passport.ecl = val,
                "pid" => passport.pid = val,
                "cid" => passport.cid = val,
                _ => panic!("Unknown field {}", key_val[0]),
            }
        }
        passport
    }
}

fn parse_passports(input: &str) -> Vec<Passport> {
    let lines = input.lines().map(|line| line.trim()).collect::<Vec<_>>();
    let mut passports = vec![];
    let mut passport_string = String::default();
    for line in lines {
        if line.is_empty() {
            passports.push(Passport::from(passport_string.as_str()));
            passport_string = String::default();
        } else {
            passport_string.push_str(line);
            passport_string.push_str("\n");
        }
    }

    if !passport_string.is_empty() {
        passports.push(Passport::from(passport_string.as_str()));
    }
    passports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_passport1() {
        let passport_input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm";
        let passport = Passport::from(passport_input);
        assert_eq!(passport.byr, Some(String::from("1937")));
        assert_eq!(passport.eyr, Some(String::from("2020")));
        assert_eq!(passport.iyr, Some(String::from("2017")));
        assert_eq!(passport.hgt, Some(String::from("183cm")));
        assert_eq!(passport.hcl, Some(String::from("#fffffd")));
        assert_eq!(passport.ecl, Some(String::from("gry")));
        assert_eq!(passport.pid, Some(String::from("860033327")));
        assert_eq!(passport.cid, Some(String::from("147")));
        assert_eq!(passport.is_valid(), true);
    }

    #[test]
    fn test_parse_passport2() {
        let passport_input = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929";
        let passport = Passport::from(passport_input);
        assert_eq!(passport.byr, Some(String::from("1929")));
        assert_eq!(passport.eyr, Some(String::from("2023")));
        assert_eq!(passport.iyr, Some(String::from("2013")));
        assert_eq!(passport.hgt, None);
        assert_eq!(passport.hcl, Some(String::from("#cfa07d")));
        assert_eq!(passport.ecl, Some(String::from("amb")));
        assert_eq!(passport.pid, Some(String::from("028048884")));
        assert_eq!(passport.cid, Some(String::from("350")));
        assert_eq!(passport.is_valid(), false);
    }

    #[test]
    fn test_parse_passport3() {
        let passport_input = "hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm";
        let passport = Passport::from(passport_input);
        assert_eq!(passport.byr, Some(String::from("1931")));
        assert_eq!(passport.eyr, Some(String::from("2024")));
        assert_eq!(passport.iyr, Some(String::from("2013")));
        assert_eq!(passport.hgt, Some(String::from("179cm")));
        assert_eq!(passport.hcl, Some(String::from("#ae17e1")));
        assert_eq!(passport.ecl, Some(String::from("brn")));
        assert_eq!(passport.pid, Some(String::from("760753108")));
        assert_eq!(passport.cid, None);
        assert_eq!(passport.is_valid(), true);
    }

    #[test]
    fn test_parse_passport4() {
        let passport_input = "hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";
        let passport = Passport::from(passport_input);
        assert_eq!(passport.byr, None);
        assert_eq!(passport.eyr, Some(String::from("2025")));
        assert_eq!(passport.iyr, Some(String::from("2011")));
        assert_eq!(passport.hgt, Some(String::from("59in")));
        assert_eq!(passport.hcl, Some(String::from("#cfa07d")));
        assert_eq!(passport.ecl, Some(String::from("brn")));
        assert_eq!(passport.pid, Some(String::from("166559648")));
        assert_eq!(passport.cid, None);
        assert_eq!(passport.is_valid(), false);
    }

    #[test]
    fn test_parse_passports() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm
        
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929
        
        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm
        
        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";

        let passports = parse_passports(&input);
        assert_eq!(passports.len(), 4);
        let valid_passports = passports
            .iter()
            .filter(|p| p.is_valid())
            .collect::<Vec<_>>();

        assert_eq!(valid_passports.len(), 2);
    }

    #[test]
    fn test_part1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm
        
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929
        
        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm
        
        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";

        let num_valid_passports = run_part1(&input);
        assert_eq!(num_valid_passports, 2);
    }

    #[test]
    fn test_part2_invalid_passports() {
        let input = "eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
        
        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946
        
        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
        
        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007";

        let passports = parse_passports(&input);
        assert_eq!(passports.len(), 4);
        assert_eq!(passports[0].is_valid_part2(), false);
        assert_eq!(passports[1].is_valid_part2(), false);
        assert_eq!(passports[2].is_valid_part2(), false);
        assert_eq!(passports[3].is_valid_part2(), false);
    }

    #[test]
    fn test_part2_valid_passports() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f
        
        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
        
        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022
        
        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let passports = parse_passports(&input);
        assert_eq!(passports.len(), 4);
        assert_eq!(passports[0].is_valid_part2(), true);
        assert_eq!(passports[1].is_valid_part2(), true);
        assert_eq!(passports[2].is_valid_part2(), true);
        assert_eq!(passports[3].is_valid_part2(), true);
    }
}
