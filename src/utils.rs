
use std::io::{self, BufRead};
use std::io::Read;
use std::{fs::File, path::Path};

// Helper from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn words_by_line<'a>(s: &'a str) -> Vec<Vec<&'a str>> {
    s.lines().map(|line| {
        line.split_whitespace().collect()
    }).collect()
}

pub fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

// macro from https://github.com/danvk/aoc2020/blob/master/src/util.rs

#[macro_export]
macro_rules! set(
    { $($key:expr),+ } => {
        {
            let mut m = ::std::collections::HashSet::new();
            $(
                m.insert($key);
            )+
            m
        }
     };
);

#[macro_export]
macro_rules! map(
    { $($key:expr => $val:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $val);
            )+
            m
        }
     };
);