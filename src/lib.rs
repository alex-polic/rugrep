use std::error::Error;
use std::fs::read_to_string;

pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("There must be 2 arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            file_name: args[2].clone()
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_name);

    let contents = read_to_string(config.file_name)?;

    let res = search(&config.query, &contents);

    dbg!(res);

    Ok(())
}

fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str>{
    let mut res: Vec<&str> = Vec::new();
    for line in contents.lines() {
       if line.contains(query) {
           res.push(line);
       }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
