use std::collections::HashSet;

const INPUT: &str = include_str!("../../resources/day06_input.txt");

fn main() {
    println!(
        "Day 05 Part 1: {}",
        count_all_questions(&INPUT, count_questions)
    );
    println!(
        "Day 05 Part 2: {}",
        count_all_questions(&INPUT, count_questions2)
    );
}

fn count_questions(input: &str) -> usize {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<HashSet<char>>()
        .len()
}

fn count_questions2(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.trim().chars().collect::<HashSet<char>>())
        .fold(
            "abcdefghijklmnopqrstuvwxyz"
                .chars()
                .collect::<HashSet<char>>(),
            |set1, set2| set1.intersection(&set2).cloned().collect(),
        )
        .len()
}

fn count_all_questions(input: &str, count_questions_fn: fn(&str) -> usize) -> usize {
    input
        .replace(" ", "")
        .split("\n\n")
        .map(count_questions_fn)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_questions() {
        let input = "abcx
        abcy
        abcz";
        let number_of_questions = count_questions(&input);
        assert_eq!(number_of_questions, 6);
    }

    #[test]
    fn test_count_all_questions() {
        let input = "abc

        a
        b
        c
        
        ab
        ac
        
        a
        a
        a
        a
        
        b";
        let count_of_questions = count_all_questions(&input, count_questions);
        assert_eq!(count_of_questions, 11);
    }

    #[test]
    fn test_count_questions2() {
        let input = "abcx
        abcy
        abcz";
        let number_of_questions = count_questions2(&input);
        assert_eq!(number_of_questions, 3);
    }

    #[test]
    fn test_count_all_questions2() {
        let input = "abc

        a
        b
        c
        
        ab
        ac
        
        a
        a
        a
        a
        
        b";
        let count_of_questions = count_all_questions(&input, count_questions2);
        assert_eq!(count_of_questions, 6);
    }
}
