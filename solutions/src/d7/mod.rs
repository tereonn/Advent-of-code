use std::fs::read_to_string;

#[derive(Debug)]
enum Command<'a> {
    Cd(&'a str),
    Ls,
}
#[derive(Debug)]
enum LineType<'a> {
    Command(Command<'a>),
    InfoFile((u32, &'a str)),
    InfoDir(&'a str),
}
#[derive(Debug)]
enum FsNodeType<'a> {
    Root,
    Dir(&'a str),
    File(&'a str),
}

#[derive(Debug)]
struct FsNode<'a> {
    node_type: FsNodeType<'a>,
    size: u32,
    children: Option<Vec<Box<FsNode<'a>>>>,
}

impl<'a> FsNode<'a> {
    fn new() -> Box<FsNode<'a>> {
        Box::new(Self {
            node_type: FsNodeType::Root,
            size: 0,
            children: Some(Vec::new()),
        })
    }
    fn with_type(node_type: FsNodeType<'a>) -> Box<FsNode<'a>> {
        Box::new(Self {
            node_type,
            size: 0,
            children: Some(Vec::new()),
        })
    }
    fn new_file(name: &'a str, size: u32) -> Box<FsNode<'a>> {
        Box::new(Self {
            node_type: FsNodeType::File(name),
            size,
            children: None,
        })
    }
    fn add_child(&mut self, node: Box<FsNode<'a>>) -> &mut Self {
        self.children.as_mut().map(|c| c.push(node));

        self
    }

    fn calc_size(&mut self) -> &mut Self {
        if self.children.is_none() {
            return self;
        }
        let children: &mut Vec<Box<FsNode>> = self.children.as_mut().unwrap();

        self.size = get_size(children);

        self
    }

    fn get_dir_sum_size_le_than(&self, th_size: u32) -> u32 {
        if self.children.is_none() {
            return self.size;
        }
        let mut total_size = 0;
        for child in self.children.as_ref().unwrap() {
            match child.node_type {
                FsNodeType::Dir(_) => {
                    if child.size < th_size {
                        total_size += child.size;
                    }
                    total_size += child.get_dir_sum_size_le_than(th_size);
                }
                _ => (),
            }
        }

        total_size
    }

    fn get_closest_dir_size_greater_than(&self, th_size: u32, old_val: u32) -> u32 {
        let mut result = old_val;
        if self.children.is_none() {
            return 0;
        }
        for child in self.children.as_ref().unwrap() {
            match child.node_type {
                FsNodeType::Dir(_) => {
                    if child.size < th_size {
                        continue;
                    }
                    let child_closest =
                        child.get_closest_dir_size_greater_than(th_size, child.size);
                    if child_closest < result && child_closest > th_size {
                        result = child_closest;
                    }
                }
                _ => (),
            }
        }

        result
    }
}
fn get_size(children: &mut Vec<Box<FsNode>>) -> u32 {
    let mut total_size = 0u32;
    for child in children {
        match child.node_type {
            FsNodeType::File(_) => {
                total_size += child.size;
            }
            FsNodeType::Dir(_) => {
                let dir_size = get_size(child.children.as_mut().unwrap());
                child.size = dir_size;
                total_size += dir_size;
            }
            _ => (),
        }
    }

    total_size
}
fn parse<'a, 'b>(
    content: &'a str,
    parent: &'b mut Box<FsNode<'a>>,
) -> (&'b mut Box<FsNode<'a>>, &'a str) {
    let mut chunk = content;
    let mut left_idx = 0usize;
    let mut chars = chunk.chars();
    let mut chunk_size = chunk.len();

    while let Some(ch) = chars.next() {
        if ch == '\n' {
            let right_idx = chunk_size - chars.as_str().len();

            let line = &chunk[left_idx..right_idx];
            left_idx = right_idx;

            match get_line_type(line) {
                LineType::InfoFile((fsize, fname)) => {
                    parent.add_child(FsNode::new_file(fname, fsize));
                }
                LineType::Command(cmd) => match cmd {
                    Command::Ls => (),
                    Command::Cd(dir) => {
                        if dir == ".." {
                            return (parent, chars.as_str());
                        }

                        let mut new_dir = FsNode::with_type(FsNodeType::Dir(dir));
                        let (_, rest) = parse(chars.as_str(), &mut new_dir);

                        chunk = rest;
                        chars = rest.chars();
                        left_idx = 0;
                        chunk_size = rest.len();
                        parent.add_child(new_dir);
                    }
                },
                _ => (),
            };
        }
    }

    (parent, chars.as_str())
}

fn get_line_type(line: &str) -> LineType {
    let mut splitted = line.split_whitespace();
    let first_word = splitted.next().unwrap();
    let sec_word = splitted.next().unwrap();

    match first_word {
        "$" => match sec_word {
            "ls" => LineType::Command(Command::Ls),
            _ => {
                let third_word = splitted.next().unwrap();

                LineType::Command(Command::Cd(third_word))
            }
        },
        "dir" => LineType::InfoDir(sec_word),
        _ => LineType::InfoFile((u32::from_str_radix(first_word, 10).unwrap(), sec_word)),
    }
}
pub fn do_first_part(fpath: &str) -> u32 {
    let file = read_to_string(fpath).unwrap();
    let mut root = FsNode::new();
    let _ = parse(&file, &mut root);
    root.calc_size();

    root.children.unwrap()[0].get_dir_sum_size_le_than(100000)
}

pub fn do_sec_part(fpath: &str) -> u32 {
    let file = read_to_string(fpath).unwrap();
    let mut root = FsNode::new();
    let _ = parse(&file, &mut root);
    root.calc_size();

    let free = 70_000_000 - root.size;
    let need_to_free = 30_000_000 - free;

    root.children.unwrap()[0].get_closest_dir_size_greater_than(need_to_free, u32::MAX)
}

#[cfg(test)]
mod d7_test {
    use super::*;

    #[test]
    fn test_first() {
        let res = do_first_part("./src/d7/test.txt");

        assert_eq!(res, 95437);
    }

    #[test]
    fn test_sec() {
        let res = do_sec_part("./src/d7/test.txt");

        assert_eq!(res, 24933642);
    }
}
