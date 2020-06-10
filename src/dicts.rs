use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::MyError;

pub fn test() {
    println!("dicts test called");
}

pub fn get_dict_words<P>(filename: P) -> Result<Vec<String>, MyError>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename)?;
    let words = lines
        .map(|l| l.unwrap().splitn(2, '#').map(str::to_owned).next())
        .filter_map(|w| w)
        .collect::<Vec<_>>();
    Ok(words)
}

pub fn get_blacklist_words<P>(filename: P) -> Result<Vec<String>, MyError>
where
    P: AsRef<Path>,
{
    Ok(read_lines(filename)?.filter_map(Result::ok).collect())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
