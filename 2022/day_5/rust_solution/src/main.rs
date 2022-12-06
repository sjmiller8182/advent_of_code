use std::fs;

fn chunk_string(row: &str) -> Vec<String> {
    row.chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
}

fn decompose_row(row: &Vec<String>) -> Vec<String> {
    row.iter()
        .map(|s| s.replace(" ", "").replace("[", "").replace("]", ""))
        .collect()
}

fn parse_move_line(line: &str) -> (u32, usize, usize) {
    let line: Vec<&str> = line.split_whitespace().collect();
    let n_crates: u32 = line[1].parse().unwrap();
    let from: usize = line[3].parse().unwrap();
    let to: usize = line[5].parse().unwrap();
    (n_crates, from, to)
}

#[derive(Debug)]
struct Crane {
    crane: Vec<Vec<String>>,
}

impl Crane {
    pub fn from_diagram(diagram: &str) -> Crane {
        let rows: Vec<&str> = diagram.split("\n").collect();
        let mut rows = rows.iter().rev();
        // get the first rwo which has the number of crates
        let columns: Vec<&str> = rows.next().unwrap().split_whitespace().collect();
        let columns: Vec<u32> = columns.iter().map(|s| s.parse().unwrap()).collect();
        let size = *columns.last().unwrap() as usize;

        let mut crane: Vec<Vec<String>> = vec![vec![]; size];
        // iter over rows and insert into columns
        let rows = rows
            .map(|row| chunk_string(row))
            .map(|row| decompose_row(&row));
        for row in rows {
            for (idx, item) in row.iter().enumerate() {
                if !item.is_empty() {
                    crane[idx].push(item.to_string())
                }
            }
        }

        Crane { crane }
    }

    pub fn move_crates(&mut self, n_crates: u32, from: usize, to: usize, as_stack: bool) {
        if as_stack {
            for _ in 0..n_crates {
                let popped = self.crane[from - 1].pop().unwrap();
                self.crane[to - 1].push(popped);
            }
        } else {
            let mut to_move = vec![];
            for _ in 0..n_crates {
                let element = self.crane[from - 1].pop().unwrap();
                to_move.insert(0, element)
            }
            self.crane[to - 1].append(&mut to_move);
        }
    }

    pub fn get_tops(self) -> String {
        self.crane
            .iter()
            .map(|c| c.last().unwrap().to_string())
            .collect()
    }
}

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Missing input file");
    let contents: Vec<&str> = contents.split("\n\n").collect();

    let mut crane = Crane::from_diagram(contents[0]);

    for line in contents[1].lines() {
        let (n_crates, from, to) = parse_move_line(line);
        crane.move_crates(n_crates, from, to, true);
    }

    println!("Part One Answer: {}\n", crane.get_tops());

    let mut crane = Crane::from_diagram(contents[0]);

    for line in contents[1].lines() {
        let (n_crates, from, to) = parse_move_line(line);
        crane.move_crates(n_crates, from, to, false);
    }

    println!("Part Two Answer: {}", crane.get_tops());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_string() {
        let row = "[N] [C]    ";
        let result = chunk_string(row);

        assert_eq!("[N] ", result[0]);
        assert_eq!("[C] ", result[1]);
        assert_eq!("   ", result[2]);

        let row = "    [D]    ";
        let result = chunk_string(row);

        assert_eq!("    ", result[0]);
        assert_eq!("[D] ", result[1]);
        assert_eq!("   ", result[2])
    }

    #[test]
    fn test_decompose_row() {
        let row = vec!["[N] ".to_owned(), "[C] ".to_owned(), "   ".to_owned()];
        let result = decompose_row(&row);

        assert_eq!("N", result[0]);
        assert_eq!("C", result[1]);
        assert_eq!("", result[2])
    }

    #[test]
    fn test_crane() {
        let diagram = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3  ";
        let mut crane = Crane::from_diagram(diagram);
        assert_eq!(
            vec![
                vec!["Z".to_owned(), "N".to_owned()],
                vec!["M".to_owned(), "C".to_owned(), "D".to_owned()],
                vec!["P".to_owned()]
            ],
            crane.crane
        );

        crane.move_crates(1, 2, 1, true);
        assert_eq!(
            vec![
                vec!["Z".to_owned(), "N".to_owned(), "D".to_owned()],
                vec!["M".to_owned(), "C".to_owned()],
                vec!["P".to_owned()]
            ],
            crane.crane
        );

        crane.move_crates(3, 1, 3, true);
        assert_eq!(
            vec![
                vec![],
                vec!["M".to_owned(), "C".to_owned()],
                vec![
                    "P".to_owned(),
                    "D".to_owned(),
                    "N".to_owned(),
                    "Z".to_owned()
                ]
            ],
            crane.crane
        );

        crane.move_crates(2, 2, 1, true);
        assert_eq!(
            vec![
                vec!["C".to_owned(), "M".to_owned()],
                vec![],
                vec![
                    "P".to_owned(),
                    "D".to_owned(),
                    "N".to_owned(),
                    "Z".to_owned()
                ]
            ],
            crane.crane
        );

        crane.move_crates(1, 1, 2, true);
        assert_eq!(
            vec![
                vec!["C".to_owned()],
                vec!["M".to_owned()],
                vec![
                    "P".to_owned(),
                    "D".to_owned(),
                    "N".to_owned(),
                    "Z".to_owned()
                ]
            ],
            crane.crane
        );

        assert_eq!("CMZ".to_owned(), crane.get_tops())
    }

    #[test]
    fn test_parse_move_line() {
        assert_eq!((1, 3, 9), parse_move_line("move 1 from 3 to 9"))
    }
}
