// https://www.codechef.com/START26C/problems/SMOKE
use std::io::{self, BufRead};
use std::io::Read;

fn get_input() -> String {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();
    stdin.read_until(&mut input).unwrap();
    input
}

fn main() {
    let input = get_input();
    println!("{input}");
}