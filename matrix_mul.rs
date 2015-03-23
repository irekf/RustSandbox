use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::thread;

fn main() {

    let mat_path = Path::new("big_matrix_1.txt");

    let mat_file = File::open(&mat_path).unwrap();

    let mat_reader = BufReader::new(mat_file);

    let mut mat_line_iterator = mat_reader.lines();

    let mut mat_content: Vec<Vec<u32>> = Vec::new();

    loop {
        match mat_line_iterator.next() {
            Some(result) => {
                                let mat_line = result.unwrap();
                                let slice_iter = mat_line.as_slice().trim_matches(',').split(',')
                                    .map(|x| {
                                                FromStr::from_str(x).unwrap()
                                            });
                                mat_content.push(slice_iter.collect());
                            },
                                 
            None => break,
        }
    }
}
