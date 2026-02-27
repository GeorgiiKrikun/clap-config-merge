use clap::{ Parser,Subcommand};
use figment::{Figment, providers::{Format, Toml, Env, Serialized}};
use serde::{Deserialize, Serialize};
use std::fs;
use chrono::Utc;

pub const DATETIME_FORMAT: &str = "%Y-%m-%d-%H-%M-%S";

#[derive(Parser, Debug, Deserialize, Serialize)]
#[command(name = "LSTM Plotter")]
#[command(author = "Your Name <georgii.krikun@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Tests an LSTM on binance data files", long_about = None)]
struct Args {
    #[arg(short, long, value_name = "PATTERN", num_args(1..))]
    input: Option<Vec<String>>,
    
    #[arg(short, long)]
    model: Option<String>,
    
    #[arg(short, long)]
    output: Option<String>,
    
    #[arg(short, long)]
    batch_size: Option<usize>,
    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug, Deserialize, Serialize)]
enum Commands {
    Single {
        #[arg(short, long,
            help = format!("Datetime in {} format (e.g., {})", DATETIME_FORMAT, Utc::now().format(DATETIME_FORMAT).to_string())
        )]
        datetime: Option<String>,
        
        #[arg(short, long, action = clap::ArgAction::SetTrue)] 
        absolute: bool,
    },
    Window {
        #[arg(short, long, required = true,
            help = format!("Start Datetime in {} format (e.g., {})", DATETIME_FORMAT, Utc::now().format(DATETIME_FORMAT).to_string())
        )]
        datetime_start: Option<String>,
        
        #[arg(long, required = true,
            help = "Duration (e.g., 4h for a 4-hour window)"
        )]
        duration: Option<String>,
        
        #[arg(short, long, action = clap::ArgAction::SetTrue)] 
        absolute: bool,
    },
    WindowTransformed {
        #[arg(short, long, required = true,
            help = format!("Start Datetime in {} format (e.g., {})", DATETIME_FORMAT, Utc::now().format(DATETIME_FORMAT).to_string())
        )]
        datetime_start: Option<String>,
        
        #[arg(long, required = true,
            help = "Duration (e.g., 4h for a 4-hour window)"
        )]
        duration: Option<String>,
    }
}

// 1. We use this just to get the config file path from the CLI
#[derive(Parser, Debug, Serialize)]
struct ProviderArgs {
    #[arg(short, long)]
    conf_file: Option<String>,
}

fn parse_config<ArgsType>() -> Result<ArgsType, ConfigParserError> 
where ArgsType: for<'a> Deserialize<'a> 
    + Parser
    + serde::Serialize
    + std::fmt::Debug
{
    let provider_args = ProviderArgs::try_parse().ok();
    println!("Provider args: {:#?}", provider_args);
    
    let mut figment = Figment::new()
        .merge(Env::prefixed("APP_").split("__"));
    
    if let Some(args) = provider_args {
        if let Some(path) = args.conf_file {
            figment = figment.merge(Toml::file(path));
        }
    }
    
    println!("Figment after merging env and file: {:#?}", figment);
    let cli_provider = Serialized::defaults(ArgsType::parse());
    println!("CLI provider: {:#?}", cli_provider);
    figment = figment.merge(cli_provider);
    
    figment.extract().map_err(ConfigParserError::from)
}

#[derive(Debug, thiserror::Error)]
enum ConfigParserError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),
    
    #[error("Command-line parsing error: {0}")]
    ClapError(#[from] clap::error::Error),
    
    #[error("Figment error: {0}")]
    FigmentError(#[from] figment::Error),
}

fn main() {
    env_logger::init();
    match parse_config::<Args>() {
        Ok(args) => {
            println!("Parsed configuration: {:#?}", args);
            // Here you would call your main application logic, passing `args`
        },
        Err(e) => {
            eprintln!("Error parsing configuration:\n{}", e);
            std::process::exit(1);
        }
    }
}
