
use std::fs;
use std::error::Error;

struct Config {
    pub query: String,
    pub file_path: String,
}




impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str>  { // previously new()
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> { // dyn: dynamic dispatch - handles any type of error. e.g, io::Error, fmt::Error
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    
    // let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    let contents = fs::read_to_string(config.file_path)?; // ? - "If it's ok, keep going; if it's Err, return that error to the caller."

    println!("With text:\n{contents}");

    Ok(())
}