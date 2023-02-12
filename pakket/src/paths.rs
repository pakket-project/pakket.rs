use std::path::PathBuf;

use directories::ProjectDirs;

pub enum Path {
    StateDir,
    Lockfile,
}

impl Path {
    fn get_project_dirs() -> ProjectDirs {
        ProjectDirs::from("", "", env!("CARGO_PKG_NAME")).unwrap()
    }

    pub fn to_path_buf(&self) -> PathBuf {
        match self {
            Self::StateDir => Self::get_project_dirs().config_dir().to_path_buf(),
            Self::Lockfile => Self::StateDir.to_path_buf().join("lock.toml"),
        }
    }
}
