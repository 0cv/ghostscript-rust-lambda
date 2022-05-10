#[macro_use]
extern crate error_chain;


use std::{fs, borrow::Borrow};
use serde::{Serialize, Deserialize};

mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            Ftp(::ftp::errors::Error);
            Yaml(::serde_yaml::Error);
            Pdf(::pdf2tiff::errors::Error);
        }
    }
}
use errors::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config{
    user: String,
    pass: String,
    addr: String,
    base64: String,
}

fn main() {
    if let Err(e) = run() {
        println!("error ====> {}", e);

        /////// look at the chain of errors... ///////
        // for e in e.iter().skip(1) {
        //     println!("caused by: {}", e);
        // }

        match e.kind() {
            &ErrorKind::Ftp(ref s) => println!("ftp error => {}",s),
            &ErrorKind::Io(ref s) => println!("io {}",s),
            &ErrorKind::Msg(ref s) => println!("msg {}",s),
            &ErrorKind::Pdf(ref s) => println!("pdf error => {}",s),
            &ErrorKind::Yaml(ref s) => println!("yaml error => {}",s),
            &_ => println!("unhandled error!!"),
        }

        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let file_config: String = fs::read_to_string("app-fr/config.yaml")?;

    let config: Config = serde_yaml::from_str(file_config.borrow())?;
    
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
