use std::fs::read_to_string;

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}
impl Move {
    fn new(count: usize, from: usize, to: usize) -> Self {
        Self { count, from, to }
    }
}
type StockpileStack = Vec<char>;

fn parse(file_path: &str) -> (Vec<StockpileStack>, Vec<Move>) {
    let file = read_to_string(file_path).unwrap();
    let mut splitted = file.split("\n\n");
    let (stock, moves) = (splitted.next().unwrap(), splitted.next().unwrap());

    (parse_stock(stock), parse_commands(moves))
}

fn parse_stock(raw: &str) -> Vec<StockpileStack> {
    let mut by_rows = raw
        .split("\n")
        .map(|line| line.chars().skip(1).step_by(4).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    by_rows.pop();
    let stack_num = by_rows[0].len();
    let max_height = stack_num * by_rows.len();
    let mut result: Vec<StockpileStack> = Vec::with_capacity(stack_num);

    for _ in 0..stack_num {
        result.push(Vec::with_capacity(max_height));
    }

    while let Some(row) = by_rows.pop() {
        for (idx, sym) in row.into_iter().enumerate() {
            if sym != ' ' {
                result[idx].push(sym);
            }
        }
    }

    result
}

fn parse_commands(raw: &str) -> Vec<Move> {
    raw.split("\n")
        .map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(
                    line.split_whitespace()
                        .skip(1)
                        .step_by(2)
                        .map(|n| n.parse::<usize>().unwrap()),
                )
            }
        })
        .filter_map(|l| l)
        .map(|mut p| (p.next().unwrap(), p.next().unwrap(), p.next().unwrap()))
        .map(|v| Move::new(v.0, v.1, v.2))
        .collect::<Vec<_>>()
}

pub fn do_first_part(file_path: &str) -> String {
    let (mut stock, cmds) = parse(file_path);
    for cmd in cmds {
        let target_len = stock[cmd.from - 1].len();

        let mut moved = stock[cmd.from - 1]
            .drain(target_len - cmd.count..)
            .rev()
            .collect::<Vec<_>>();

        stock[cmd.to - 1].append(&mut moved);
    }
    stock
        .iter()
        .map(|c| c.iter().rev().next())
        .filter_map(|c| c)
        .collect::<String>()
}

pub fn do_sec_part(file_path: &str) -> String {
    let (mut stock, cmds) = parse(file_path);
    for cmd in cmds {
        let target_len = stock[cmd.from - 1].len();

        let mut moved = stock[cmd.from - 1]
            .drain(target_len - cmd.count..)
            .collect::<Vec<_>>();

        stock[cmd.to - 1].append(&mut moved);
    }

    stock
        .iter()
        .map(|c| c.iter().rev().next())
        .filter_map(|c| c)
        .collect::<String>()
}

#[cfg(test)]
mod d5_test {
    use super::*;

    #[test]
    fn test_first() {
        let res = do_first_part("./src/d5/test.txt");

        assert_eq!(res, "CMZ".to_owned());
    }

    #[test]
    fn test_sec() {
        let res = do_sec_part("./src/d5/test.txt");

        assert_eq!(res, "MCD".to_owned());
    }
}
