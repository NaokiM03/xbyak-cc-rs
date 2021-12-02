use std::env;

use xbyak_cc_rs::cc;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let result = cc(&args[1]);
    println!("{}", result);
}
