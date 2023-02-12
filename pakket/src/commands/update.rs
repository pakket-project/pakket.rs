use std::fs::{self, File};
use std::io::Write;

use pakket::paths::Path;
use pakket_core::lock::Lockfile;

pub fn update() {
    fs::create_dir_all(Path::StateDir.to_path_buf()).expect("state directory could not be created");

    let mut f = File::create(Path::Lockfile.to_path_buf())
        .expect("lockfile could not be opened for writing");

    let output = toml::to_string(&Lockfile::default()).expect("lockfile could not be serialized");
    write!(f, "{output}").expect("failed writing to lockfile");
}
