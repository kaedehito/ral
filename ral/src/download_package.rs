use reqwest::blocking;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::fs::File;
use std::io::{Write, Read};
use crate::{dprintln, file_paths};


#[allow(warnings)]
pub fn download_package(url: &str, package_name: &String, version: String) -> Result<(), Box<dyn Error>>{
    dprintln!("sending http reqwest...");
    let mut response = blocking::get(url)?;
    let total_size = response.content_length().ok_or("Failed to get content length")?;
    dprintln!("get: "; total_size);
    let path = file_paths::get_ral_build().join(package_name);

    let mut file = File::create(path)?;
    dprintln!("file created");
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}[{bar:40.white/white}]  {bytes}/{total_bytes} ({eta})")?
        .progress_chars("#>-"));
        let message = format!("{}-{}",package_name.to_string(), version);
    dprintln!("download package: "; message);
    pb.set_message(message);

    let mut downloaded: u64 = 0;
    let mut buffer = [0; 4096];

    while let Ok(bytes_read) = response.read(&mut buffer){
        if bytes_read == 0{
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
        downloaded += bytes_read as u64;

        pb.set_position(downloaded);

    }
    pb.finish();
    file.flush();
    Ok(())
}
