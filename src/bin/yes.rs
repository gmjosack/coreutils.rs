#![feature(slicing_syntax)]

use std::os;

fn main() {

    let args = os::args();
    let output = match args.len() {
        1 => "y".to_string(),
        _ => args[1..].connect(" "),
    };

    loop {
        println!("{}", output);
    }
}
