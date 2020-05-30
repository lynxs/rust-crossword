use dotenv::dotenv;
use log::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    // for (key, value) in env::vars() {
    //     println!("{}: {}", key, value);
    // }

    let key = "DICT_PATH";
    let value = dotenv::var(key).unwrap();
    println!("{}: {}", key, value);

    debug!("debugging");

    if let Ok(lines) = read_lines(value) {
        // for line in lines {
        //     if let Ok(word) = line {
        //         //println!("{}", word);
        //         let v: Vec<&str> = word.splitn(2, '#').collect();
        //         println!("{}", v[0]);
        //     }
        // }
        let words = lines
                    .map(|l| l.unwrap().splitn(2, '#').map(str::to_owned).next())
                    .filter_map(|w| w)
                    .collect::<Vec<_>>();
        println!("{}", words.len());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
