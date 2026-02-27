use clap::{ Parser,Subcommand, Arg, Command };
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
    input: String,
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

fn read_config(file_path: &str) -> Result<Args, Box<dyn std::error::Error>> {
    // Read the file content
    let content = fs::read_to_string(file_path)?;
    
    // Parse the TOML into the Config struct
    let config: Args =  toml::from_str(&content)?;
    Ok(config)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::try_parse();
    if let Err(e) = args {
        let cfg_args = ConfigArgs::try_parse();
        if let Err(cfg_e) = &cfg_args {
            eprintln!("Error parsing command-line arguments: {}", e);
            eprintln!("Error parsing config arguments: {}", cfg_e);
            std::process::exit(1);
        } else {
            let cfg_args = cfg_args.as_ref().unwrap();
            println!("Parsed config arguments: {:?}", cfg_args);
            let config = read_config(&cfg_args.input)?;
            println!("Parsed config: {:?}", config);
        }
    } else {
        let args = args.as_ref().unwrap();
        println!("Parsed command-line arguments: {:?}", args);
    }
    
    Ok(())
}
