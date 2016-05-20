// Place a function in a module
mod f_1 {
    pub fn arg_to_str(s: &String) -> &str {
        s.as_str()
    }
}

// Place another function in a different module
mod f_2 {
    pub fn strip_first_arg(i: i32) -> i32 {
        const OFFSET: i32 = 1;
        let o = OFFSET;

        // we capture o
        let offset_lambda = || o;

        // return o; // <- can't capture dynamic environment in a fn item
        // we can't capture o in
        fn _offset_fn() -> i32 {
            OFFSET
        }

        // convoluted, just looking for an excuse to call the lambda
        i - offset_lambda()
    }
}

use std::env;
use std::process;

fn c_main(args: &Vec<String>) -> i32 {
    let n_args = f_2::strip_first_arg(args.len() as i32);
    println!("{} args", n_args);

    // enumerate is cute!
    for (counter, a) in args.iter().enumerate() {
        println!("arg[{}] is {}", counter, f_1::arg_to_str(&a));
    }

    // if !valid_args(args) { return 2 }

    if n_args > 0 { 0 } else { 1 }
}

fn main() {
    let args_vec: Vec<_> = env::args().collect();

    let result = c_main(&args_vec);

    process::exit(result);
}
