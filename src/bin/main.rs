use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argv = args[1].to_string();
    let leet_str = rust_leet::leet(argv);
    println!("{}", leet_str);
}
