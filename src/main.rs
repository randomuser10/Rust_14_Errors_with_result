// use std::fs::File;
// use std::io::ErrorKind;

use std::fs::File;

fn main(){
    // let greeting_file = File::open("hello.txt");

    // let _greeting_check = match greeting_file {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fs) => fs,
    //             Err(e) => panic!("Failed to create file {e:?}"),                
    //         },
    //         _ => panic!("Failed to read the file"),
    //     }       
    // };
    //using unwrap, if ok gives result or else it will panic
    // let greeting_file = File::open("hello.txt").unwrap();

    //using expect, if ok gives result or else it will panic but will give out the error message that we want.
    let _greeting_file = File::open("hello.txt").expect("No such file exists, please check the file name");
}