use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_char_cost(ch: char) -> u32 {
    u32::from(ch) - if ch.is_uppercase() { 38 } else { 96 }
}

pub fn do_first_part(file_path: &str) -> u32 {
    let f = File::open(file_path).expect("Failed to open file");

    BufReader::new(f)
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| {
            let (first_half, sec_half) = (&l[..l.len() / 2], &l[l.len() / 2..]);
            let mut chars_dict = HashSet::new();

            for c in first_half.chars() {
                chars_dict.insert(c);
            }

            sec_half.chars().find(|c| chars_dict.contains(c))
        })
        .fold(0, |acc, ch| acc + get_char_cost(ch))
}

pub fn do_sec_part(file_path: &str) -> u32 {
    let f = File::open(file_path).expect("Failed to open file");

    let r = BufReader::new(f)
        .lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<_>>();

    r[..]
        .chunks(3)
        .map(|chunk| {
            chunk
                .iter()
                .map(|c| {
                    c.chars()
                        .map(|c| get_char_cost(c))
                        .fold(0u64, |acc, c| acc | (1 << c - 1))
                })
                .fold(u64::MAX, |acc, c| acc & c)
        })
        .fold(0, |acc, v| acc + v.trailing_zeros() + 1)
}

#[cfg(test)]
mod d3_test {
    use super::*;

    #[test]
    fn test_first() {
        let res = do_first_part("./src/d3/test.txt");

        assert_eq!(res, 157);
    }

    #[test]
    fn test_sec() {
        let res = do_sec_part("./src/d3/test.txt");

        assert_eq!(res, 70);
    }
}
