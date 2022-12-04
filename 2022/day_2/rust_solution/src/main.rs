use std::fs;

fn string_to_ascii(stringy: &str) -> u32 {
    let as_chars: Vec<char> = stringy.chars().collect();
    let as_chars = as_chars.get(0).unwrap();
    *as_chars as u32
}

fn score(opponent: &str, me: &str) -> u32 {
    let me = string_to_ascii(me);
    let opponent = string_to_ascii(opponent);
    let difference = me - opponent;
    let base_score: u32 = match difference {
        23 => 3,
        22 | 25 => 0,
        _ => 6,
    };
    base_score + (me % 87)
}

fn accumate_scores(content: &str) -> u32 {
    let mut acc_score: u32 = 0;
    for line in content.lines() {
        let mut line = line.split_whitespace();
        let opponent = line.next().unwrap();
        let me = line.next().unwrap();
        acc_score += score(opponent, me);
    }
    acc_score
}

/// A = X = Rock
/// B = Y = Paper
/// C = Z = Scissors
fn main() {
    let content = fs::read_to_string("../input").unwrap();
    let final_score = accumate_scores(&content);
    println!("Got score of {:?}", final_score)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_string_to_ascii() {
        assert_eq!(65, string_to_ascii("A"))
    }

    #[test]
    fn test_score() {
        assert_eq!(8, score("A", "Y"));
        assert_eq!(4, score("A", "X"));
        assert_eq!(3, score("A", "Z"));

        assert_eq!(5, score("B", "Y"));
        assert_eq!(1, score("B", "X"));
        assert_eq!(9, score("B", "Z"));

        assert_eq!(2, score("C", "Y"));
        assert_eq!(7, score("C", "X"));
        assert_eq!(6, score("C", "Z"));
    }
}
