use dotenv::dotenv;
use log::*;

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
