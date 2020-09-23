use std::env;
use std::error::Error;
use std::fs;

//| My study project for the Rust

#[cfg(test)]
mod tests {
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

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

use std::collections::HashMap;
struct Cacher<T, U>
where
    T: Fn(&U) -> U,
    U: std::cmp::Eq + std::hash::Hash,
{
    values: HashMap<U, U>,
    calculation: T,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(&U) -> U,
    U: std::cmp::Eq + std::hash::Hash + Copy,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, val: U) -> U {
        match self.values.get(&val) {
            None => {
                let result = (self.calculation)(&val);
                self.values.insert(val, result);
                result
            }
            Some(v) => *v,
        }
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut cache_example = Cacher::new(|x| x + 1);
    let res1 = cache_example.value(1);
    let res2 = cache_example.value(1);
    println!("{} {}", res1, res2);
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Search 'query' in the content using case insensitive method
///
/// # Examples
///
/// ```
/// let query = "hey";
/// let contents = "my string\nwith hey\nHey";
/// let result = minigrep::search_case_insensitive(&query, &contents);
/// assert_eq!(vec!["with hey", "Hey"], result);
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
