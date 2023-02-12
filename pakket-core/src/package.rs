use semver::Version;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub description: String,
    pub version: Version,
    pub license: String,
    pub homepage: String,
}

#[derive(Deserialize)]
pub struct PackageVersion {
    pub sources: Vec<String>,
    pub dependencies: PackageDependencies,
    pub platforms: Vec<PackagePlatform>,
}

#[derive(Deserialize)]
pub struct PackageDependencies {
    pub runtime: Vec<PackageDependency>,
    pub build: Vec<PackageDependency>,
}

#[derive(Deserialize)]
pub struct PackageDependency;

#[derive(Deserialize)]
pub struct PackagePlatform {
    pub checksum: String,
    #[serde(flatten)]
    pub platform: PlatformData,
}

#[derive(Deserialize)]
pub enum PlatformData {
    Amd64,
    Arm64,
}
