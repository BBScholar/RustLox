

use std::env;
use std::process;
use std::io::{self, Read};
use std::fs::{File};

mod token;
mod scanner;
mod error_handling;

use scanner::Scanner;
use error_handling::*;

fn run(input: &String) {
    let mut scanner = Scanner::new(input.to_string());
    let tokens  = scanner.scan_tokens();

    for t in tokens {
        println!("{:?}", t);    
    }
}

fn run_file(path: String) {
    let f = File::open(path);
    
    let mut f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Could not open the file: {:?}", error);
        },
    };

    let mut buffer = String::new();
    match f.read_to_string(&mut buffer) {
        Ok(_) => {} ,
        Err(error) => {
            panic!("Could not read from file: {:?}", error);
        },
    }
    run(&buffer);

    unsafe {
        if HAS_ERROR {
            process::exit(65);
        }
    }
}

fn run_prompt() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        print!("> ");
        match handle.read_to_string(&mut buffer) {
            Ok(_) => {},
            Err(error) => {
                panic!("Could not read line: {:?}", error);
            },
        }

        run(&buffer);
        unsafe {
            HAS_ERROR = false;
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Usage: jlox [script]");
        process::exit(64);
    } else if args.len() == 1 {
        unsafe {
            run_file(args[0].clone());
        }
    } else {
        unsafe {
            run_prompt();
        }
    }
    Ok(())
}
