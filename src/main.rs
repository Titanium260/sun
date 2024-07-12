use std::fs::read_to_string;
use std::env::args;

fn main() {

    let err1 = String::from("sun: No arguments, aborting!");

    if args().len() > 1 {

        let args: Vec<String> = args().collect();

        let path = &args[1];

        let text = read_to_string(path);

        match text {
            
            Ok(content) => { print!("{}", content); }
            Err(error) => { eprintln!("sun: error: {}", error); }
        
        }

    } else {

        eprintln!("{}", err1);

    }
}
