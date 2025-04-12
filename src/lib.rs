use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub show_line_number: bool,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        let show_line_number = args.iter().find(|a| *a == "-n").is_some();
        let ignore_case = args.iter().find(|a| *a == "-i").is_some();

        Ok(Config {
            query,
            file_path,
            ignore_case,
            show_line_number,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;

    let results: Vec<String> = if config.ignore_case {
        search_case_insensitive(&config.query, &contents, config.show_line_number)
    } else {
        search(&config.query, &contents, config.show_line_number)
    };

    if results.len() == 0 {
        println!("No results found");
    }

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search(query: &str, contents: &str, with_number: bool) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for (index, line) in contents.lines().enumerate() {
        if line.contains(query) {
            result.push(collect_formatted_line(line, index, with_number))
        }
    }

    result
}

pub fn search_case_insensitive(query: &str, contents: &str, with_number: bool) -> Vec<String> {
    let query = query.to_lowercase();
    let mut result: Vec<String> = Vec::new();

    for (index, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            result.push(collect_formatted_line(line, index, with_number))
        }
    }

    result
}

fn collect_formatted_line(line: &str, index: usize, with_number: bool) -> String {
    if with_number {
        let line_number = index + 1;
        format!("{line_number} {line}")
    } else {
        line.to_string()
    }
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

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, false)
        )
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
            search_case_insensitive(query, contents, false)
        )
    }

    #[test]
    fn case_sensitive_with_line_number() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["2 safe, fast, productive."],
            search(query, contents, true)
        )
    }

    #[test]
    fn case_insensitive_with_line_number() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["1 Rust:", "4 Trust me."],
            search_case_insensitive(query, contents, true)
        )
    }
}
