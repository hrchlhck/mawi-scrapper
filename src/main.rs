#![allow(warnings)]

use std::sync::Arc;

use clap::Parser;
use cli::Program;
use tokio::runtime::Builder;
use tokio::task::JoinHandle;

use log::{error, info};

mod cli;
mod parser;
mod scrapper;

#[tokio::main]
async fn main() -> Result<(), scrapper::error::Error> {
    env_logger::builder()
        .default_format()
        .filter_level(log::LevelFilter::Info)
        .init();
    let args = cli::CLI::parse();

    match args.program() {
        cli::Program::Scrapper {
            base_url,
            base_output_path,
            whole_year,
            year,
            month,
            day,
        } => {
            let mut url: String = String::new();
            let mut output_path: String = String::new();

            if whole_year {
                let bu: Arc<String> = Arc::new(base_url.clone());
                let bo: Arc<String> = Arc::new(base_output_path.clone());
                let mut handles: Vec<JoinHandle<()>> = Vec::new();

                for month in 1..=12 {
                    let bu1 = bu.clone();
                    let bo1 = bo.clone();
                    let url: String;
                    let output_path: String;
                    (url, output_path) = Program::format_custom(bu1, bo1, year, month as u8, day);
                    let t = tokio::task::spawn(async {
                        scrapper::run(url, output_path).await;
                    });

                    handles.push(t);
                }

                for h in handles {
                    h.await;
                }
                
                return Ok(())
            } 
            
            (url, output_path) = Program::format(&args.program()).unwrap();
            return scrapper::run(url, output_path).await;
        }
        cli::Program::PcapParser => {
            error!("Not implemented yet!");
            return Ok(())
        }
    }

    Ok(())
}
