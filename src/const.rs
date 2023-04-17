#![allow(unused)]
//常量
pub fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!(
        "{} seconds are {} hours",
        THREE_HOURS_IN_SECONDS,
        THREE_HOURS_IN_SECONDS / 3600
    );
}
