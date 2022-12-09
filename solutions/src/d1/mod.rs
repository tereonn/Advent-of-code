use std::fs::read_to_string;

pub fn do_first_part(file_path: &str) -> u32 {
    parse_cals(file_path)
        .and_then(|cals| {
            cals.into_iter()
                .max()
                .ok_or("Can't find max element".to_string())
        })
        .unwrap()
}

pub fn do_sec_part(file_path: &str) -> u32 {
    parse_cals(file_path)
        .and_then(|mut cals| {
            cals.sort_by(|a, b| b.cmp(a));
            Ok(cals.into_iter().take(3).fold(0, |acc, cal| acc + cal))
        })
        .unwrap()
}

fn parse_cals(file_path: &str) -> Result<Vec<u32>, String> {
    read_to_string(file_path)
        .map_err(|e| format!("{e}"))
        .and_then(|content| {
            Ok(content
                .split("\n\n")
                .map(|cal| {
                    cal.split("\n")
                        .filter(|str| !str.is_empty())
                        .map(|cal_as_string| cal_as_string.parse::<u32>().unwrap())
                        .fold(0, |acc, e| acc + e)
                })
                .collect::<Vec<_>>())
        })
}

#[cfg(test)]
mod d1_tests {
    use super::*;

    #[test]
    fn test_first() {
        let res = do_first_part("./src/d1/test.txt");

        assert_eq!(res, 24000);
    }

    #[test]
    fn test_sec() {
        let res = do_sec_part("./src/d1/test.txt");

        assert_eq!(res, 45000);
    }
}
