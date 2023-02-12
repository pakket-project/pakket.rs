use std::{
    fs, io,
    path::{Path, PathBuf},
};

use super::Repository;
use crate::package::{Package, PackageVersion};

use semver::Version;

pub struct FsRepository {
    path: PathBuf,
}

pub enum FsError {
    IO(io::Error),
    Deserialize(toml::de::Error),
}

impl From<io::Error> for FsError {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<toml::de::Error> for FsError {
    fn from(err: toml::de::Error) -> Self {
        Self::Deserialize(err)
    }
}

impl FsRepository {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_owned(),
        }
    }
}

impl Repository for FsRepository {
    type Error = FsError;

    fn get(&self, name: &str) -> Result<Package, Self::Error> {
        let pkg_data = fs::read_to_string(self.path.join(name).join("package.toml"))?;
        Ok(toml::from_str(&pkg_data)?)
    }

    fn get_version(&self, _name: &str, _version: Version) -> Result<PackageVersion, Self::Error> {
        todo!()
    }
}
