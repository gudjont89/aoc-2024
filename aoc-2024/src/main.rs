use std::time::Instant;

#[allow(dead_code)]
mod dec_01;
#[allow(dead_code)]
mod dec_02;
#[allow(dead_code)]
mod dec_03;
#[allow(dead_code)]
mod dec_04;
#[allow(dead_code)]
mod dec_05;
#[allow(dead_code)]
mod dec_06;
#[allow(dead_code)]
mod dec_07;
#[allow(dead_code)]
mod dec_08;
mod dec_09;

mod util;

fn main() {
    // println!("December 1st");
    // println!("Part 1: {}", dec_01::run_first(true));
    // println!("Part 2: {}", dec_01::run_second(true));

    // println!("December 2nd");
    // println!("Part 1: {}", dec_02::run_first(true));
    // println!("Part 2: {}", dec_02::run_second(true));

    // println!("December 3rd");
    // println!("Part 1: {}", dec_03::run_first(true));
    // println!("Part 2: {}", dec_03::run_second(true));

    // println!("December 4th");
    // println!("Part 1: {}", dec_04::run_first(true));
    // println!("Part 2: {}", dec_04::run_second(true));

    // println!("December 5th");
    // println!("Part 1: {}", dec_05::run_first(true));
    // println!("Part 2: {}", dec_05::run_second(true));

    // println!("December 6th");
    // println!("Part 1: {}", dec_06::run_first(true));
    // println!("Part 2: {}", dec_06::run_second(true));

    // println!("December 7th");
    // println!("Part 1: {}", dec_07::run_first(true));
    // println!("Part 2: {}", dec_07::run_second(true));

    // println!("December 8th");
    // println!("Part 1: {}", dec_08::run_first(true));
    // println!("Part 2: {}", dec_08::run_second(true));

    println!("December 9th");
    println!("Part 1: {}", dec_09::run_first(true));
    println!("Part 2: {}", dec_09::run_second(true));
}
