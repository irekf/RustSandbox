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

fn single_thread_mul(m1: &Vec<Vec<u32>>, m2: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {

    let mut result: Vec<Vec<u32>> = Vec::new();
    let mul_length = m2.len();

    for y in 0..m1.len() {

        let mut result_row: Vec<u32> = Vec::new();
        let mut entry_value: u32 = 0;

        for x in 0..m2[0].len() {

            for i in 0..mul_length { 
                entry_value += m1[y][i] * m2[i][x];
            }

            result_row.push(entry_value);
        }

        result.push(result_row);
    }

    result
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("incorrect arguments");
        return;
    }

    let m1: Vec<Vec<u32>> = load_matrix(Path::new(&args[1]));
    let m2: Vec<Vec<u32>> = load_matrix(Path::new(&args[2]));

    assert!(m1[0].len() == m2.len());

    let result: Vec<Vec<u32>> = single_thread_mul(&m1, &m2);

    for y in 0..result.len() {
        for x in 0..result[0].len() {
            print!("{},", result[y][x]);
        }
        print!("\n");
    }
}
