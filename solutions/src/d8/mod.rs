use std::fs::read_to_string;

pub fn do_first_part(fpath: &str) -> u32 {
    let file = read_to_string(fpath).unwrap();
    let matrix = parse(&file);

    let side_size = matrix.len();
    let mut vis_matrix = vec![vec![false; side_size]; side_size];
    for row_step in 0..side_size {
        let (mut l_t_r, mut r_t_l, mut t_t_d, mut d_t_t) = (None, None, None, None);
        for col_step in 0..side_size {
            let h_l_r = matrix[row_step][col_step];
            let h_r_l = matrix[row_step][side_size - col_step - 1];
            let h_t_d = matrix[col_step][row_step];
            let h_d_t = matrix[side_size - col_step - 1][row_step];

            if l_t_r.is_none() || l_t_r.unwrap() < h_l_r {
                l_t_r = Some(h_l_r);
                vis_matrix[row_step][col_step] |= true;
            }
            if r_t_l.is_none() || r_t_l.unwrap() < h_r_l {
                r_t_l = Some(h_r_l);
                vis_matrix[row_step][side_size - col_step - 1] |= true;
            }
            if t_t_d.is_none() || t_t_d.unwrap() < h_t_d {
                t_t_d = Some(h_t_d);
                vis_matrix[col_step][row_step] |= true;
            }
            if d_t_t.is_none() || d_t_t.unwrap() < h_d_t {
                d_t_t = Some(h_d_t);
                vis_matrix[side_size - col_step - 1][row_step] |= true;
            }
        }
    }

    vis_matrix.iter().fold(0, |acc, r| {
        acc + r
            .iter()
            .map(|v| if *v { 1 } else { 0 })
            .fold(0, |vacc, c| vacc + c)
    })
}

#[derive(Clone, Copy, Debug)]
struct HeightWithIdx {
    height: u32,
    idx: usize,
}
impl HeightWithIdx {
    fn new(height: u32, idx: usize) -> Self {
        Self { height, idx }
    }
}

pub fn do_sec_part(fpath: &str) -> u32 {
    let file = read_to_string(fpath).unwrap();
    let matrix = parse(&file);

    let side_size = matrix.len();
    let mut vis_matrix = vec![vec![1u32; side_size]; side_size];

    let mut ltr_help: Vec<HeightWithIdx> = Vec::with_capacity(side_size);
    let mut rtl_help: Vec<HeightWithIdx> = Vec::with_capacity(side_size);
    let mut ttd_help: Vec<HeightWithIdx> = Vec::with_capacity(side_size);
    let mut dtt_help: Vec<HeightWithIdx> = Vec::with_capacity(side_size);
    for row in 0..side_size {
        ltr_help.clear();
        rtl_help.clear();
        ttd_help.clear();
        dtt_help.clear();
        for col in 0..side_size {
            fill_vision(
                &matrix,
                row,
                col,
                &mut vis_matrix,
                &mut ltr_help,
                |cur, prev| cur - prev,
                |cur, _| cur,
                |_, col| col,
            );

            fill_vision(
                &matrix,
                row,
                side_size - col - 1,
                &mut vis_matrix,
                &mut rtl_help,
                |cur, prev| prev - cur,
                |cur, size| size - cur - 1,
                |_, col| col,
            );

            fill_vision(
                &matrix,
                col,
                row,
                &mut vis_matrix,
                &mut ttd_help,
                |cur, prev| cur - prev,
                |cur, _| cur,
                |row, _| row,
            );

            fill_vision(
                &matrix,
                side_size - col - 1,
                row,
                &mut vis_matrix,
                &mut dtt_help,
                |cur, prev| prev - cur,
                |cur, size| size - cur - 1,
                |row, _| row,
            );
        }
    }

    *vis_matrix
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap()
}

fn fill_vision(
    heights: &Vec<Vec<u32>>,
    row: usize,
    col: usize,
    vis_matrix: &mut Vec<Vec<u32>>,
    help_stack: &mut Vec<HeightWithIdx>,

    dist_calc: fn(usize, usize) -> usize,
    corn_dist_calc: fn(usize, usize) -> usize,
    act_idx_extractor: fn(usize, usize) -> usize,
) {
    let size = heights.len();
    let cur_el_h = heights[row][col];
    let act_idx = act_idx_extractor(row, col);
    if corn_dist_calc(act_idx, size) == 0 {
        vis_matrix[row][col] = 0;
        help_stack.push(HeightWithIdx::new(cur_el_h, act_idx));
    } else {
        while let Some(stack_el) = help_stack.pop() {
            if stack_el.height >= cur_el_h {
                vis_matrix[row][col] *= u32::try_from(dist_calc(act_idx, stack_el.idx)).unwrap();
                help_stack.push(stack_el);

                break;
            }
        }

        if help_stack.is_empty() {
            vis_matrix[row][col] *= u32::try_from(corn_dist_calc(act_idx, size)).unwrap();
        }
        help_stack.push(HeightWithIdx::new(cur_el_h, act_idx));
    }
}

fn parse(file: &str) -> Vec<Vec<u32>> {
    let mut res = Vec::new();
    for line in file.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut v = Vec::new();
        for ch in line.chars() {
            v.push(ch.to_digit(10).unwrap());
        }
        res.push(v);
    }

    res
}

#[cfg(test)]
mod d8_test {
    use super::*;

    #[test]
    fn test_first() {
        let res = do_first_part("./src/d8/test.txt");

        assert_eq!(res, 21);
    }

    #[test]
    fn test_sec() {
        let res = do_sec_part("./src/d8/test.txt");

        assert_eq!(res, 8);
    }
}
