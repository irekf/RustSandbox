
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
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
                                let slice_iter = mat_line.trim_matches(',').split(',')
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

fn multi_thread_mul(m1: Vec<Vec<u32>>, m2: Vec<Vec<u32>>, thread_num: usize) -> Vec<Vec<u32>> {

    let rows_to_process = m1.len();
    let rows_per_thread = rows_to_process / thread_num;

    let shared_m1 = Arc::new(m1);
    let shared_m2 = Arc::new(m2);

    // is there a better way to initialize a vector?
    let mut result: Vec<Vec<u32>> = Vec::with_capacity(rows_to_process);
    //result.resize(rows_to_process, Vec::new());
    for _ in 0..rows_to_process {
        result.push(Vec::new());
    }

    let mul_length = shared_m2.len();

    let shared_result = Arc::new(Mutex::new(Some(result)));

    let mut threads = vec![];
    for thread_idx in 0..thread_num {

        let child_m1 = shared_m1.clone();
        let child_m2 = shared_m2.clone();
        let child_result_arcptr = shared_result.clone();

        let join_handle = thread::spawn(move || {

            let start_row_idx = thread_idx * rows_per_thread;
            let end_row_idx =
                if start_row_idx + rows_per_thread < rows_to_process - 1 {
                    start_row_idx + rows_per_thread
                }
                else {
                    rows_to_process
                };

            println!("thread #{}: start={}, end={}", thread_idx, start_row_idx, end_row_idx);

            for y in start_row_idx..end_row_idx {

                let mut result_row: Vec<u32> = Vec::new();
                let mut entry_value: u32 = 0;

                for x in 0..child_m2[0].len() {

                    for i in 0..mul_length {
                        entry_value += child_m1[y][i] * child_m2[i][x];
                    }

                    result_row.push(entry_value);
                }

                // get the guard
                let mut result_guard = child_result_arcptr.lock().unwrap();
                // extract the result vector from the Option and substitute it wth a None
                let mut child_result_vector = result_guard.take().unwrap();
                // alter the result vector
                child_result_vector[y] = result_row;
                // put the vector back to the Option
                *result_guard = Some(child_result_vector);
            }

            println!("thread #{} finished", thread_idx);

        });

        threads.push(join_handle);
    }

    for t in threads {
        let _ = t.join();
    }

    // get the guard and extract the result vector from the Option, leave a None there(take())
    let mut guard = shared_result.lock().unwrap();
    guard.take().unwrap()
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

    //let result: Vec<Vec<u32>> = single_thread_mul(&m1, &m2);
/*
    for y in 0..result.len() {
        for x in 0..result[y].len() {
            print!("{},", result[y][x]);
        }
        print!("\n");
    }

*/
    let result: Vec<Vec<u32>> = multi_thread_mul(m1, m2, 2);
/*
    for y in 0..result.len() {
        for x in 0..result[y].len() {
            print!("{},", result[y][x]);
        }
        print!("\n");
    }
*/
}
