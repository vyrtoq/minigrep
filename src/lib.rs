use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // automatic coercion of &Vec<String> into String slice
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 { 
            return Err("not enough arguments")
        }
        let query = args[1].clone(); // for simplicity
        let file_path = args[2].clone(); // for simplicity
        Ok(Config { query, file_path})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?; // only continues if config::build did not error
    
    println!("Content of file:\n{contents}");

    Ok(())
}