
use std::{fs::{ File}, io::{ErrorKind}};

fn main() {
    let read_file = File::open("hello.txt");
    // read_file.expect("msg");
    // println!("{:?}", read_file);

    let greeting_file = match read_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {e:?}"),                
            },
            _ => {
                panic!("Error reading the file")
            }
        },        
    };
    println!("{:?}", greeting_file);
    // println!("{:?}", &read_file);
}

    