mod lexer;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn run_file(path: String) {
    let path = Path::new(&path);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, buffer),
    };
    run(&buffer);
}

fn run_prompt() {
    use std::io::{stdin, stdout};
    let mut buffer = String::new();
    loop {
        buffer.clear();
        print!("> ");
        let _ = stdout().flush();
        let nbytes = stdin().read_line(&mut buffer).expect("failed to read line");
        if nbytes == 0 && buffer == "" {
            break;
        }
        run(&buffer);
    }
}

fn run(source: &String) {
    let mut lexer = lexer::Lexer::new(source);
    while let Some(token) = lexer.next() {
        print!("{:?}\n", token);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: schmuse [script]");
        return
    } else if args.len() == 2 {
        run_file(args[1].clone())
    } else {
        run_prompt();
    }
}
