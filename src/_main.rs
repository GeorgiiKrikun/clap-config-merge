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
    
    /// The path to the input file (overrides config if provided)
    #[arg(long)]
    input: Option<String>,
    
    /// The path to the output file (overrides config if provided)
    #[arg(long)]
    output: Option<String>,
}

#[derive(Deserialize, Debug)]
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


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_params = CliParams::parse();
    
    println!("Input file: {}", cli_params.config);
    let mut config = read_config(&cli_params.config)?;
    
    if let Some(input) = cli_params.input {
        config.input = input;        
    }
    if let Some(output) = cli_params.output {
        config.output = output;        
    }
    
    println!("Final Config: {:?}", config);
    Ok(())
}
