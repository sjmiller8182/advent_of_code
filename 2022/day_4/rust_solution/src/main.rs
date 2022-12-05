use std::fs;

fn pair_contained(left_pair: (u32, u32), right_pair: (u32, u32)) -> bool {
    (left_pair.0 >= right_pair.0 && left_pair.1 <= right_pair.1)
        || (right_pair.0 >= left_pair.0 && right_pair.1 <= left_pair.1)
}

fn pair_partial_overlap(left_pair: (u32, u32), right_pair: (u32, u32)) -> bool {
    (right_pair.0 <= left_pair.1 && right_pair.0 >= left_pair.0)
        || (right_pair.1 <= left_pair.1 && right_pair.1 >= left_pair.0)
        || (left_pair.1 <= right_pair.1 && left_pair.1 >= right_pair.0)
        || (left_pair.0 <= right_pair.1 && left_pair.0 >= right_pair.0)
}

fn decompose_line(line: &str) -> ((u32, u32), (u32, u32)) {
    let line: Vec<u32> = line
        .split(&['-', ','])
        .map(|str| str.parse().unwrap())
        .collect();
    ((line[0], line[1]), (line[2], line[3]))
}

fn count_contained_pairs(content: &str) -> u32 {
    content
        .lines()
        .map(|line| decompose_line(line))
        .map(|pairs| pair_contained(pairs.0, pairs.1) as u32)
        .sum()
}

fn count_partial_pairs(content: &str) -> u32 {
    content
        .lines()
        .map(|line| decompose_line(line))
        .map(|pairs| pair_partial_overlap(pairs.0, pairs.1) as u32)
        .sum()
}

fn main() {
    let content = fs::read_to_string("../input").expect("Missing input file.");
    let contained_pairs = count_contained_pairs(&content);
    println!("The number of contained pairs is {}", contained_pairs);
    let partial_pairs = count_partial_pairs(&content);
    println!(
        "The number of partially overlapping pairs is {}",
        partial_pairs
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_pair_contained() {
        assert!(pair_contained((3, 7), (2, 8)));
        assert!(pair_contained((2, 8), (3, 7)));
        assert!(pair_contained((3, 7), (7, 7)));
        assert!(pair_contained((6, 6), (4, 6)));

        assert!(!pair_contained((2, 4), (6, 8)));
        assert!(!pair_contained((2, 3), (4, 5)));
        assert!(!pair_contained((5, 7), (7, 9)));
        assert!(!pair_contained((2, 6), (4, 8)))
    }

    #[test]
    fn test_decompose_line() {
        assert_eq!(((2, 6), (4, 8)), decompose_line("2-6,4-8"))
    }
}
