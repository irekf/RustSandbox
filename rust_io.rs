use std::io::prelude::*; // imports all the io related stuff altogether
use std::fs::File; // imports the File struct which holds information about a file
use std::env;

fn main() {

    // Get the second argument in an ugly way... 
    // but it's a way to get to know iterators, Option etc
    let mut args: env::Args = env::args();
    args.next();
    let first_argument = match args.next() {

        Some(argument_value) => {
            println!("Second argument passed: {}", argument_value);
            argument_value
        },

        None => panic!("Only one argument passed!"),

    };

    let path: Path = Path::new(first_argument); // creates a path to our file
    let display = path.display(); // for safely printing paths

    let mut input_file: File = match File::open(&path) {
 
        // Err and Ok are parts of the Result struct, Err contains the cause of an error, Ok has the result
        Err(reason) => panic!("Couldn't open {}, the reason: {}", display, reason.description()),

        Ok(file) => file,

    };

    let mut file_row = String::new();
    match input_file.read_to_string(&mut file_row) {

        Err(why) => panic!("Couldn't read from the file, reason {}", why.description()),

        Ok(_) => print!("{}", file_row),

    }










}
