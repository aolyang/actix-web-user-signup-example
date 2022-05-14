use std::env;
use dotenv::dotenv;

pub fn dot_env_loaded() {
    dotenv().ok();

    let key = "DB.USER";
    // match env::var(key) {
    //     Ok(v) => println!("env car from .env file: {}, {}", key, v),
    //     Err(e) => println!("env car get error, {:?}", e)
    // }
    println!("env car form .env file: {}, {}", key, env::var(key).expect("get env var error"));

    // 如果key不存在于.env（一般在cargo.toml）就会报错；
}
