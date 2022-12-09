use solutions::{d1, d2, d3, d4, d5};
macro_rules! add_day {
    ($a:ident,$b:tt) => {
        let path = format!("./data/{}.txt", $b);
        let fp = $a::do_first_part(&path);
        let sp = $a::do_sec_part(&path);
        println!("{}:\n\tp1: {}\n\tp2: {}", $b, fp, sp);
    };
}
fn main() {
    add_day!(d1, "d1");
    add_day!(d2, "d2");
    add_day!(d3, "d3");
    add_day!(d4, "d4");
    add_day!(d5, "d5");
}
