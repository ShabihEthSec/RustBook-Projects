use std::fs;
use std::env;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1); // note: Exiting with 1 signals to the OS that the program failed.
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> { // dyn: dynamic dispatch - handles any type of error. e.g, io::Error, fmt::Error
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    
    // let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    let contents = fs::read_to_string(config.file_path)?; // ? - "If it's ok, keep going; if it's Err, return that error to the caller."

    println!("With text:\n{contents}");

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}


impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str>  { // previously new()
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}