// this line imports the asparagus struct from the vegetables module
// the vegetables module is in the garden module
// the garden module is in the src dir, and is declared a few lines later
use crate::garden::vegetables::Asparagus;

// this code imports the eat_at_restaurant function from the lib.rs file
// syntax is [package name]::[public module or code in lib]
use rust_modules::eat_at_restaurant;

// this line specifies that there is a public module called garden
// either there is a file called garden.rs or 
// there is a directory called garden with a file called mod.rs
// this is the pattern for modules in binaries
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);
    eat_at_restaurant();
}
