// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.



use std::ops::RangeInclusive;

fn main() {
   let a : [&str; 100]  = ["Are we cool yet?";100];

    if a.len() >= 100 {
        println!("Wow, that's a big array! {:?}", a);
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
