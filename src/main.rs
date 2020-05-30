use dotenv::dotenv;
use log::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error(transparent)]
    IoError(#[from] io::Error),
}

fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let key = "DICT_PATH";
    let dict_path = dotenv::var(key).unwrap();
    println!("{}: {}", key, dict_path);

    let blacklist_path = dotenv::var("BLACKLIST_PATH").unwrap();

    //debug!("debugging");

    let words = get_dict_words(dict_path).unwrap();
    println!("{} dictionary words readed", words.len());

    let blacklist = get_blacklist_words(blacklist_path).unwrap();
    println!("{} blacklist words readed", blacklist.len());
}

fn get_dict_words<P>(filename: P) -> Result<Vec<String>, MyError>
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

fn get_blacklist_words<P>(filename: P) -> Result<Vec<String>, MyError>
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
