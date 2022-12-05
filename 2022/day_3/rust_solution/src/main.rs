use std::{collections::HashSet, fs};

fn get_priority(chr: char) -> u32 {
    let chr = chr as u32;
    match chr {
        65..=90 => chr % 64 + 26,
        97..=122 => chr % 96,
        _ => 0,
    }
}

fn get_common_in_components(first: &str, second: &str) -> char {
    let first: HashSet<char> = first.chars().collect();
    let second: HashSet<char> = second.chars().collect();
    let intersection: HashSet<_> = first.intersection(&second).collect();
    *intersection.iter().next().unwrap().clone()
}

fn split_string(line: &str) -> (String, String) {
    let half = line.len() / 2;
    let first: String = line.chars().take(half).collect();
    let second: String = line.chars().rev().take(half).collect();
    (first, second)
}

fn accumate_priority(contents: &str) -> u32 {
    contents
        .lines()
        .map(|line| split_string(line))
        .map(|tup| get_common_in_components(&tup.0, &tup.1))
        .map(|chr| get_priority(chr))
        .sum()
}

fn to_sets_of_three(contents: &str) -> Vec<Vec<&str>> {
    let mut sets_of_three = vec![];
    let lines: Vec<&str> = contents.lines().collect();
    for idx in (0..lines.len()).step_by(3) {
        sets_of_three.push(vec![lines[idx], lines[idx + 1], lines[idx + 2]])
    }
    sets_of_three
}

fn common_in_set(sets: Vec<&str>) -> char {
    let mut iterator = sets
        .iter()
        .map(|str| str.chars().collect::<HashSet<char>>());
    let intersection: HashSet<char> = iterator
        .next()
        .map(|set| {
            iterator.fold(set, |set1, set2| {
                set1.intersection(&set2).copied().collect()
            })
        })
        .unwrap();
    intersection.iter().next().unwrap().clone()
}

fn accumulate_badges(contents: &str) -> u32 {
    let sets = to_sets_of_three(contents);
    sets.iter()
        .map(|s| common_in_set(s.to_vec()))
        .map(|chr| get_priority(chr))
        .sum()
}

fn main() {
    let contents = fs::read_to_string("../input").expect("Missing input file");
    let priority_sum = accumate_priority(&contents);
    println!("The sum of common priorities is {}", priority_sum);
    let badge_sum = accumulate_badges(&contents);
    println!("The sum of badge priorities is {}", badge_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_priority() {
        assert_eq!(1, get_priority('a'));
        assert_eq!(26, get_priority('z'));
        assert_eq!(27, get_priority('A'));
        assert_eq!(52, get_priority('Z'))
    }

    #[test]
    fn test_get_common_in_components() {
        assert_eq!(
            'p',
            get_common_in_components("vJrwpWtwJgWr", "hcsFMMfFFhFp")
        )
    }

    #[test]
    fn test_split_string() {
        assert_eq!(
            (
                "vJrwpWtwJgWr".to_owned(),
                "hcsFMMfFFhFp".chars().rev().collect::<String>()
            ),
            split_string("vJrwpWtwJgWrhcsFMMfFFhFp")
        )
    }

    #[test]
    fn test_accumate_priority() {
        let contents = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(157, accumate_priority(contents))
    }
}
