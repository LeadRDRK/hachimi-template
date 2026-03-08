use std::path::{Path, PathBuf};

use super::utils;

pub fn get_package_name() -> String {
    utils::get_exec_path()
        .to_str()
        .unwrap()
        .to_owned()
}

pub fn get_data_dir(package_name: &str) -> PathBuf {
    Path::new(package_name)
        .parent()
        .unwrap()
        .join(env!("CARGO_PKG_NAME"))
}