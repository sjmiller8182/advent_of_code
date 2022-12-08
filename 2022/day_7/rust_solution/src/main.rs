use std::fs;

#[derive(Debug, PartialEq)]
enum CommandKind {
    ChangeDir,
    ListDir,
    Unk,
}

#[derive(Debug, PartialEq)]
struct Command {
    kind: CommandKind,
    argument: Option<String>,
}

#[derive(Debug, PartialEq)]
enum FileSystemEntity {
    File(FileMetaData),
    Dir(DirMetaData),
}

#[derive(Debug, PartialEq)]
struct FileMetaData {
    pub name: String,
    pub size: u64,
}

#[derive(Debug, PartialEq)]
struct DirMetaData {
    pub name: String,
    pub contains: Vec<FileSystemEntity>,
}

#[derive(Debug, PartialEq)]
enum Line {
    Command(Command),
    FileSystemEntity(FileSystemEntity),
    Empty,
}

fn starts_with_number(line: &str) -> bool {
    line.chars().nth(0).unwrap().is_numeric()
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct TreeNode {
    pub name: String,
    pub files: Vec<FileMetaData>,
    pub childern: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(name: String) -> TreeNode {
        TreeNode {
            name,
            files: vec![],
            childern: vec![],
            parent: None,
        }
    }

    pub fn get_size(&self) -> (u64, String) {
        let mut size = 0;
        for f in &self.files {
            size += f.size;
        }
        for child in &self.childern {
            let (s, _) = child.borrow_mut().get_size();
            size += s
            //size += child.borrow_mut().get_size();
        }
        (size, self.name.clone())
    }
}

fn parse_line(line: &str) -> Line {
    let split_line: Vec<_> = line.split_whitespace().collect();

    if line.starts_with('$') {
        let kind = match split_line.get(1).unwrap() {
            &"cd" => CommandKind::ChangeDir,
            &"ls" => CommandKind::ListDir,
            _ => CommandKind::Unk,
        };
        let argument = split_line.get(2).map(|s| s.to_string());

        Line::Command(Command { kind, argument })
    } else if line.starts_with("dir") {
        let name = split_line.last().unwrap().to_string();
        Line::FileSystemEntity(FileSystemEntity::Dir(DirMetaData {
            name,
            contains: vec![],
        }))
    } else if starts_with_number(&line) {
        let size = split_line.get(0).unwrap().parse().unwrap();
        let name = split_line.get(1).unwrap().to_string();
        Line::FileSystemEntity(FileSystemEntity::File(FileMetaData { name, size }))
    } else {
        Line::Empty
    }
}

fn build_tree(contents: &str) -> Rc<RefCell<TreeNode>> {
    let mut lines = contents.lines();

    // get the initial cd / command
    let initial = parse_line(lines.next().unwrap());

    //Line::Command(intial) = parse_line(lines.next().unwrap())
    let root = Rc::new(RefCell::new(TreeNode::new("/".to_string())));
    let mut current = Rc::clone(&root);

    for line in lines {
        let line = parse_line(line);
        match line {
            Line::Command(command) => match command.kind {
                CommandKind::ChangeDir => {
                    let name = command.argument.unwrap().clone();
                    if name == ".." {
                        let current_clone = Rc::clone(&current);
                        current = Rc::clone(current_clone.borrow_mut().parent.as_ref().unwrap());
                    } else {
                        let child = Rc::new(RefCell::new(TreeNode::new(name)));
                        current.borrow_mut().childern.push(Rc::clone(&child));
                        {
                            let mut mut_child = child.borrow_mut();
                            mut_child.parent = Some(Rc::clone(&current));
                        }
                        current = child;
                    }
                }
                CommandKind::ListDir | CommandKind::Unk => {}
            },
            Line::FileSystemEntity(entity) => match entity {
                FileSystemEntity::File(file_data) => {
                    current.borrow_mut().files.push(file_data);
                }
                FileSystemEntity::Dir(_) => {}
            },
            Line::Empty => {}
        }
    }
    root
}

fn get_sizes(tree: &Rc<RefCell<TreeNode>>) -> Vec<(u64, String)> {
    let mut sizes = vec![];

    let file_size: u64 = tree.borrow_mut().files.iter().map(|f| f.size).sum();

    if tree.borrow_mut().childern.len() == 0 {
        return vec![(file_size, tree.borrow_mut().name.clone())];
    } else {
        //get size of each child
        for c in &tree.borrow_mut().childern {
            sizes.append(&mut get_sizes(c));
        }
        let current_size: u64 = sizes.iter().map(|t| t.0).sum::<u64>() + file_size;
        sizes.push((current_size, tree.borrow_mut().name.clone()));
        sizes
    }
}

fn get_sizes_2(tree: &Rc<RefCell<TreeNode>>, sizes: &mut Vec<(u64, String)>) -> () {
    sizes.push(tree.borrow_mut().get_size());
    if tree.borrow_mut().childern.len() == 0 {
        return ();
    } else {
        //get size of each child
        for c in &tree.borrow_mut().childern {
            get_sizes_2(c, sizes)
        }
    }
}

fn main() {
    let contents = fs::read_to_string("./sample_input.txt").expect("Missing input file");
    let tree = build_tree(&contents);

    let sizes = get_sizes(&tree);

    let totals: u64 = sizes.iter().filter(|s| s.0 <= 100000).map(|t| t.0).sum();
    println!("Part 1: {}", totals); // 1084134

    let (outer_size, _) = tree.borrow_mut().get_size();
    println!("Outer size \"/\": {}", outer_size);
    let total_free: u64 = 70000000 - outer_size;
    println!("Current free space: {}", total_free);
    let total_needed = 30000000 - total_free;
    println!("Total Needed: {:?}", total_needed);

    //let mut filtered:Vec<_> = sizes.iter().filter(|s| s.0 >= total_needed).collect();
    //let mut filtered: Vec<&(u64, String)> = sizes.iter().filter(|s| s.0 <= 30000000).collect();
    //filtered.sort_by(|a, b| a.0.cmp(&b.0));
    //println!("Part 2: {:?}", filtered);

    let mut sizes: Vec<(u64, String)> = vec![];
    get_sizes_2(&tree, &mut sizes);
    sizes.sort_by(|a, b| a.0.cmp(&b.0));
    //println!("Part 2: {:?}", sizes)

    let mut filtered: Vec<_> = sizes.iter().filter(|s| s.0 >= total_needed).collect();
    filtered.sort_by(|a, b| a.0.cmp(&b.0));
    println!("Part 2: {:?}", filtered);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            Line::Command(Command {
                kind: CommandKind::ChangeDir,
                argument: Some("/".to_string())
            }),
            parse_line("$ cd /")
        );

        assert_eq!(
            Line::Command(Command {
                kind: CommandKind::ChangeDir,
                argument: Some("..".to_string())
            }),
            parse_line("$ cd ..")
        );

        assert_eq!(
            Line::Command(Command {
                kind: CommandKind::ChangeDir,
                argument: Some("f".to_string())
            }),
            parse_line("$ cd f")
        );

        assert_eq!(
            Line::Command(Command {
                kind: CommandKind::ListDir,
                argument: None
            }),
            parse_line("$ ls")
        );

        assert_eq!(
            Line::FileSystemEntity(FileSystemEntity::Dir(DirMetaData {
                name: "d".to_string(),
                contains: vec![]
            })),
            parse_line("dir d")
        );

        assert_eq!(
            Line::FileSystemEntity(FileSystemEntity::File(FileMetaData {
                name: "f".to_string(),
                size: 29116
            })),
            parse_line("29116 f")
        );

        assert_eq!(
            Line::FileSystemEntity(FileSystemEntity::File(FileMetaData {
                name: "f".to_string(),
                size: 0
            })),
            parse_line("0 f")
        )
    }
}
