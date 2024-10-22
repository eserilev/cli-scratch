// use clap::builder::styling::AnsiColor;
// use clap::builder::Styles;
// use clap::{Parser, ValueEnum};
use clap::Parser;
use serde::{Deserialize, Serialize};
use toml::Value;
use std::fs;

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct TestCli {
    /// Some option that can be provided via CLI or TOML
    #[clap(long, required_unless_present = "config_file", default_value="")]
    pub some_option: String,

    #[clap(long, conflicts_with="some_option")]
    pub conflicting_option: Option<String>,

    /// Path to the TOML configuration file
    #[clap(long)]
    pub config_file: Option<String>,
}

/// Recursively converts TOML values into key-value pairs in a `--key value` format.
fn toml_to_kv_list(value: &Value, prefix: Option<String>) -> Vec<String> {
    let mut kv_list = Vec::new();

    match value {
        Value::Table(table) => {
            // Iterate over the table entries (e.g., [nested] blocks)
            for (key, val) in table {
                let key = key.replace("_", "-");
                // Append the current key to the prefix
                let new_prefix = if let Some(ref p) = prefix {
                    format!("{}.{}", p, key) // Use "." for nested keys
                } else {
                    key.clone()
                };
                // Recursively handle nested values
                kv_list.extend(toml_to_kv_list(val, Some(new_prefix)));
            }
        }
        Value::Array(arr) => {
            // Handle arrays by indexing each element
            for (i, val) in arr.iter().enumerate() {
                let new_prefix = format!("{}[{}]", prefix.as_ref().unwrap(), i);
                kv_list.extend(toml_to_kv_list(val, Some(new_prefix)));
            }
        }
        _ => {
            // Handle basic values (strings, integers, etc.)
            if let Some(key) = prefix {
                kv_list.push(format!("--{}", key));
                kv_list.push(format!("{}", value));
                
            }
        }
    }

    kv_list
}


fn main() {
    let cli = TestCli::parse();
    let cli = if let Some(config_file) = cli.config_file {
        let config_content = fs::read_to_string(&config_file).expect("Failed to read config file");
        let parsed_toml: Value = config_content.parse().expect("Failed to parse toml");
        let mut args = vec![String::new()];
        let kv_list = toml_to_kv_list(&parsed_toml, None);
        args.extend(kv_list);
        TestCli::parse_from(args)
    } else {
        cli
    };
    

    // Use the CLI value or fallback to the TOML config value
    let option_value = cli.some_option;
    println!("{}", option_value);
}