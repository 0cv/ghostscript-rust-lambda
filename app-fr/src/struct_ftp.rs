use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FtpServer {
    #[serde(rename="sftp-servers")]
    pub servers: BTreeMap<String, Server>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Server {
    pub host: String,
    pub username: String,
    pub password: String,
}
