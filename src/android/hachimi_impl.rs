use serde::{Deserialize, Serialize};

use crate::core::Hachimi;

pub fn is_il2cpp_lib(filename: &str) -> bool {
    filename.ends_with("libil2cpp.so")
}

pub fn on_hooking_finished(_hachimi: &Hachimi) {
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub hook_libc_dlopen: bool
}

impl Config {
}