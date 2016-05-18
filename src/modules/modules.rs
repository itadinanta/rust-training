//! This module has a `main()` function which invokes
//! other functions in external modules

// this is for the compiler, looks for module3 in module3.rs
mod module3;
mod module4;

mod module1 {
    pub fn hello() {
        println!("module1::Hello, ");
    }
}

mod module2 {
    pub fn world() {
        println!("module2::World!");
    }
}

fn main() {
    // local modules
    module1::hello();
    module2::world();

    // modules are somewhere else
    module4::hello();
    module3::space();

    use module3::*;
    space();
    submodule::space();
}
