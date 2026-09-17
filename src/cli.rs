use std::sync::Arc;

use crate::scrapper::create_path;
use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Subcommand)]
pub enum Program {
    Scrapper {
        /// Base URL for the MAWI pcap files
        #[arg(
            short,
            long,
            default_value = "https://mawi.wide.ad.jp/mawi/samplepoint-F",
            required = false
        )]
        base_url: String,
        
        /// Base output path for saving the PCAP files
        #[arg(short = 'o', long, required = true)]
        base_output_path: String,
        
        /// Download first day of every month of a year
        #[arg(short='A', long, default_value_t=false)]
        whole_year: bool,

        /// Year of the data
        #[arg(short, long)]
        year: u16,

        /// Month of the data
        #[arg(short, long, required = false, required_if_eq("whole_year", "false"))]
        month: Option<u8>,

        /// Day number
        #[arg(short, long, default_value = None, required = false)]
        day: Option<u8>,

    },
    PcapParser,
}

impl Program {
    pub fn format(inst: &Program) -> Option<(String, String)> {
        match inst {
            Program::PcapParser => None,
            Program::Scrapper {
                base_url,
                base_output_path,
                year,
                month,
                day,
                whole_year,
            } => {
                let new_base_path: String = create_path(base_output_path);

                let m = month.unwrap();
                match day {
                    Some(d) => Some((
                        format!("{base_url}/{year}/{year}{m:02}{d:02}1400.pcap.gz"),
                        format!("{new_base_path}/{year}_{m:02}_{d:02}.pcap.gz"),
                    )),
                    None => Some((
                        format!("{base_url}/{year}/{year}{m:02}011400.pcap.gz"),
                        format!("{new_base_path}/{year}_{m:02}.pcap.gz"),
                    )),
                }
            }
        }
    }

    pub fn format_custom(base_url: Arc<String>, base_path: Arc<String>, year: u16, month: u8, day: Option<u8>) -> (String, String) {
        let new_base_path: String = create_path(&base_path);
        match day {
            Some(d) => (
                format!("{base_url}/{year}/{year}{month:02}{d:02}1400.pcap.gz"),
                format!("{new_base_path}/{year}_{month:02}_{d:02}.pcap.gz"),
            ),
            None => (
                format!("{base_url}/{year}/{year}{month:02}011400.pcap.gz"),
                format!("{new_base_path}/{year}_{month:02}.pcap.gz"),
            ),
        }
    }
}

/// Program to download and parse PCAP files from MAWILab
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLI {
    /// Type of program
    #[command(subcommand)]
    program: Program,

    #[arg(short, long, default_value_t = 4, required = false)]
    max_threads: usize,
}

impl CLI {
    pub fn program(&self) -> Program {
        self.program.clone()
    }

    pub fn num_threads(&self) -> usize {
        self.max_threads
    }
}
