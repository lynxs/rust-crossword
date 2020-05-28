//extern crate dotenv;

use dotenv::dotenv;
//#[allow(unused_imports)]
//use std::env;

//extern crate pretty_env_logger;
#[macro_use] extern crate log;

fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    // for (key, value) in env::vars() {
    //     println!("{}: {}", key, value);
    // }

    let key = "DICT_PATH";
    let value= dotenv::var(key).unwrap();
    println!("{}: {}", key, value);

    debug!("debugging");
}
