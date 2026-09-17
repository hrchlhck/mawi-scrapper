pub mod error;

mod http;

use log::info;
use reqwest::Url;
use std::fs::create_dir;
use std::path::Path;
use std::path::absolute;

pub async fn run(url: String, output_path: String) -> Result<(), error::Error> {
    http::download_file(
        &Url::parse(&url).unwrap(),
        Path::new(&output_path).to_path_buf(),
    )
    .await
}

pub fn create_path(base_output_path: &String) -> String {
    let base_path = absolute(Path::new(&base_output_path)).unwrap();

    if !base_path.exists() {
        match create_dir(base_path.clone()) {
            Ok(_) => {},
            Err(e) => {}
        }
        info!("Created {base_path:#?}");
    }

    base_path.to_str().unwrap().to_string()
}
