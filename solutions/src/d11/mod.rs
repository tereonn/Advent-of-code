use std::{fmt::Debug, fs::read_to_string};

enum Operand {
    Old,
    Val(u64),
}

enum Token {
    Id(usize),
    StartingItems(Vec<u64>),
    Operation(Box<dyn Fn(u64) -> u64>),
    Test((Box<dyn Fn(u64) -> bool>, u64)),
    IfTestTrue(usize),
    IfTestFalse(usize),
    Unknown,
    UnknownOp,
}
impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(v) => write!(f, "Id({v})"),
            Self::Unknown => write!(f, "Unknown"),
            Self::Operation(_) => write!(f, "Operation closure"),
            Self::Test(_) => write!(f, "Test closure"),
            Self::StartingItems(v) => write!(f, "StartingItems({v:?})"),
            Self::IfTestTrue(v) => write!(f, "IfTestTrue({v})"),
            Self::IfTestFalse(v) => write!(f, "IfTestFalse({v})"),
            Self::UnknownOp => write!(f, "UnknownOperation"),
        }
    }
}

struct Monkey<'a> {
    id: usize,
    items: Vec<u64>,
    op: Box<dyn Fn(u64) -> u64 + 'a>,
    test: Box<dyn Fn(u64) -> bool + 'a>,
    target_test_true: usize,
    target_test_false: usize,
    items_handled: u32,
    divider: u64,
}
impl<'a> Monkey<'a> {
    fn inspect(&mut self, item: u64) -> u64 {
        self.items_handled += 1;
        (self.op)(item)
    }
    fn get_throw_target_id(&self, lvl: u64) -> usize {
        if (self.test)(lvl) {
            self.target_test_true
        } else {
            self.target_test_false
        }
    }
    fn catch(&mut self, item: u64) {
        self.items.push(item);
    }
}

impl<'a> Monkey<'a> {}
struct MonkeyBuilder<'a> {
    id: Option<usize>,
    items: Option<Vec<u64>>,
    op: Option<Box<dyn Fn(u64) -> u64 + 'a>>,
    test: Option<Box<dyn Fn(u64) -> bool + 'a>>,
    target_test_true: Option<usize>,
    target_test_false: Option<usize>,
    divider: Option<u64>,
}
impl<'a> MonkeyBuilder<'a> {
    fn new() -> Self {
        Self {
            id: None,
            items: None,
            op: None,
            test: None,
            target_test_true: None,
            target_test_false: None,
            divider: None,
        }
    }
    fn add_from_token(&mut self, t: Token) -> &mut Self {
        match t {
            Token::Id(v) => self.id = Some(v),
            Token::StartingItems(v) => self.items = Some(v),
            Token::Operation(v) => self.op = Some(v),
            Token::Test(v) => {
                self.test = Some(v.0);
                self.divider = Some(v.1)
            }
            Token::IfTestTrue(v) => self.target_test_true = Some(v),
            Token::IfTestFalse(v) => self.target_test_false = Some(v),
            Token::UnknownOp => (),
            Token::Unknown => (),
        }

        self
    }
    fn build(self) -> Monkey<'a> {
        Monkey {
            id: self.id.unwrap(),
            items: self.items.unwrap(),
            op: self.op.unwrap(),
            test: self.test.unwrap(),
            target_test_true: self.target_test_true.unwrap(),
            target_test_false: self.target_test_false.unwrap(),
            items_handled: 0,
            divider: self.divider.unwrap(),
        }
    }
}
pub fn do_first_part(fpath: &str) -> u32 {
    let mut monkeys = parse(fpath);
    monkeys.sort_by(|a, b| a.id.cmp(&b.id));

    for _ in 1..=20 {
        for idx in 0..monkeys.len() {
            while let Some(item) = monkeys[idx].items.pop() {
                let monkey = &mut monkeys[idx];
                let new_worry = monkey.inspect(item) / 3;
                let target = monkey.get_throw_target_id(new_worry);
                let target_monkey = monkeys.iter_mut().find(|m| m.id == target).unwrap();
                target_monkey.catch(new_worry);
            }
        }
    }

    monkeys.sort_by(|a, b| b.items_handled.cmp(&a.items_handled));

    monkeys
        .iter()
        .take(2)
        .fold(1, |acc, e| acc * e.items_handled)
}

