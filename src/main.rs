use clap::{ Parser,Subcommand, Arg, Command };
use serde::Deserialize;
use std::fs;

// #[derive(Parser)]
// struct CliParams {
//         /// The path to the input file
//         #[arg(short, long)]
//         input: String,
//     
//         /// The path to the output file
//         #[arg(short, long)]
//         output: String,
// }

#[derive(Parser)]
struct CliParams {
        /// The path to the config file
        #[arg(short, long, default_value = "config.toml")]
        config: String,
}

#[derive(Deserialize)]
struct Config {
    input: String,
    output: String,
}

fn read_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    // Read the file content
    let content = fs::read_to_string(file_path)?;
    
    // Parse the TOML into the Config struct
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}


fn main() {
    let cli_params = CliParams::parse();
    println!("Input file: {}", cli_params.config);
    match read_config(&cli_params.config) {
        Ok(config) => {
            println!("Input: {}", config.input);
            println!("Output: {}", config.output);
        },
        Err(e) => eprintln!("Error reading config: {}", e),
    }
}
