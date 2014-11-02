extern crate getopts;

extern crate coreutils;

use std::os;


fn strip_suffix(string: &str, suffix: &str) -> String {
    if !string.ends_with(suffix) {
        return string.to_string();
    }

    return string.slice_to(string.len() - suffix.len()).to_string();
}


fn main() {
    let args = os::args();
    let program = args[0].clone();
    let program_usage = format!("{} NAME [SUFFIX]", program);

    let opts = [
        getopts::optflag("h", "help", "display this help and exit"),
        getopts::optflag("V", "version", "output version information and exit"),
    ];

    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => { m },
        Err(f) => {
            println!("{}", f.to_string());
            println!("-------------------------------");
            println!("Usage: {}", getopts::usage(program_usage.as_slice(), opts));
            return;
        },
    };

    if matches.opt_present("help") {
        println!("Usage: {}", getopts::usage(program_usage.as_slice(), opts));
        return;
    }

    if matches.opt_present("version") {
        println!("{}: {}", program, coreutils::version());
        return;
    }

    let num_args = args.tail().len();
    if num_args < 1 || num_args > 2 {
        println!("Invalid number of arguments.");
        println!("-------------------------------");
        println!("Usage: {}", getopts::usage(program_usage.as_slice(), opts));
        return;
    }

    let mut should_strip = None;
    if num_args == 2 {
        should_strip = Some(args[2].as_slice());
    }

    let file = Path::new(args[1].as_slice());
    let basename = match file.filename_str() {
        None => file.as_str().unwrap(),
        Some(name) => name,
    };

    match should_strip {
        None => println!("{}", basename),
        Some(suffix) => println!("{}", strip_suffix(basename, suffix)),
    };
}


