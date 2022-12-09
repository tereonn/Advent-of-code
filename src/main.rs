use solutions::{d1, d2, d3, d4};

fn main() {
    let fp = d1::do_first_part("./data/d1.txt").unwrap();
    let sp = d1::do_sec_part("./data/d1.txt").unwrap();
    println!("D1:\n\tp1: {}\n\tp2: {}", fp, sp);

    let fp = d2::do_first_part("./data/d2.txt");
    let sp = d2::do_sec_part("./data/d2.txt");
    println!("D2:\n\tp1: {}\n\tp2: {}", fp, sp);

    let fp = d3::do_first_part("./data/d3.txt");
    let sp = d3::do_sec_part("./data/d3.txt");
    println!("D3:\n\tp1: {}\n\tp2: {}", fp, sp);

    let fp = d4::do_first_part("./data/d4.txt");
    let sp = d4::do_sec_part("./data/d4.txt");
    println!("D3:\n\tp1: {}\n\tp2: {}", fp, sp);
}
