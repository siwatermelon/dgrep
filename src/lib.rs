#![allow(unused_variables,unused_imports)]

use std::{fs, error::Error, env, collections::btree_map::Iter};
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
       mut args: impl Iterator<Item = String>
    ) -> Result<Config,&'static str> {
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        
        Ok(Config { query, file_path, ignore_case})
    }
}

pub fn run(config: Config) -> Result<(),Box<dyn Error>> {

    let contents = fs::read_to_string(config.file_path)?;
    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &contents){
            println!("{line}");
        }
    }else{
        for line in search(&config.query, &contents){
            println!("{line}");
        }
    }
    
    Ok(())
}
/// 1
/// # Example
/// it's a demo for veritifying the behavior of the search function.
/// ```
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// ick three.
/// Duct tape.";
/// assert_eq!(vec!["safe, fast, productive."], dgrep::search(query, contents));
/// ```
///
pub fn search<'a>(query: &str,contents: &'a str) -> Vec<&'a str> {
    contents
    .lines()
    .filter(|x| x.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str,contents: &'a str)-> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line)
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

