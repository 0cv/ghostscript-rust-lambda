#[cfg(bootstrap)]

extern crate aws_sdk_appconfigdata;
extern crate aws_smithy_http;
extern crate ftp;
extern crate pdf2tiff;
extern crate anyhow;

mod load_config;

use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Result;

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

async fn handler(event: LambdaEvent<Request>) -> Result<()>{
    if let Err(e) = run().await {
        println!("error ====> {}", e);

        if let Some(my_error) = e.downcast_ref::<pdf2tiff::PdfError>() {
            match my_error {
                pdf2tiff::PdfError::BuilderError{kind, code, user_data} => println!("builder error {}, {}, {}", kind, code, user_data),
                pdf2tiff::PdfError::PdfRunningInstanceException(s) => println!("Instance run {}", e),
            }
        } else if let Some(my_error) = e.downcast_ref::<load_config::LoadConfigError>() {
            println!("configuration loading error: {}", my_error);
        } else if let Some(my_error) = e.downcast_ref::<ftp::FtpError>() {
            println!("pdf transformation error: {}", my_error);
        } else {
            println!("something else: {}", e);
        }

        return Err(e);
    }

    Ok(())
}


async fn run() -> Result<()> {
    let config = load_config::load_config().await?;

    println!("config : {}", config);

    let config: Config = serde_yaml::from_str(&config)?;
    
    let ftp_connect = ftp::builder::FtpBuilder::new(&config.user, &config.pass, &config.addr)?;

    let res = ftp_connect.list_dir("/QA")?;

    let file = ftp_connect.download("/QA/Inbound/Insmed_Promotional_Material_Request_File_20220506.txt")?;

    println!("file.. {:?}", file);

    fs::write("./file.txt", file)?;

    let tiff_base64 = pdf2tiff::builder::PdfBuilder::new(&config.base64).convert()?;

    println!("tiff_base64 => {:?}", tiff_base64);

    Ok(())
}
