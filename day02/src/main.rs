fn main() {
    let input = aoc2020::get_input_string_from_file().unwrap();
    let password_db = parse_password_db(&input);
    let valid_password_count =
        count_valid_passwords(&password_db, PasswordPolicy::is_valid_password);
    println!("Day 02 Part 1: {}", valid_password_count);
    let valid_password_count =
        count_valid_passwords(&password_db, PasswordPolicy::is_valid_password_part2);
    println!("Day 02 Part 2: {}", valid_password_count);
}

#[derive(PartialEq, Debug)]
struct PasswordPolicy {
    lowest: usize,
    highest: usize,
    letter: String,
}

impl PasswordPolicy {
    fn is_valid_password(&self, password: &str) -> bool {
        let letter_count = password
            .chars()
            .filter(|&c| String::from(c) == self.letter)
            .count();
        letter_count >= self.lowest && letter_count <= self.highest
    }

    fn is_valid_password_part2(&self, password: &str) -> bool {
        let password_low_char = password.chars().nth(self.lowest - 1).unwrap();
        let password_high_char = password.chars().nth(self.highest - 1).unwrap();

        let password_low_char_match = String::from(password_low_char) == self.letter;
        let password_high_char_match = String::from(password_high_char) == self.letter;

        (password_low_char_match && !password_high_char_match)
            || (!password_low_char_match && password_high_char_match)
    }
}

fn count_valid_passwords(
    password_db: &[(PasswordPolicy, String)],
    password_valid_fn: fn(&PasswordPolicy, &str) -> bool,
) -> usize {
    password_db
        .iter()
        .filter(|(policy, password)| password_valid_fn(policy, password.as_str()))
        .count()
}

fn parse_password_policy(policy_input: &str) -> PasswordPolicy {
    let first_split: Vec<&str> = policy_input.split_whitespace().collect();
    let second_split: Vec<&str> = first_split[0].split('-').collect();

    PasswordPolicy {
        lowest: second_split[0].parse().unwrap(),
        highest: second_split[1].parse().unwrap(),
        letter: String::from(first_split[1]),
    }
}

fn parse_password_line(password_policy_line: &str) -> (PasswordPolicy, String) {
    let first_split: Vec<&str> = password_policy_line.split(':').collect();
    let password_policy = parse_password_policy(&first_split[0]);
    let password = String::from(first_split[1]);

    // &password[1..] strips the leading space character
    (password_policy, String::from(&password[1..]))
}

fn parse_password_db(password_db_input: &str) -> Vec<(PasswordPolicy, String)> {
    password_db_input.lines().map(parse_password_line).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_password_policy() {
        let policy_input = "1-3 a";
        let policy = parse_password_policy(&policy_input);
        assert_eq!(policy.lowest, 1);
        assert_eq!(policy.highest, 3);
        assert_eq!(policy.letter, "a");
    }

    #[test]
    fn test_parse_password_line() {
        let password_line_input = "1-3 a: abcde";
        let policy_password_pair = parse_password_line(&password_line_input);

        assert_eq!(
            policy_password_pair.0,
            PasswordPolicy {
                lowest: 1,
                highest: 3,
                letter: String::from("a")
            }
        );
        assert_eq!(policy_password_pair.1, String::from("abcde"));
    }

    #[test]
    fn test_parse_password_database() {
        let password_db_input = "1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc";
        let password_db = parse_password_db(&password_db_input);
        assert_eq!(password_db.len(), 3);
    }

    #[test]
    fn test_valid_passwords_part1() {
        let password_db_input = "1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc";
        let password_db = parse_password_db(&password_db_input);
        let (policy, password) = &password_db[0];
        assert!(policy.is_valid_password(&password));
        let (policy, password) = &password_db[1];
        assert!(!policy.is_valid_password(&password));
        let (policy, password) = &password_db[2];
        assert!(policy.is_valid_password(&password));
    }

    #[test]
    fn test_valid_passwords_part2() {
        let password_db_input = "1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc";
        let password_db = parse_password_db(&password_db_input);
        let (policy, password) = &password_db[0];
        assert!(policy.is_valid_password_part2(&password));
        let (policy, password) = &password_db[1];
        assert!(!policy.is_valid_password_part2(&password));
        let (policy, password) = &password_db[2];
        assert!(!policy.is_valid_password_part2(&password));
    }

    #[test]
    fn test_count_valid_passwords() {
        let password_db_input = "1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc";
        let password_db = parse_password_db(&password_db_input);
        let valid_password_count =
            count_valid_passwords(&password_db, PasswordPolicy::is_valid_password);
        assert_eq!(valid_password_count, 2);
    }
}
