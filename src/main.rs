use clap::{ Parser,Subcommand};
use serde::Deserialize;
use std::fs;
use chrono::Utc;

pub const DATETIME_FORMAT: &str = "%Y-%m-%d-%H-%M-%S";

#[derive(Parser, Debug, Deserialize)]
#[command(name = "LSTM Plotter")]
#[command(author = "Your Name <georgii.krikun@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Tests an LSTM on binance data files", long_about = None)]
struct Args {
    #[arg(short, long, value_name = "PATTERN", num_args(1..), required = true)]
    input: Vec<String>,
    
    #[arg(short, long, required = true)]
    model: String,
    
    #[arg(short, long, required = true)]
    output: Option<String>,
    
    #[arg(short, long, default_value_t = 1024)]
    batch_size: usize,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
struct ConfigArgs {
    #[arg(short, long, required = true)]
    conf_file: String,
}

#[derive(Subcommand, Debug, Deserialize)]
enum Commands {
    Single {
        #[arg(short, long, required = true,
            help = format!("Datetime in {} format (e.g., {})", DATETIME_FORMAT, Utc::now().format(DATETIME_FORMAT).to_string())
        )]
        datetime: String,
        
        #[arg(short, long, action = clap::ArgAction::SetTrue)] 
        absolute: bool,
    },
    Window {
        #[arg(short, long, required = true,
            help = format!("Start Datetime in {} format (e.g., {})", DATETIME_FORMAT, Utc::now().format(DATETIME_FORMAT).to_string())
        )]
        datetime_start: String,
        
        #[arg(long, required = true,
            help = "Duration (e.g., 4h for a 4-hour window)"
        )]
        duration: String,
        
        #[arg(short, long, action = clap::ArgAction::SetTrue)] 
        absolute: bool,
    },
    WindowTransformed {
        #[arg(short, long, required = true,
            help = format!("Start Datetime in {} format (e.g., {})", DATETIME_FORMAT, Utc::now().format(DATETIME_FORMAT).to_string())
        )]
        datetime_start: String,
        
        #[arg(long, required = true,
            help = "Duration (e.g., 4h for a 4-hour window)"
        )]
        duration: String,
    }
}

#[derive(Debug, thiserror::Error)]
enum ConfigParserError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),
    
    #[error("Command-line parsing error: {0}")]
    ClapError(#[from] clap::error::Error),
    
    #[error("Multiple errors occurred:\n{}", format_multiple(.0))]
    Multiple(Vec<ConfigParserError>),
}

// Helper function to join the errors properly
fn format_multiple(errors: &[ConfigParserError]) -> String {
    errors.iter()
        .map(|e| e.to_string()) // This calls the Display trait, which handles ANSI codes
        .collect::<Vec<_>>()
        .join("\n---\n") // Visual separator between errors
}

fn read_config<ArgsType>(config: &ConfigArgs) -> Result<ArgsType, ConfigParserError> 
where ArgsType: for<'a> Deserialize<'a> {
    // Read the file content
    let content = fs::read_to_string(&config.conf_file)?;
    
    // Parse the TOML into the Config struct
    let config: ArgsType = toml::from_str(&content)?;
    Ok(config)
}

fn parse_config<ArgsType>() -> Result<ArgsType, ConfigParserError> 
where ArgsType: for<'a> Deserialize<'a> + Parser + std::fmt::Debug
{
    let cli_args = ArgsType::try_parse()
        .map_err(ConfigParserError::from);
    let cfg_args = ConfigArgs::try_parse()
        .map_err(ConfigParserError::from)
        .and_then(|cfg_args| {
            read_config::<ArgsType>(&cfg_args)
        });
    
    log::debug!("Parsed command-line arguments: {:?}", cli_args);
    log::debug!("Parsed config arguments: {:?}", cfg_args);
    
    match (cli_args, cfg_args) {
        (Ok(cli_args), Ok(_)) => Ok(cli_args),
        (Ok(cli_args), Err(_)) => Ok(cli_args),
        (Err(_), Ok(cfg_args)) => Ok(cfg_args),
        (Err(cli_err), Err(cfg_err)) => {
            let errors = vec![
                cli_err,
                cfg_err,
            ];
            Err(ConfigParserError::Multiple(errors))
        }
    }
}

fn main() {
    env_logger::init();
    let args = parse_config::<Args>();
    match args {
        Ok(args) => {
            log::info!("Successfully parsed arguments: {:?}", args);
        },
        Err(e) => {
            log::error!("Error parsing arguments: {}", e);
            std::process::exit(1);
        }
    }
}
