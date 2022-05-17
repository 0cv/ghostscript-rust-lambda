extern crate anyhow;
extern crate aws_sdk_config;
extern crate aws_sdk_appconfigdata;
extern crate retry;

mod struct_ftp;

use anyhow::Result;
use std::fs;
use struct_ftp::FtpServer;
use retry::{retry, delay::Fixed};


fn main() {
    if let Err(e) = run() {
        if let Some(my_error) = e.downcast_ref::<pdf2tiff::PdfError>() {
            match my_error {
                pdf2tiff::PdfError::BuilderError{kind, code, user_data} => println!("builder error {}, {}, {}", kind, code, user_data),
                pdf2tiff::PdfError::PdfRunningInstanceException(s) => println!("Instance run {}", e),
            }
        } else if let Some(my_error) = e.downcast_ref::<ftp::FtpError>() {
            println!("ftp error: {}", my_error);
        } else if let Some(my_error) = e.downcast_ref::<retry::Error<ftp::FtpError>>() {
            println!("retry ftp error: {}", my_error);
        } else {
            println!("something else: {}", e);
        }

        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let file_config: String = fs::read_to_string("ftp_servers.yaml")?;

    let config: FtpServer = serde_yaml::from_str(&file_config)?;

    println!("config: {:?}", config);
    
    let ftp_connect = retry(Fixed::from_millis(1000).take(3), || {
        println!("Retry ..");
        ftp::FtpBuilder::new(
            &config.servers["insmedaws"].username, 
            &config.servers["insmedaws"].password, 
            &config.servers["insmedaws"].host,
        )
    })?;

    let res = retry(Fixed::from_millis(1000).take(3), || {
        ftp_connect.list_dir("/")
    })?;

    let file = retry(Fixed::from_millis(1000).take(3), || { 
        ftp_connect.download("/QA/Inbound/Insmed_Promotional_Material_Request_File_20220506.txt")
    })?;

    println!("file.. {:?}", file);

    fs::write("./file.txt", file)?;

    // let pdf = pdf2tiff::builder::PdfBuilder::new();
    // let tiff_base64 = pdf.convert(config.base64)?;

    // println!("tiff_base64 => {:?}", tiff_base64);

    Ok(())
}