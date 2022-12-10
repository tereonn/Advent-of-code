use std::{collections::HashSet, fs::read_to_string};

pub fn do_first_part(file_path: &str) -> u32 {
    let file = read_to_string(file_path).unwrap();

    find_marker(&file, 4)
}

pub fn do_sec_part(file_path: &str) -> u32 {
    let file = read_to_string(file_path).unwrap();

    find_marker(&file, 14)
}

pub fn find_marker(data: &String, win_size: usize) -> u32 {
    let mut dict = HashSet::with_capacity(win_size);
    let chars = data.chars().collect::<Vec<_>>();

    for (win_start_idx, win) in chars[..].windows(win_size).enumerate() {
        for ch in win.iter() {
            dict.insert(ch);
        }

        if dict.len() == win_size {
            return u32::try_from(win_start_idx + win_size).unwrap();
        }
        dict.clear();
    }

    0
}

#[cfg(test)]
mod d6_test {
    use super::*;

    #[test]
    fn test_first() {
        let input = vec![
            "bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned(),
            "nppdvjthqldpwncqszvftbrmjlhg".to_owned(),
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_owned(),
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_owned(),
        ];
        let answers = vec![5u32, 6, 10, 11];

        for idx in 0..4 {
            let res = find_marker(&input[idx], 4);

            assert_eq!(res, answers[idx]);
        }
    }

    #[test]
    fn test_sec() {
        let input = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_owned(),
            "bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned(),
            "nppdvjthqldpwncqszvftbrmjlhg".to_owned(),
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_owned(),
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_owned(),
        ];
        let answers = vec![19u32, 23, 23, 29, 26];

        for idx in 0..4 {
            let res = find_marker(&input[idx], 14);

            assert_eq!(res, answers[idx]);
        }
    }
}
