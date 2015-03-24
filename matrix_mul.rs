#![feature(core)]

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::thread;

fn load_matrix(file_path: &Path) -> Vec<Vec<u32>> {

    let mat_file = File::open(file_path).unwrap();

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

    mat_content
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("incorrect arguments");
        return;
    }

    let matrix_1: Vec<Vec<u32>> = load_matrix(Path::new(&args[1]));
    let matrix_2: Vec<Vec<u32>> = load_matrix(Path::new(&args[2]));

    assert!(matrix_1[0].len() == matrix_2.len());

    let mut result: Vec<Vec<u32>> = Vec::new();
    let mul_length = matrix_2.len();

    for y in 0..matrix_1.len() {

        let mut result_row: Vec<u32> = Vec::new();
        let mut entry_value: u32 = 0;

        for x in 0..matrix_2[0].len() {

            for i in 0..mul_length { 
                entry_value += matrix_1[y][i] * matrix_2[i][x];
            }

            result_row.push(entry_value);
        }

        result.push(result_row);
    }

    for y in 0..result.len() {
        for x in 0..result[0].len() {
            print!("{},", result[y][x]);
        }
        print!("\n");
    }
}
