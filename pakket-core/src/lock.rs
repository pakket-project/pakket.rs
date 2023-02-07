use semver::Version;

pub struct Lockfile {
    pub packages: Vec<Package>,
}

pub struct Package {
    pub name: String,
    pub version: Version,
    pub checksum: u32,
}
