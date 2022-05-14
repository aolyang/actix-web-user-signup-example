use std::env;

pub fn native_envs() {
    /// cargo run --example main extra-arg
    /// print ["...path/to/main.rs", "extra-arg"]
    let args: Vec<String> = env::args().collect();

    println!("args, {:?}", args);

    for (k, v) in env::vars() {
        println!("arg {}: {}", k, v)
    }
}