#![allow(warnings)]

use std::{io::Write, path::PathBuf};

use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;

use super::error::Error;
use log::info;
use reqwest::{StatusCode, Url, get};
use std::time::{Duration, Instant};

pub async fn page_exist(url: &Url) -> Result<(), Error> {
    let u = url.as_str();
    let r = get(u).await.unwrap().status();

    if r == StatusCode::OK {
        return Ok(());
    }

    return Err(Error::UrlNotFound);
}

pub async fn download_file(url: &Url, output_path: PathBuf) -> Result<(), Error> {
    match page_exist(url).await {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    let mut r = get(url.as_str()).await.unwrap();
    let total = r.content_length();
    let filename = output_path.file_name().unwrap().to_str().unwrap();
    info!("Started downloading {filename}");
    match total {
        Some(c) => {
            info!("[{url}] Total file size: {:.2}GB", c as f32 / 1024.0_f32.powf(3.0));
        }
        None => {}
    }

    let mut opt = OpenOptions::new();
    let mut f = opt
        .create(true)
        .write(true)
        .open(output_path)
        .await
        .unwrap();

    let mut total_bytes_downloaded: usize = 0;
    let mut start = Instant::now();
    let mut seconds: u64 = 0;
    let mut byte_window: usize = 0;
    let n_secs: u64 = 10;
    while let Some(chunk) = r.chunk().await.unwrap() {
        let chunk_size = chunk.len();
        f.write_all(&chunk).await.unwrap();
        total_bytes_downloaded += chunk_size;
        byte_window += chunk_size;

        if start.elapsed() >= Duration::from_secs(n_secs) {
            start = Instant::now();
            seconds += n_secs;
            let avg = total_bytes_downloaded as f64 / seconds as f64 / 1024.0_f64.powf(2.0);
            let bps = byte_window as f64 / n_secs as f64 / 1024.0_f64.powf(2.0);
            let perc: f64 = (total_bytes_downloaded as f64 / total.unwrap() as f64) * 100.0;
            byte_window = 0;

            info!(
                "[{url}] Average {:.2}MB/s\tPer second: {:.2}MB/s\tTotal downloaded: {:.2}%",
                avg, bps, perc
            );
        }
    }

    f.sync_all().await.unwrap();
    info!("Done");

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    const INVALID: &'static str = "https://somethingthatdoesntexist.com/";
    const VALID: &'static str = "https://prhrck.com/";

    #[tokio::test]
    async fn test_page_exist() {
        assert!(page_exist(&Url::parse(VALID).unwrap()).await.is_ok());
    }

    #[tokio::test]
    async fn test_page_dont_exist() {
        assert!(page_exist(&Url::parse(INVALID).unwrap()).await.unwrap_err() == Error::UrlNotFound);
    }

    #[tokio::test]
    async fn test_download() {
        let p = Path::new("/home/pedro/Documents/pcap/data/ok.html").to_path_buf();
        let u = Url::parse(VALID).unwrap();

        let _ = download_file(&u, p.clone()).await.is_ok();

        assert!(p.exists());

        std::fs::remove_file(p).unwrap();
    }
}
