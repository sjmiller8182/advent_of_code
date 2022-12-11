use std::{collections::HashSet, fs, vec};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Right(i32),
    Left(i32),
    Up(i32),
    Down(i32),
}

impl Move {
    fn new(direction: &str, value: &str) -> Move {
        let value = value.parse::<i32>().unwrap();
        match direction {
            "D" => Move::Down(value),
            "U" => Move::Up(value),
            "R" => Move::Right(value),
            "L" => Move::Left(value),
            _ => {
                println!("Issue parsing {}, {}", direction, value);
                Move::Up(0)
            }
        }
    }

    fn consume(&self) -> Vec<Move> {
        match self {
            Move::Down(value) => match value {
                0 => vec![],
                _ => vec![Move::Down(1); value.abs() as usize],
            },
            Move::Up(value) => match value {
                0 => vec![],
                _ => vec![Move::Up(1); value.abs() as usize],
            },
            Move::Right(value) => match value {
                0 => vec![],
                _ => vec![Move::Right(1); value.abs() as usize],
            },
            Move::Left(value) => match value {
                0 => vec![],
                _ => vec![Move::Left(1); value.abs() as usize],
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Head {
    x: i32,
    y: i32,
}

impl Head {
    fn new() -> Head {
        Head { x: 0, y: 0 }
    }

    fn from_xy(x: i32, y: i32) -> Head {
        Head { x, y }
    }

    fn move_it(&mut self, steps: Move) {
        let x = self.x;
        let y = self.y;
        match steps {
            Move::Down(n) => {
                let x = x - n;
                self.x = x;
            }
            Move::Up(n) => {
                let x = x + n;
                self.x = x;
            }
            Move::Right(n) => {
                let y = y + n;
                self.y = y;
            }
            Move::Left(n) => {
                let y = y - n;
                self.y = y;
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Tail {
    x: i32,
    y: i32,
    history: Vec<(i32, i32)>,
}

impl Tail {
    fn new() -> Tail {
        Tail {
            x: 0,
            y: 0,
            history: vec![(0, 0)],
        }
    }

    fn from_xy(x: i32, y: i32) -> Tail {
        Tail {
            x,
            y,
            history: vec![(x, y)],
        }
    }

    fn like_head(&self) -> Head {
        Head {
            x: self.x,
            y: self.y,
        }
    }

    fn too_far_away(&self, head: &Head) -> bool {
        [head.x - self.x, head.y - self.y]
            .iter()
            .any(|cord| cord.abs() > 1)
    }

    fn move_it(&mut self, steps: Move) {
        let x = self.x;
        let y = self.y;
        match steps {
            Move::Down(n) => {
                let x = x - n;
                self.x = x;
            }
            Move::Up(n) => {
                let x = x + n;
                self.x = x;
            }
            Move::Right(n) => {
                let y = y + n;
                self.y = y;
            }
            Move::Left(n) => {
                let y = y - n;
                self.y = y;
            }
        }
        self.history.push((self.x, self.y));
    }

    fn follow_it(&mut self, head: &Head) {
        let diff_x = head.x - self.x;
        let diff_y = head.y - self.y;
        if diff_x == 0 {
            if diff_y > 0 {
                self.y += 1
            } else {
                self.y -= 1
            };
        } else {
            if diff_x > 0 {
                self.x += 1
            } else {
                self.x -= 1
            }
        }
        self.history.push((self.x, self.y));
    }

    fn follow(&mut self, head: &Head, direction: Move) {
        match [head.x - self.x, head.y - self.y]
            .iter()
            .any(|cord| cord.abs() == 0)
        {
            // take a normal step here
            true => self.follow_it(&head),
            // take a diag step here
            false => {
                let diff_x = head.x - self.x;
                let diff_y = head.y - self.y;
                // up-right
                if diff_x > 0 && diff_y > 0 {
                    self.x += 1;
                    self.y += 1;
                // up-left
                } else if diff_x < 0 && diff_y > 0 {
                    self.x += -1;
                    self.y += 1;
                }
                // down-right
                if diff_x > 0 && diff_y < 0 {
                    self.x += 1;
                    self.y += -1;
                // down-left
                } else if diff_x < 0 && diff_y < 0 {
                    self.x += -1;
                    self.y += -1;
                }
                self.history.push((self.x, self.y));
            }
        }
    }

    fn unique_loc(&self) -> usize {
        let unique = self.history.iter().collect::<HashSet<_>>();
        unique.len()
    }
}

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(size: usize) -> Grid {
        Grid {
            grid: vec![vec!['.'; size]; size],
        }
    }

    fn update(&mut self, head: &Head, tail: &Tail) {
        let mut new_grid = Grid::new(self.grid.len());
        new_grid.grid[head.x as usize][head.y as usize] = 'H';
        new_grid.grid[tail.x as usize][tail.y as usize] = 'T';
        self.grid = new_grid.grid
    }

    fn update_multitail(&mut self, head: &Head, tail: &Vec<Tail>) {
        let mut new_grid = Grid::new(self.grid.len());
        new_grid.grid[head.x as usize][head.y as usize] = 'H';
        for (idx, tail) in tail.iter().enumerate() {
            new_grid.grid[tail.x as usize][tail.y as usize] =
                char::from_digit((idx + 1) as u32, 10).unwrap();
        }
        self.grid = new_grid.grid
    }

    fn print(&self) {
        println!("");
        let f = self
            .grid
            .iter()
            .rev()
            .map(|vector| vector.iter().collect::<String>())
            .collect::<Vec<_>>();
        for v in f {
            println!("{}", v);
        }
        println!("");
    }
}

fn parse_lines(file_path: &str) -> Vec<Move> {
    fs::read_to_string(file_path)
        .expect("Missing input file")
        .lines()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .map(|tup| Move::new(tup[0], tup[1]))
        .collect::<Vec<_>>()
}

fn main() {
    let mut grid = Grid::new(6);
    let mut head = Head::new();
    let mut tail = Tail::new();
    grid.update(&head, &tail);

    let moves = parse_lines("../input.txt");
    for m in &moves {
        println!("{:?}", m);
        let mut m = m.consume();
        while m.len() > 0 {
            let steps = m.pop().unwrap();
            head.move_it(steps);
            if tail.too_far_away(&head) {
                tail.follow(&head, steps);
            }
            //grid.update(&head, &tail);
            //grid.print();
        }
    }

    println!("Tail location count: {}", tail.unique_loc());

    let mut grid = Grid::new(28);
    let mut head = Head::from_xy(5, 11);
    let mut tails = vec![Tail::from_xy(5, 11); 9];

    for m in moves {
        println!("{:?}", m);
        let mut m = m.consume();
        while m.len() > 0 {
            let steps = m.pop().unwrap();
            head.move_it(steps);
            let tail = tails.get_mut(0).unwrap();
            if tail.too_far_away(&head) {
                tail.follow(&head, steps);
            }
            for i in 1..tails.len() {
                let previous_tail = tails[i - 1].like_head();
                let tail = tails.get_mut(i).unwrap();
                if tail.too_far_away(&previous_tail) {
                    tail.follow(&previous_tail, steps)
                }
            }
            //grid.update_multitail(&head, &tails);
            //grid.print();
        }
    }

    println!(
        "Tail location count: {}",
        tails.last().unwrap().unique_loc()
    );
}

