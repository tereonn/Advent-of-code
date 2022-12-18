use std::{collections::HashSet, fs::read_to_string};

#[repr(u8)]
#[derive(Debug)]
enum Direction {
    Up = 1,
    Down = 2,
    Left = 4,
    Right = 8,
}
impl Direction {
    fn test(&self, x: usize, y: usize, f: &Field) -> bool {
        match self {
            Self::Up => x > 0,
            Self::Down => x + 1 < f.rows,
            Self::Left => y > 0,
            Self::Right => y + 1 < f.cols,
        }
    }
    fn encode_dir(self, dir: u8) -> u8 {
        dir | self as u8
    }
    fn extract_dir(dir: u8) -> Option<(u8, Self)> {
        if dir & Direction::Up as u8 != 0 {
            Some((dir ^ Direction::Up as u8, Direction::Up))
        } else if dir & Direction::Down as u8 != 0 {
            Some((dir ^ Direction::Down as u8, Direction::Down))
        } else if dir & Direction::Left as u8 != 0 {
            Some((dir ^ Direction::Left as u8, Direction::Left))
        } else if dir & Direction::Right as u8 != 0 {
            Some((dir ^ Direction::Right as u8, Direction::Right))
        } else {
            None
        }
    }
    fn get_idx_by_move(&self, idx: usize, f: &Field) -> usize {
        let (start_x, start_y) = idx_to_x_y(idx, f);

        match self {
            Self::Up => x_y_to_idx((start_x - 1, start_y), f),
            Self::Down => x_y_to_idx((start_x + 1, start_y), f),
            Self::Left => x_y_to_idx((start_x, start_y - 1), f),
            Self::Right => x_y_to_idx((start_x, start_y + 1), f),
        }
    }
}
impl TryFrom<u8> for Direction {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Up),
            1 => Ok(Self::Down),
            2 => Ok(Self::Left),
            3 => Ok(Self::Right),
            _ => Err("Available range for create Direction is (0..4)"),
        }
    }
}
struct Field {
    rows: usize,
    cols: usize,
}
struct InitialState {
    nodes: Vec<char>,
    field: Field,
}
fn idx_to_x_y(idx: usize, f: &Field) -> (usize, usize) {
    (idx / f.cols, idx % f.cols)
}
fn x_y_to_idx((x, y): (usize, usize), f: &Field) -> usize {
    x * f.cols + y
}
type CanGoChecker = fn(&Direction, &[u32], usize, &Field) -> bool;
fn can_go_up(dir: &Direction, map: &[u32], current_idx: usize, field: &Field) -> bool {
    let target_idx = dir.get_idx_by_move(current_idx, field);

    map[current_idx] >= map[target_idx] || map[target_idx] - map[current_idx] == 1
}
fn can_go_down(dir: &Direction, map: &[u32], current_idx: usize, field: &Field) -> bool {
    let target_idx = dir.get_idx_by_move(current_idx, field);

    map[current_idx] <= map[target_idx] || map[current_idx] - map[target_idx] == 1
}
fn get_possible_moves(idx: usize, f: &Field) -> u8 {
    let mut dir = 0;
    let (x, y) = idx_to_x_y(idx, f);

    for d in 0u8..4 {
        let d = Direction::try_from(d).unwrap();
        if d.test(x, y, f) {
            dir = d.encode_dir(dir);
        }
    }

    dir
}
fn parse(path: &str) -> InitialState {
    let input = read_to_string(path).unwrap();
    let (rows, cols) = {
        let mut by_line = input.trim().split('\n');
        let col_num = by_line.next().unwrap().chars().count();

        (by_line.count() + 1, col_num)
    };
    let nodes: Vec<char> = input.chars().filter(|&c| c != '\n').collect();
    InitialState {
        nodes,
        field: Field { rows, cols },
    }
}
fn dijkstra_path(map: &[u32], field: &Field, start_idx: usize, checker: CanGoChecker) -> Vec<u32> {
    let mut spt_set = HashSet::<usize>::with_capacity(map.len());
    let mut dist = vec![u32::MAX; map.len()];
    dist[start_idx] = 0;
    while let Some(current_idx) = get_min_val(&spt_set, &dist[..]) {
        spt_set.insert(current_idx);
        let mut moves = get_possible_moves(current_idx, field);

        while let Some((mv, dir)) = Direction::extract_dir(moves) {
            moves = mv;
            if checker(&dir, map, current_idx, field) {
                let current_dist = dist[current_idx];
                let target_idx = dir.get_idx_by_move(current_idx, field);
                if dist[target_idx] > current_dist + 1 {
                    dist[target_idx] = current_dist + 1;
                }
            }
        }
    }

    dist
}
fn get_min_val(spt_set: &HashSet<usize>, dist: &[u32]) -> Option<usize> {
    let mut min_val = u32::MAX;
    let mut min_idx: Option<usize> = None;

    for (idx, &dist) in dist.iter().enumerate() {
        if !spt_set.contains(&idx) && dist < min_val {
            min_val = dist;
            min_idx = Some(idx);
        }
    }

    min_idx
}

pub fn do_first_part(path: &str) -> u32 {
    let InitialState { nodes, field } = parse(path);

    let start_idx = nodes.iter().position(|&c| c == 'S').unwrap();
    let final_idx = nodes.iter().position(|&c| c == 'E').unwrap();
    let normalized: Vec<u32> = nodes
        .into_iter()
        .map(|c| match c {
            'S' => 'a',
            'E' => 'z',
            ch => ch,
        })
        .map(|c| u32::from(c))
        .collect();

    let dist = dijkstra_path(&normalized[..], &field, start_idx, can_go_up);

    dist[final_idx]
}

pub fn do_sec_part(path: &str) -> u32 {
    let InitialState { nodes, field } = parse(path);

    let final_idx = nodes.iter().position(|&c| c == 'E').unwrap();
    let normalized: Vec<u32> = nodes
        .into_iter()
        .map(|c| match c {
            'S' => 'a',
            'E' => 'z',
            ch => ch,
        })
        .map(|c| u32::from(c))
        .collect();

    let dist = dijkstra_path(&normalized[..], &field, final_idx, can_go_down);

    let f = normalized
        .iter()
        .enumerate()
        .filter(|&v| *v.1 == u32::from('a'))
        .map(|(idx, _)| dist[idx])
        .min()
        .unwrap();

    f
}

#[cfg(test)]
mod d12_test {
    use super::*;

    #[test]
    fn test_first() {
        let res = do_first_part("./src/d12/test.txt");

        assert_eq!(res, 31);
    }

    #[test]
    fn test_sec() {
        let res = do_sec_part("./src/d12/test.txt");

        assert_eq!(res, 29);
    }
}
