use failure::Context;
use failure::ResultExt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "minigrep", about = "A simple grep tool")]
pub struct Config {
    /// the pattern
    pub pattern: String,
    /// the path
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,
    /// case sensitive
    #[structopt(short = "i", long)]
    pub is_case_sensitive: bool,
}

pub fn run(config: &Config) -> Result<Vec<String>, Context<String>> {
    let content = std::fs::read_to_string(&config.path)
        .with_context(|_| format!("Error reading `{:?}`", &config.path))?;

    let result = if config.is_case_sensitive {
        search(&config.pattern, &content)
    } else {
        search_case_insensitive(&config.pattern, &content)
    };
    Ok(result)
}

fn search(pattern: &str, content: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(pattern) {
            result.push(line.into());
        }
    }
    result
}

fn search_case_insensitive(pattern: &str, content: &str) -> Vec<String> {
    let pattern = pattern.to_lowercase();
    let mut result = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&pattern) {
            result.push(line.into());
        }
    }
    result
}

#[cfg(test)]
mod tests {

    #[test]
    fn search_case_insensitive() {
        let pattern = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape."],
            super::search_case_insensitive(pattern, content)
        );
    }

    #[test]
    fn search_case_sensitive() {
        let pattern = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            super::search(pattern, content)
        );
    }
}
