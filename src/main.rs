//! A cli based SQRL client
//!
//! <https://grc.com/sqrl>

#![deny(missing_docs)]

mod error;
mod server;

use clap::{Parser, Subcommand};
use error::CliError;
use sqrl_client::SqrlClient;
use std::{error::Error, path::{self, Path}};

const DEFAULT_SQRL_FILE: &str = "~/.sqrl";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let command = SQRLCommand::parse();

    if let Err(error) = handle_command(&command).await {
        println!("Error: {}", error);
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct SQRLCommand {
    #[arg(short, long, default_value_t=DEFAULT_SQRL_FILE.to_owned())]
    sqrl_path: String,

    #[arg(short, long)]
    password: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

impl SQRLCommand {
    fn get_password(&self) -> Result<String, CliError> {
        match &self.password {
            Some(p) => Ok(p.clone()),
            None => Ok(rpassword::prompt_password("Please enter a password: ")?),
        }
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    New,
    Authenticate {
        url: String,
    },
    TextualIdentity,
    RunServer
}

async fn handle_command(command: &SQRLCommand) -> Result<(), CliError> {
    let sqrl_path = Path::new(&command.sqrl_path).canonicalize().unwrap(); 
    println!("Path: {}", sqrl_path.to_str().unwrap());
    match &command.command {
        Commands::New => {
            let password = command.get_password()?;
            let (client, recovery_key) = SqrlClient::new(&password)?;
            println!("Recover key {}", &recovery_key);

            client.to_file(sqrl_path.to_str().unwrap())?;
        }
        Commands::Authenticate { url } => {
            let client = load_client(sqrl_path.to_str().unwrap())?;
            let password = command.get_password()?;
            let verifying_key = client.get_public_identity(&password, &url, None)?;
            println!("Bytes: {:?}", verifying_key.to_bytes());
        }
        Commands::TextualIdentity => {
            let client = load_client(sqrl_path.to_str().unwrap())?;
            println!("{}", client.to_textual_identity_format()?);
        }
        Commands::RunServer => {
            let client = load_client(sqrl_path.to_str().unwrap())?;
        }
    }

    Ok(())
}

fn load_client(file_path: &str) -> Result<SqrlClient, CliError> {
    Ok(SqrlClient::from_file(file_path)?)
}
