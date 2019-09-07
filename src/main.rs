#[macro_use]
extern crate lazy_static;

use std::env;
use std::process;
use std::io::{self, Read, Write, BufRead};
use std::fs::{File};

mod token;
mod scanner;
mod error_handling;
mod util;

use scanner::Scanner;
use error_handling::*;

fn run(input: &String) {
    let mut scanner = Scanner::new(input.to_string());
    let tokens  = scanner.scan_tokens();
    println!("Generated {} tokens from source", tokens.len());
    for t in tokens {
        println!("{:?}", t);    
    }
}

fn run_file(path: String) {
    println!("Running from file");
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
    println!("Running from command line.");
    let mut buffer = String::new();

    let o = io::stdout();
    let i = io::stdin();

    let mut out_h = o.lock();
    let mut in_h = i.lock();
    loop {
        print!("> ");
        out_h.flush().unwrap();
        match in_h.read_line(&mut buffer) {
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
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(args[1].clone());
    } else {
        run_prompt();
    }
    Ok(())
}
