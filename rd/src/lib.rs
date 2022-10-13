mod node;
pub use node::*;

mod con;
pub use con::*;

pub trait Transmit {
    fn transmit(&self);
}

pub fn clear_console() {
    print!("{}c", 27 as char);
}
