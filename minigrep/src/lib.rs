use std::fs;
use std::env;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args : &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments. need query and filename.");
        }

        Ok(Config {
            query: args[1].to_string(),
            filename: args[2].to_string(),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

pub fn run(config : Config) -> Result<(), Box<dyn Error>> {

    let filename = config.filename;
    let query = config.query;
    let file_contents = fs::read_to_string(filename)?;

    if config.case_sensitive {
        for line in search(&query, &file_contents) {
            println!("{}", line);
        }
    } else {
        for line in search_case_insensitive(&query, &file_contents) {
            println!("{}", line);
        }
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

pub fn search_case_insensitive<'a>(query : &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line)
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
Duct tape.
";
        assert_eq!(
            vec!["safe, fast, productive!"],
            search(query, contents)
        );
    }

    #[test]
    fn test_case_insensitive_search() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive!
Pick three.
RuSt is awesome.
";
        assert_eq!(
            vec!["Rust:", "RuSt is awesome."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn test_config() {
        let args = [String::from("binary"),
                    String::from("needle"),
                    String::from("haystack")];
        let config = Config::new(&args).unwrap();

        assert_eq!(true, config.case_sensitive);
    }
}
