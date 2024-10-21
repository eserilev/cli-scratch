// use clap::builder::styling::AnsiColor;
// use clap::builder::Styles;
// use clap::{Parser, ValueEnum};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct TestCli {
    /// Some option that can be provided via CLI or TOML
    #[clap(long, required_unless_present = "config_file", default_value="")]
    pub some_option: String,

    /// Path to the TOML configuration file
    #[clap(long)]
    pub config_file: Option<String>,
}


fn main() {
    let cli = TestCli::parse();
    let cli = if let Some(config_file) = cli.config_file {
        let config_content = fs::read_to_string(&config_file).expect("Failed to read config file");
        toml::from_str(&config_content).expect("Failed to parse TOML")
    } else {
        cli
    };
    

    // Use the CLI value or fallback to the TOML config value
    let option_value = cli.some_option;
    println!("{:?}", option_value);
}