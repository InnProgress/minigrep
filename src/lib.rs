use colored::Colorize;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("You must provide two arguments: filename query");
        }

        let filename = args[1].clone();
        let query = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let found = search(&config.query, &contents);
    print!("{}", found);

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> String {
    let lines: Vec<String> = contents.split("\n").map(String::from).collect();

    let mut found_lines: String = String::new();
    for (i, line) in lines.iter().enumerate() {
        if line.contains(&query) {
            let new_line = line.replace(&query, &query.red().to_string());

            found_lines.push_str(&format!("{}:{}\n", (i + 1).to_string().green(), new_line));
        }
    }

    found_lines
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

        assert_eq!(
            format!(
                "{}:safe, fast, pro{}ive.\n",
                "2".green(),
                "duct".red().to_string()
            ),
            search(query, contents)
        );
    }
}
