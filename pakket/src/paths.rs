use std::path::PathBuf;

use directories::ProjectDirs;

pub enum Path {
    Lockfile,
}

impl Path {
    fn get_project_dirs(&self) -> ProjectDirs {
        ProjectDirs::from("", "", "pakket").unwrap()
    }

    pub fn to_path_buf(&self) -> PathBuf {
        match self {
            Self::Lockfile => self.get_project_dirs().config_dir().into(),
        }
    }
}
