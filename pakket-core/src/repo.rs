pub mod fs;

use semver::Version;

use crate::package::{Package, PackageVersion};

pub trait Repository {
    type Error;

    fn get(&self, name: &str) -> Result<Package, Self::Error>;
    fn get_version(&self, name: &str, version: Version) -> Result<PackageVersion, Self::Error>;
}
