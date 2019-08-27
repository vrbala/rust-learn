use std::fs;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args : &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments. need query and filename.");
        }

        Ok(Config {
            query: args[1].to_string(),
            filename: args[2].to_string(),
        })
    }
}

pub fn run(config : Config) -> Result<(), Box<dyn Error>> {

    let filename = config.filename;
    let query = config.query;
    let file_contents = fs::read_to_string(filename)?;
    for line in search(&query, &file_contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query : &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive!
Pick three.
";
        assert_eq!(
            vec!["safe, fast, productive!"],
            search(query, contents)
        );
    }
}
