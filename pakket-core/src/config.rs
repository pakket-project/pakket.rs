use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct Config {
    pub repositories: Vec<Repository>,
}

#[derive(Deserialize)]
pub struct Repository {
    pub name: String,
    #[serde(flatten)]
    pub protocol: Protocol,
}

#[derive(Deserialize)]
#[serde(tag = "protocol", rename_all = "camelCase")]
pub enum Protocol {
    Fs { path: String },
}
