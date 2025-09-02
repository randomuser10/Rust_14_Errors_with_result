use std::fs::File;
use std::io::ErrorKind;

fn main(){
    let greeting_file = File::open("hello.txt");

    let _greeting_check = match greeting_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fs) => fs,
                Err(e) => panic!("Failed to create file {e:?}"),                
            },
            _ => panic!("Failed to read the file"),
        }       
    };
}