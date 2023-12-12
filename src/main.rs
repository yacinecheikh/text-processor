mod commands;
mod parse;
mod args;
mod generate;
mod path;

use std::process::{exit};


#[cfg(test)]
mod tests;

fn main() {
    let args = args::parse_args();

    let Ok(mut args) = args else {
        let err = args.err().unwrap();
        println!("error while parsing arguments: {}", err);
        exit(0);
    };

    commands::init(&mut args);

    generate::generate_all(&args);

    match commands::cleanup(args.file.as_path()) {
        Ok(_) => {}
        Err(err) => {
            println!("error while removing cache directories: {}", err);
        }
    }
}
