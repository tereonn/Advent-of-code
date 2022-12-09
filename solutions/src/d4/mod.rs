use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn check_num_in_interval(num: u32, l_border: u32, r_border: u32) -> bool {
    num >= l_border && num <= r_border
}

fn check_point_in_range(p: (u32, u32), range: (u32, u32)) -> bool {
    check_num_in_interval(p.0, range.0, range.1) && check_num_in_interval(p.1, range.0, range.1)
}

fn check_point_overlaps(p: (u32, u32), range: (u32, u32)) -> bool {
    check_num_in_interval(p.0, range.0, range.1) || check_num_in_interval(p.1, range.0, range.1)
}

pub fn exec<F>(f_path: &str, f: F) -> u32
where
    F: FnMut(((u32, u32), (u32, u32))) -> bool,
{
    let file = File::open(f_path).expect("Failed to open file");

    BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .map(|line| {
            let mut i = line
                .split(",")
                .map(|pair| pair.split("-").map(|v| v.parse::<u32>().unwrap()))
                .map(|mut i| (i.next().unwrap(), i.next().unwrap()));

            (i.next().unwrap(), i.next().unwrap())
        })
        .map(f)
        .filter(|r| *r)
        .count()
        .try_into()
        .unwrap()
}
pub fn do_first_part(file_path: &str) -> u32 {
    let f = |(p1, p2)| check_point_in_range(p1, p2) || check_point_in_range(p2, p1);

    exec(file_path, f)
}

pub fn do_sec_part(file_path: &str) -> u32 {
    let f = |(p1, p2)| check_point_overlaps(p1, p2) || check_point_overlaps(p2, p1);

    exec(file_path, f)
}

#[cfg(test)]
mod d4_test {
    use super::*;

    #[test]
    fn test_first() {
        let res = do_first_part("./src/d4/test.txt");

        assert_eq!(res, 2);
    }

    #[test]
    fn test_sec() {
        let res = do_sec_part("./src/d4/test.txt");

        assert_eq!(res, 4);
    }
}
