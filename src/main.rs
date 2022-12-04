use d1::{do_first_part, do_sec_part};

fn main() {
    let fp = do_first_part("./data/d1.txt").unwrap();
    let sp = do_sec_part("./data/d1.txt").unwrap();
    println!("D1:\n\tp1: {}\n\tp2: {}", fp, sp);
}
