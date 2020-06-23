mod dicts;
mod word;

use dotenv::dotenv;
use log::*;
//use std::fs::File;
//use std::path::Path;
//use std::io::{self, BufRead};
use std::io;
use thiserror::Error;
use itertools::Itertools;

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

    let all_words = dicts::get_dict_words(dict_path).unwrap();
    println!("{} dictionary words readed", all_words.len());

    let blacklist = dicts::get_blacklist_words(blacklist_path).unwrap();
    println!("{} blacklist words readed", blacklist.len());

    //word::is_same_chars_bits("клоун", "лук");

    let set = "бортса";
    let sort_by_len_then_by_str = |w1: &&String, w2: &&String| w1.len().cmp(&w2.len()).then(w1.cmp(w2));
    let words: Vec<&String> = all_words.iter()
                         .filter(|x| x.chars().count() > 2)
                         .sorted_by(sort_by_len_then_by_str)
                         .unique()
                         .filter(|x| word::is_same_chars_bits(set, &x))
                         .collect();

    //let words_copies: Vec<String> = words.iter().map(|&s| s.clone()).collect();
    //words.iter().for_each(|w| println!("{}, len: {}", w, w.len()));
    
    for w in words {
        println!("{}, len: {}", w, w.len());
    }
}
