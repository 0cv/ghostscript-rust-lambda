extern crate anyhow;
extern crate aws_sdk_config;
extern crate aws_sdk_appconfigdata;

use serde::{Serialize, Deserialize};
use std::fs;
use anyhow::Result;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config{
    user: String,
    pass: String,
    addr: String,
    base64: String,
}

fn main() {
    if let Err(e) = run() {
        if let Some(my_error) = e.downcast_ref::<pdf2tiff::PdfError>() {
            match my_error {
                pdf2tiff::PdfError::BuilderError{kind, code, user_data} => println!("builder error {}, {}, {}", kind, code, user_data),
                pdf2tiff::PdfError::PdfRunningInstanceException(s) => println!("Instance run {}", e),
            }
        } else if let Some(my_error) = e.downcast_ref::<ftp::FtpError>() {
            println!("pdf transformation error: {}", my_error);
        } else {
            println!("something else: {}", e);
        }

        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let file_config: String = fs::read_to_string("app-fr/config.yaml")?;

    let config: Config = serde_yaml::from_str(&file_config)?;
    
    let ftp_connect = ftp::builder::FtpBuilder::new(&config.user, &config.pass, &config.addr)?;

    let res = ftp_connect.list_dir("/")?;


    let res = ftp_connect.list_dir("/QA")?;

    let file = ftp_connect.download("/QA/Inbound/Insmed_Promotional_Material_Request_File_20220506.txt")?;

    println!("file.. {:?}", file);

    fs::write("./file.txt", file)?;

    let pdf2tiff = pdf2tiff::builder::PdfBuilder::new(&config.base64);
    let tiff_base64 = pdf2tiff.convert()?;

    println!("tiff_base64 => {:?}", tiff_base64);

    Ok(())
}