use semver::Version;

pub struct Lockfile {
    pub name: String,
    pub version: Version,
    pub checksum: u32,
}
