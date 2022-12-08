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
        TreeNode { name, files: vec![], childern: vec![], parent: None }
    }

    pub fn get_size(&self) -> u64 {
        let mut size = 0;
        for f in &self.files {
            size += f.size;
        }
        for child in &self.childern {
            size += child.borrow_mut().get_size();
        }
        size
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
        Line::FileSystemEntity(FileSystemEntity::Dir(DirMetaData { name, contains:vec![] }))
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
        println!("{}", line);
        let line = parse_line(line);
        match line {
            Line::Command(command) => {
                match command.kind {
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
                }
            },
            Line::FileSystemEntity(entity) => {
                match entity {
                    FileSystemEntity::File(file_data) => {
                        current.borrow_mut().files.push(file_data);
                    },
                    FileSystemEntity::Dir(dir_fata) => {}
                }
            },
            Line::Empty => {},
        }
    }
    root

}

fn main() {
    let contents = fs::read_to_string("./sample_input.txt").expect("Missing input file");
    let mut tree = build_tree(&contents);
    println!("{:?}", tree.borrow_mut().get_size())
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
