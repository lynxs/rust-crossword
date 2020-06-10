mod dicts;

use dotenv::dotenv;
use log::*;
//use std::fs::File;
//use std::path::Path;
//use std::io::{self, BufRead};
use std::io;
use thiserror::Error;

//use crate::dicts;

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
    dicts::test();

    let words = dicts::get_dict_words(dict_path).unwrap();
    println!("{} dictionary words readed", words.len());

    let blacklist = dicts::get_blacklist_words(blacklist_path).unwrap();
    println!("{} blacklist words readed", blacklist.len());
}
