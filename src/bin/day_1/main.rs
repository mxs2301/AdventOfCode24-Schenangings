// Day 1 is a riddle, where two lists are given.
// The task is to figure out the added up distance between each lists lowest num
mod functions_day1;
use functions_day1::{get_answear_riddle_1, get_answear_riddle_2};
fn main() {
    let x = get_answear_riddle_1();
    let y = get_answear_riddle_2();
    println!("Answear Riddle 1: {}\nAnswear Riddle 2: {}",x, y);
}
