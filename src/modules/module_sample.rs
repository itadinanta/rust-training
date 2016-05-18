// this is for the compiler, looks for module3 in module3.rs
mod module3;

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
    
    module1::hello();
    
    // modules are somewhere else
    module3::space();
    
    use module3::*;
    space();
    submodule::space();
}
