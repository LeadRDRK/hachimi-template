use std::path::PathBuf;

use crate::game_impl;

pub struct Game {
    pub package_name: String,
    pub data_dir: PathBuf
}

impl Game {
    pub fn init() -> Game {
        let package_name = game_impl::get_package_name();
        let data_dir = game_impl::get_data_dir(&package_name);

        Game {
            package_name,
            data_dir,
        }
    }
}