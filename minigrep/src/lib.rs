use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
	pub case_sensitive: bool,
}

impl Config {
	// clone() can be costly, but we can't even pass in an owned Vec<String>
	// here and expect it to work because we can't move out of an index of a vector.
	// Clone is still required for Config to own the query/filename strings.
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

		// If the environment variable CASE_INSENSITIVE is set to anything,
		// is_err will return false and the program will perform a case-insensitive search.
		// Otherwise, if CASE_INSENSITIVE is not set to anything, env::var.is_err()
		// would return true, and we'd do a case-sensitive search.
		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

/**
* - Returns the unit value "()" in the Ok case.
* - Returns a trait object: a type that implements the
* Error trait, but we don't have to specify what particular
* type the return value will be.
*/
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

	let results = if config.case_sensitive {
		search(&config.query, &contents)
	} else {
		search_case_insensitive(&config.query, &contents)
	};
    
	for line in results {
		println!("{}", line);
	}

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();
	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}
	results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			results.push(line);
		}
	}
	results
}

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
