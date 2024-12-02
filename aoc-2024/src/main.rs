mod dec_02;
mod util;

fn main() {
    println!("December 2nd");
    println!("Part 1: {}", dec_02::dec_02::run_one(true));
    println!("Part 2: {}", dec_02::dec_02::run_two(true));
}
