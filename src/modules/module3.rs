//! this is `module3`

pub mod submodule {
	/// `module3::submodule::space()`
    pub fn space() {
        println!("module3::submodule::planets!");
    }
}

/// `module3::space()`
pub fn space() {
    println!("module3::planets!");
}
