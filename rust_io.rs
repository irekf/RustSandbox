use std::io::prelude::*; // imports all the io related stuff altogether
use std::fs::File; // imports the File struct which holds information about a file
use std::env;
use std::path::Path;

fn main() {

    let args: Vec<String> = env::args().collect(); 
    for idx in 0..args.len() {
        println!("argument #{}: {}", idx, args[idx]);
    }

    let path  = Path::new(&args[1]); // creates a path to our file
    let display = path.display(); // for safely printing paths

    let mut input_file: File = match File::open(&path) {
 
        // Err and Ok are parts of the Result struct, Err contains the cause of an error, Ok has the result
        Err(reason) => panic!("Couldn't open {}, the reason: {}", display, reason),

        Ok(file) => file,

    };

    let mut file_row = String::new();
    match input_file.read_to_string(&mut file_row) {

        Err(why) => panic!("Couldn't read from the file, reason {}", why),

        Ok(_) => print!("{}", file_row),

    }

    let mut my_vector: Vec<i32> = Vec::new();
    my_vector.push(3);








}
