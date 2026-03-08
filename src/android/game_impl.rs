use procfs::process::Process;
use std::{path::{Path, PathBuf}, process};

use crate::android::zygisk;

pub fn get_package_name() -> String {
    match zygisk::get_package_name() {
        Some(name) => name.clone(),
        None => {
            let proc = Process::myself().unwrap_or_else(|_| {
                error!("FATAL: Failed to read /proc/self");
                process::exit(1);
            });
            let cmdline = proc.cmdline().unwrap_or_else(|_| {
                error!("FATAL: Failed to read /proc/self/cmdline");
                process::exit(1);
            });
            cmdline.get(0).unwrap_or_else(|| {
                error!("FATAL: Invalid cmdline");
                process::exit(1);
            }).to_owned()
        }
    }
}

pub fn get_data_dir(package_name: &str) -> PathBuf {
    let mut path = Path::new("/sdcard/Android/media").join(package_name);
    path.push(env!("CARGO_PKG_NAME"));
    path
}