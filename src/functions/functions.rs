mod f_1 {
    pub fn arg_to_str(s: &String) -> &str {
        return s.as_str();
    }
}

mod f_2 {
    pub fn strip_first_arg(i: i32) -> i32 {
        return i - 1;
    }
}

use std::env;

fn main() {
    let args = env::args();
    let args_vec: Vec<_> = args.collect();
    let n_args = f_2::strip_first_arg(args_vec.len() as i32);
    println!("{} args", n_args);

    let mut counter: u32 = 0;
    for a in args_vec {
        println!("arg[{}] is {}", counter, f_1::arg_to_str(&a));
        counter += 1;
    }
}