pub fn do_sec_part(fpath: &str) -> u64 {
    let mut monkeys = parse(fpath);
    monkeys.sort_by(|a, b| a.id.cmp(&b.id));
    let divider: u64 = monkeys.iter().map(|m| m.divider).product();
    for _ in 1..=10000 {
        for idx in 0..monkeys.len() {
            while let Some(item) = monkeys[idx].items.pop() {
                let monkey = &mut monkeys[idx];
                let new_worry = monkey.inspect(item) % divider;
                let target = monkey.get_throw_target_id(new_worry);
                let target_monkey = monkeys.iter_mut().find(|m| m.id == target).unwrap();
                target_monkey.catch(new_worry);
            }
        }
    }

    monkeys.sort_by(|a, b| b.items_handled.cmp(&a.items_handled));

    monkeys
        .iter()
        .take(2)
        .fold(1, |acc, e| acc * u64::from(e.items_handled))
}

fn tokens_from_str(s: &str) -> Vec<Token> {
    let mut res: Vec<Token> = Vec::new();
    for line in s.split('\n').filter(|l| !l.is_empty()) {
        let mut splitted_by_divider = line.trim().split(":");

        let (f_part, sec_part) = (
            splitted_by_divider.next().unwrap().trim(),
            splitted_by_divider.next().unwrap().trim(),
        );

        if sec_part.is_empty() {
            let id: usize = f_part
                .split_whitespace()
                .skip(1)
                .next()
                .unwrap()
                .parse()
                .unwrap();

            res.push(Token::Id(id));
        } else {
            let token = match f_part {
                "Starting items" => {
                    let items = sec_part
                        .split(',')
                        .map(|v| v.trim().parse::<u64>().unwrap())
                        .collect();

                    Token::StartingItems(items)
                }
                "Operation" => {
                    let mut splitted = sec_part.split_whitespace().skip(3);
                    let op = splitted.next().unwrap();
                    let val = match splitted.next().unwrap() {
                        "old" => Operand::Old,
                        v => {
                            let val: u64 = v.parse().unwrap();

                            Operand::Val(val)
                        }
                    };

                    match op {
                        "+" => match val {
                            Operand::Val(val) => Token::Operation(Box::new(move |v| v + val)),
                            Operand::Old => Token::Operation(Box::new(move |v| v + v)),
                        },
                        "*" => match val {
                            Operand::Val(val) => Token::Operation(Box::new(move |v| v * val)),
                            Operand::Old => Token::Operation(Box::new(move |v| v * v)),
                        },

                        _ => Token::UnknownOp,
                    }
                }
                "Test" => {
                    let divider: u64 = sec_part
                        .split_whitespace()
                        .skip(2)
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap();

                    Token::Test((Box::new(move |v| v % divider == 0), divider))
                }
                "If true" => {
                    let id: usize = sec_part
                        .split_whitespace()
                        .skip(3)
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap();

                    Token::IfTestTrue(id)
                }
                "If false" => {
                    let id: usize = sec_part
                        .split_whitespace()
                        .skip(3)
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap();

                    Token::IfTestFalse(id)
                }
                _ => Token::Unknown,
            };

            res.push(token);
        }
    }

    res
}

fn parse(fpath: &str) -> Vec<Monkey> {
    read_to_string(fpath)
        .unwrap()
        .split("\n\n")
        .map(tokens_from_str)
        .map(|monkey_tokens| {
            let mut builder = MonkeyBuilder::new();
            for token in monkey_tokens {
                builder.add_from_token(token);
            }

            builder.build()
        })
        .collect()
}

#[cfg(test)]
mod d11_test {
    use super::*;

    #[test]
    fn test_first() {
        let res = do_first_part("./src/d11/test.txt");

        assert_eq!(res, 10605);
    }

    #[test]
    fn test_sec() {
        let res = do_sec_part("./src/d11/test.txt");

        assert_eq!(res, 2713310158);
    }
}
