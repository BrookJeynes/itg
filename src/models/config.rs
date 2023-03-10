use crossterm::style::Stylize;
use serde::{Deserialize, Serialize};

use crate::reset_terminal;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub github_access_token: String,
    pub user_name: String,
}

impl Config {
    pub fn initialise_config(new_config: Config) -> Config {
        let mut config: Config = confy::load("issue-tracker", None).unwrap_or_else(|err| {
            eprintln!("{}: {}", "Error".red().bold(), err);
            reset_terminal().unwrap_or_else(|_| panic!("Failed to reset terminal"));
            std::process::exit(1);
        });

        Config::load_new_config(&mut config, new_config);
        Config::check_empty_values(&config);

        config
    }

    fn load_new_config(config: &mut Config, new_config: Config) {
        if !new_config.github_access_token.is_empty()
            && new_config.github_access_token != config.github_access_token
        {
            config.github_access_token = new_config.github_access_token;
        }

        if !new_config.user_name.is_empty() && new_config.user_name != config.user_name {
            config.user_name = new_config.user_name;
        }

        confy::store("issue-tracker", None, &config).unwrap_or_else(|err| {
            eprintln!("{}: {}", "Error".red().bold(), err);
            reset_terminal().unwrap_or_else(|_| panic!("Failed to reset terminal"));
            std::process::exit(1);
        });
    }

    fn check_empty_values(config: &Config) {
        if config.github_access_token.is_empty() {
            eprintln!(
                "{}: No Github access token set. Please set one with the --token (-t) flag.",
                "Error".red().bold()
            );
            reset_terminal().unwrap_or_else(|_| panic!("Failed to reset terminal"));
            std::process::exit(1);
        }

        if config.user_name.is_empty() {
            eprintln!(
                "{}: No Github user name is set. Please set one with the --user-name (-u) flag.",
                "Error".red().bold()
            );
            reset_terminal().unwrap_or_else(|_| panic!("Failed to reset terminal"));
            std::process::exit(1);
        }
    }
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            github_access_token: String::from(""),
            user_name: String::from(""),
        }
    }
}
