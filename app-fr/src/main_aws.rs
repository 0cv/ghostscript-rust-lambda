#[cfg(bootstrap)]

extern crate ftp;
extern crate pdf2tiff;

#[macro_use]
extern crate error_chain;

use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use std::{fs, borrow::Borrow};
use std::result::Result;

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

#[derive(Deserialize)]
struct Request {
    file: String,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    base64: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // this needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<Request>) -> Result<(), errors::Error>{
    if let Err(e) = run() {
        println!("error ====> {}", e);

        match e.kind() {
            &ErrorKind::Ftp(ref s) => println!("ftp error => {}",s),
            &ErrorKind::Io(ref s) => println!("io {}",s),
            &ErrorKind::Msg(ref s) => println!("msg {}",s),
            &ErrorKind::Pdf(ref s) => println!("pdf error => {}",s),
            &ErrorKind::Yaml(ref s) => println!("yaml error => {}",s),
            &_ => println!("unhandled error!!"),
        }

        return Err(e);
    }

    Ok(())
}


fn run() -> errors::Result<()> {
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

