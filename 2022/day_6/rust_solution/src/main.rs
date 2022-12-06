use std::collections::{HashSet, VecDeque};
use std::fs;

enum StartType {
    Packet,
    Message,
}

fn chars_are_unique(queue: &VecDeque<char>, n_size: usize) -> bool {
    let set: HashSet<&char> = queue.iter().collect();
    set.len() == n_size
}

fn check_line(line: &str, kind: StartType) -> usize {
    let n_size: usize = match kind {
        StartType::Packet => 4,
        StartType::Message => 14,
    };

    let mut chars = line.chars();
    let mut queue: VecDeque<char> = chars.by_ref().take(n_size).collect();

    let mut starting_char = n_size;

    while !chars_are_unique(&queue, n_size) {
        let next_char = chars.next().unwrap();
        queue.pop_front();
        queue.push_back(next_char);
        starting_char += 1;
    }

    starting_char
}

fn main() {
    let line = fs::read_to_string("../input.txt").expect("Missing input fle");
    let first_marker = check_line(&line, StartType::Packet);
    println!("The first marker character is {}", first_marker);

    let first_marker = check_line(&line, StartType::Message);
    println!("The first message marker character is {}", first_marker);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chars_are_unique() {
        let queue: VecDeque<char> = "mjft".chars().collect();
        assert!(chars_are_unique(&queue, 4));

        let queue: VecDeque<char> = "mjfj".chars().collect();
        assert!(!chars_are_unique(&queue, 4));
    }

    #[test]
    fn test_check_line() {
        let line = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let first_marker = check_line(line, StartType::Packet);
        assert_eq!(5, first_marker);

        let line = "nppdvjthqldpwncqszvftbrmjlhg";
        let first_marker = check_line(line, StartType::Packet);
        assert_eq!(6, first_marker);

        let line = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let first_marker = check_line(line, StartType::Packet);
        assert_eq!(10, first_marker);

        let line = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let first_marker = check_line(line, StartType::Packet);
        assert_eq!(11, first_marker);

        let line = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let first_marker = check_line(line, StartType::Message);
        assert_eq!(23, first_marker);

        let line = "nppdvjthqldpwncqszvftbrmjlhg";
        let first_marker = check_line(line, StartType::Message);
        assert_eq!(23, first_marker);

        let line = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let first_marker = check_line(line, StartType::Message);
        assert_eq!(29, first_marker);

        let line = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let first_marker = check_line(line, StartType::Message);
        assert_eq!(26, first_marker);
    }
}
