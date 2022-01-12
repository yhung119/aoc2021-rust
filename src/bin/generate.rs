use curl::easy::Easy;
use std::io::stdout;
use std::io::Write;


fn main() {
    // given what day it is, pulls the inputs, creates a small txt, and generate dummy stub code.
    let mut easy = Easy::new();
    easy.url("https://adventofcode.com/2021/day/2/input").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}