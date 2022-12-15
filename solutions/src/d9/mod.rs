use std::{
    cmp::Ordering,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn do_sec_part(fpath: &str) -> usize {
    let file = File::open(fpath).expect("Failed to open file");
    let mut path: HashSet<String> = HashSet::new();
    let mut head_position: Point = Point(0, 0);
    let mut tails: Vec<Point> = vec![Point(0, 0); 9];

    path.insert(Point(0, 0).as_string());

    for d in BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| Direction::from_str(&l).unwrap())
    {
        let dist: usize = d.get_dist().try_into().unwrap();
        for _ in 0..dist {
            head_position.move_step(&d);
            let mut h = &head_position;
            for tail_position in tails.iter_mut() {
                if !h.is_adjacent(&tail_position) {
                    let head_direct = h.get_relative_position(&tail_position);
                    tail_position.move_by_pos(head_direct);
                }
                h = &*tail_position;
            }
            path.insert(h.as_string());
        }
    }
    path.len()
}

#[derive(Clone)]
struct Point(i32, i32);
impl Point {
    fn move_step(&mut self, dir: &Direction) {
        match dir {
            Direction::Up(_) => self.1 += 1,
            Direction::Down(_) => self.1 -= 1,
            Direction::Right(_) => self.0 += 1,
            Direction::Left(_) => self.0 -= 1,
        }
    }
    fn get_relative_position(&self, t_point: &Point) -> RelativePosition {
        match ((self.0.cmp(&t_point.0)), (self.1.cmp(&t_point.1))) {
            (Ordering::Equal, Ordering::Equal) => RelativePosition::Over,
            (Ordering::Equal, Ordering::Greater) => RelativePosition::Up,
            (Ordering::Equal, Ordering::Less) => RelativePosition::Down,
            (Ordering::Greater, Ordering::Equal) => RelativePosition::Right,
            (Ordering::Less, Ordering::Equal) => RelativePosition::Left,
            (Ordering::Greater, Ordering::Greater) => RelativePosition::DiagUR,
            (Ordering::Less, Ordering::Greater) => RelativePosition::DiagUL,
            (Ordering::Greater, Ordering::Less) => RelativePosition::DiagDR,
            (Ordering::Less, Ordering::Less) => RelativePosition::DiagDL,
        }
    }
    fn is_adjacent(&self, t_point: &Point) -> bool {
        (self.0 >= t_point.0 - 1)
            && (self.0 <= t_point.0 + 1)
            && (self.1 >= t_point.1 - 1)
            && (self.1 <= t_point.1 + 1)
    }
    fn move_by_pos(&mut self, d: RelativePosition) {
        match d {
            RelativePosition::Over => (),
            RelativePosition::Up => self.1 += 1,
            RelativePosition::Left => self.0 -= 1,
            RelativePosition::Right => self.0 += 1,
            RelativePosition::Down => self.1 -= 1,
            RelativePosition::DiagUR => {
                self.0 += 1;
                self.1 += 1;
            }
            RelativePosition::DiagUL => {
                self.0 -= 1;
                self.1 += 1;
            }
            RelativePosition::DiagDR => {
                self.0 += 1;
                self.1 -= 1;
            }
            RelativePosition::DiagDL => {
                self.0 -= 1;
                self.1 -= 1;
            }
        }
    }
    fn as_string(&self) -> String {
        format!("{}:{}", self.0, self.1)
    }
}

pub fn do_first_part(fpath: &str) -> usize {
    let file = File::open(fpath).expect("Failed to open file");
    let mut path: HashSet<String> = HashSet::new();
    let mut head_position: Point = Point(0, 0);
    let mut tail_position: Point = Point(0, 0);

    path.insert(Point(0, 0).as_string());

    for d in BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| Direction::from_str(&l).unwrap())
    {
        let dist: usize = d.get_dist().try_into().unwrap();
        for _ in 0..dist {
            head_position.move_step(&d);
            if !head_position.is_adjacent(&tail_position) {
                let head_direct = head_position.get_relative_position(&tail_position);
                tail_position.move_by_pos(head_direct);
                path.insert(tail_position.as_string());
            }
        }
    }
    path.len()
}

#[derive(Debug)]
enum RelativePosition {
    Over,
    Up,
    Left,
    Right,
    Down,
    DiagUR,
    DiagUL,
    DiagDR,
    DiagDL,
}

#[derive(Debug)]
enum Direction {
    Right(i32),
    Up(i32),
    Left(i32),
    Down(i32),
}
impl Direction {
    fn get_dist(&self) -> i32 {
        match self {
            Self::Up(d) => *d,
            Self::Down(d) => *d,
            Self::Left(d) => *d,
            Self::Right(d) => *d,
        }
    }
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(l: &str) -> Result<Self, Self::Err> {
        let mut splitted = l.split_whitespace();
        let (dir, dist) = (
            splitted.next().unwrap(),
            splitted.next().unwrap().parse::<i32>().unwrap(),
        );
        match dir {
            "R" => Ok(Self::Right(dist)),
            "U" => Ok(Self::Up(dist)),
            "L" => Ok(Self::Left(dist)),
            "D" => Ok(Self::Down(dist)),
            _ => Err("Unknown direction"),
        }
    }
}

#[cfg(test)]
mod d9_test {
    use super::*;

    #[test]
    fn test_first() {
        let res = do_first_part("./src/d9/test.txt");

        assert_eq!(res, 13);
    }

    #[test]
    fn test_sec() {
        let res = do_sec_part("./src/d9/test2.txt");

        assert_eq!(res, 36);
    }
}
