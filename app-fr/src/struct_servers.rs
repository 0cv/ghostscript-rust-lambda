use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Servers {
    servers: BTreeMap<String, Server>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Server {
    active: String
}