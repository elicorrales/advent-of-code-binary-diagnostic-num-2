use std::env::args;
use std::fs::{File, self};
use std::process::exit;
use std::io::{Read, BufReader, BufRead};

fn main() {
    let args:Vec<String> = args().collect();
    //println!("{:?}", args);
    if args.len() < 2 {
        println!("\nNeed path to inputs file.\n");
        exit(1);
    }

    let path = &args[1];

    //let err_msg = format!("\nFile {} was not found.\n", path);
    let err_msg = format!("\nFile {path} was not found.\n");

    let file_result = File::open(path);
    let mut file;
    match file_result {
        Ok(f) => { file = f; },
        _ => {
            println!("{}", err_msg);
            exit(1);
        }
    }

    /*
    let contents_as_string = fs::read_to_string(path).unwrap();
    println!("{contents_as_string}");
    */

    /*
    let contents_as_vector = fs::read(path).unwrap();
    println!("{:?}", contents_as_vector);
    */

    /*
    let mut contents_as_buffer = vec![0; 15000];
    let count = file.read(&mut contents_as_buffer).unwrap();
    println!("{:?}", contents_as_buffer);
    println!("\n{count}\n");
    */

    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let binary_string = line.unwrap();
        println!("{index}, {binary_string}");
    }
}
