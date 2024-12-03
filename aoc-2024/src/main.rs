mod dec_01;
mod dec_02;
mod util;

fn main() {
    println!("December 1st");
    println!("Part 1: {}", dec_01::run_first(true));
    println!("Part 2: {}", dec_01::run_second(true));

    println!("December 2nd");
    println!("Part 1: {}", dec_02::run_first(true));
    println!("Part 2: {}", dec_02::run_second(true));
}
