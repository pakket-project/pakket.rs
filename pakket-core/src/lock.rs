use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Lockfile {
    pub packages: Vec<Package>,
}

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub checksum: u32,
}
