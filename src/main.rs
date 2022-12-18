macro_rules! add_day {
    ($a:ident,$b:tt) => {
        use solutions::$a;
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
    add_day!(d6, "d6");
    add_day!(d7, "d7");
    add_day!(d8, "d8");
    add_day!(d9, "d9");
    add_day!(d10, "d10");
    add_day!(d11, "d11");
    add_day!(d12, "d12");
}
