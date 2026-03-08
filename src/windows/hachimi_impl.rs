use serde::{Deserialize, Serialize};

use crate::core::Hachimi;

pub fn is_il2cpp_lib(filename: &str) -> bool {
    filename == "GameAssembly.dll"
}

pub fn on_hooking_finished(_hachimi: &Hachimi) {
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
}

impl Config {
}