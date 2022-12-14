use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug)]
enum Ops {
    Noop,
    Busy(usize),
    Addx(i64),
}
impl FromStr for Ops {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split_whitespace();
        let (op, val) = (splitted.next().unwrap(), splitted.next());
        match op {
            "addx" => Ok(Self::Addx(val.unwrap().parse::<i64>().unwrap())),
            "noop" => Ok(Self::Noop),
            _ => Err("Unknown operation"),
        }
    }
}

#[derive(Debug)]
struct Executor<'a> {
    busy_timer: usize,
    program: &'a [Ops],
    op_ptr: usize,
    x_reg: i64,
}
impl<'a> Executor<'a> {
    fn new(program: &'a [Ops]) -> Executor<'a> {
        Self {
            program,
            busy_timer: 0,
            op_ptr: 0,
            x_reg: 1,
        }
    }
    fn tick(&mut self) {
        if self.busy_timer > 0 {
            self.busy_timer -= 1;
        } else {
            let cmd = &self.program[self.op_ptr];
            self.op_ptr += 1;
            self.exec_cmd(cmd);
        }
    }
    fn exec_cmd(&mut self, cmd: &Ops) {
        match cmd {
            Ops::Noop => self.exec_noop(),
            Ops::Busy(t) => self.exec_busy(*t),
            Ops::Addx(v) => self.exec_addx(*v),
        }
    }
    fn exec_noop(&mut self) {
        ()
    }
    fn exec_addx(&mut self, val: i64) {
        self.x_reg += val;
    }
    fn exec_busy(&mut self, time: usize) {
        self.busy_timer = time;
    }
}

fn parse(fpath: &str) -> Vec<Ops> {
    let f = File::open(fpath).expect("failed to open file");

    BufReader::new(f)
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| Ops::from_str(&l).unwrap())
        .map(|o| match o {
            Ops::Addx(v) => vec![Ops::Busy(0), Ops::Addx(v)],
            op => vec![op],
        })
        .flatten()
        .collect()
}

pub fn do_first_part(fpath: &str) -> i64 {
    let measurement_time = HashSet::<usize>::from_iter([20, 60, 100, 140, 180, 220]);
    let mut res = 0;
    let ops = parse(fpath);
    let mut executor = Executor::new(&ops[..]);
    for step in 1..=220 {
        if measurement_time.contains(&step) {
            res += executor.x_reg * i64::try_from(step).unwrap();
        }

        executor.tick();
    }
    res
}
pub fn do_sec_part(fpath: &str) -> String {
    let measurement_time = HashSet::<i64>::from_iter([40, 80, 120, 160, 200, 240]);
    let mut res = String::with_capacity(250);
    let ops = parse(fpath);
    let mut executor = Executor::new(&ops[..]);
    for step in 0..240 {
        let reg = executor.x_reg % 40;
        let row_pos = step % 40;
        if row_pos == reg || row_pos == reg - 1 || row_pos == reg + 1 {
            res.push('#');
        } else {
            res.push('.');
        }
        if measurement_time.contains(&(step + 1)) {
            res.push('\n');
        }
        executor.tick();
    }

    println!("{res}");
    res
}

#[cfg(test)]
mod d8_test {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_first() {
        let res = do_first_part("./src/d10/test.txt");

        assert_eq!(res, 13140);
    }

    #[test]
    fn test_sec() {
        let expected: String = read_to_string("./src/d10/p2_test_res.txt").unwrap();
        let res = do_sec_part("./src/d10/test.txt");

        assert_eq!(res, expected);
    }
}
