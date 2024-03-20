use std::io::Write;

use regex::Regex;
use scaffold::{
    args::{self},
    utils::ValidInput,
};

fn main() {
    let mut buffer = String::new();
    let mut line = String::new();
    println!("Construct your template(type 'exit' to quit or 'done' to confirm):");

    loop {
        line.clear();
        let flush = std::io::stdout().flush();
        assert!(flush.is_ok());

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if let Ok(finish_args) = line.trim().parse::<args::FinishArgs>() {
            match finish_args {
                args::FinishArgs::Done(message) => {
                    println!("{}", message);
                    break;
                }
                args::FinishArgs::Exit(message) => {
                    println!("{}", message);
                    break;
                }
            }
        } else {
            buffer.push_str(&line);
        }
    }

    let vec: Vec<String> = buffer.split_whitespace().map(|x| x.to_string()).collect();
    if vec.iter().any(|x| ValidInput::validate(x).is_none()){
        println!("Some of the inputs are invalid, please try again.");
        return;
    }
    let tree = scaffold::process_template::create_tree_structure(vec);

    println!("{}", tree);
}
