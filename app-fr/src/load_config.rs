use aws_sdk_appconfigdata::Client;
use std::env::{var, VarError};
use anyhow::Result;

#[derive(Debug, thiserror::Error)]
pub enum LoadConfigError {
    #[error("Missing Variables")]
    AwsConfigLoadError(#[from] VarError),
    #[error("Start Configuration Session Error")]
    AwsSdkSession(#[from] ::aws_smithy_http::result::SdkError<::aws_sdk_appconfigdata::error::StartConfigurationSessionError>),
    #[error("Start Configuration Session Error")]
    AwsSdkConfiguration(#[from] ::aws_smithy_http::result::SdkError<::aws_sdk_appconfigdata::error::GetLatestConfigurationError>),
    #[error("UTF8 Error")]
    Utf8Error(#[from] ::std::string::FromUtf8Error),
}

pub async fn load_config() -> Result<String, LoadConfigError> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let application_identifier = var("AppId")?;
    
    let configuration_profile_identifier = var("ConfigId")?;
    
    let environment_identifier = var("EnvId")?;

    let session_output = client.start_configuration_session()
        .set_application_identifier(Some(application_identifier))
        .set_configuration_profile_identifier(Some(configuration_profile_identifier))
        .set_environment_identifier(Some(environment_identifier))
        .set_required_minimum_poll_interval_in_seconds(Some(15))
        .send()
        .await?;

    let config = client.get_latest_configuration()
        .set_configuration_token(session_output.initial_configuration_token)
        .send()
        .await?;

    let blob = config.configuration.unwrap();

    Ok(String::from_utf8(blob.into_inner())?)
}

