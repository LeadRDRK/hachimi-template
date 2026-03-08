use std::{fs, path::{Path, PathBuf}, process, sync::{atomic::{self, AtomicBool}, Arc}};
use arc_swap::ArcSwap;
use fnv::FnvHashSet;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::{hachimi_impl, il2cpp};

use super::{game::Game, utils, Error, Interceptor};

pub struct Hachimi {
    // Hooking stuff
    pub interceptor: Interceptor,
    pub hooking_finished: AtomicBool,

    // Shared properties
    pub game: Game,
    pub config: ArcSwap<Config>,
}

static INSTANCE: OnceCell<Arc<Hachimi>> = OnceCell::new();

impl Hachimi {
    pub fn init() -> bool {
        if INSTANCE.get().is_some() {
            warn!("Hachimi should be initialized only once");
            return true;
        }

        let instance = match Self::new() {
            Ok(v) => v,
            Err(e) => {
                super::log::init(false); // early init to log error
                error!("Init failed: {}", e);
                return false;
            }
        };

        super::log::init(instance.config.load().debug_mode);

        info!("Hachimi {}", env!("HACHIMI_DISPLAY_VERSION"));
        instance.load_localized_data();

        INSTANCE.set(Arc::new(instance)).is_ok()
    }

    pub fn instance() -> Arc<Hachimi> {
        INSTANCE.get().unwrap_or_else(|| {
            error!("FATAL: Attempted to get Hachimi instance before initialization");
            process::exit(1);
        }).clone()
    }

    pub fn is_initialized() -> bool {
        INSTANCE.get().is_some()
    }

    fn new() -> Result<Hachimi, Error> {
        let game = Game::init();
        let config = Self::load_config(&game.data_dir)?;

        Ok(Hachimi {
            interceptor: Interceptor::default(),
            hooking_finished: AtomicBool::new(false),

            game,
            config: ArcSwap::new(Arc::new(config))
        })
    }

    fn load_config(data_dir: &Path) -> Result<Config, Error> {
        let config_path = data_dir.join("config.json");
        if fs::metadata(&config_path).is_ok() {
            let json = fs::read_to_string(&config_path)?;
            Ok(serde_json::from_str(&json)?)
        }
        else {
            Ok(Config::default())
        }
    }

    pub fn reload_config(&self) {
        let new_config = match Self::load_config(&self.game.data_dir) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to reload config: {}", e);
                return;
            }
        };
        self.config.store(Arc::new(new_config));
    }

    pub fn save_config(&self, config: &Config) -> Result<(), Error> {
        fs::create_dir_all(&self.game.data_dir)?;
        let config_path = self.get_data_path("config.json");
        utils::write_json_file(config, &config_path)?;

        Ok(())
    }

    pub fn save_and_reload_config(&self, config: Config) -> Result<(), Error> {
        self.save_config(&config)?;
        self.config.store(Arc::new(config));
        Ok(())
    }

    pub fn load_localized_data(&self) {
    }

    pub fn on_dlopen(&self, filename: &str, handle: usize) -> bool {
        // Prevent double initialization
        if self.hooking_finished.load(atomic::Ordering::Relaxed) { return false; }

        if hachimi_impl::is_il2cpp_lib(filename) {
            info!("Got il2cpp handle");
            il2cpp::symbols::set_handle(handle);
            if let Err(e) = il2cpp::hook::bootstrap::init() {
                error!("{}", e);
            }
            true
        }
        else {
            false
        }
    }

    pub fn on_hooking_finished(&self) {
        if self.hooking_finished.load(atomic::Ordering::Relaxed) { return; }
        self.hooking_finished.store(true, atomic::Ordering::Relaxed);

        info!("GameAssembly finished loading");
        il2cpp::symbols::init();
        il2cpp::hook::init();

        hachimi_impl::on_hooking_finished(self);
    }

    pub fn get_data_path<P: AsRef<Path>>(&self, rel_path: P) -> PathBuf {
        self.game.data_dir.join(rel_path)
    }
}

fn default_serde_instance<'a, T: Deserialize<'a>>() -> Option<T> {
    let empty_data = std::iter::empty::<((), ())>();
    let empty_deserializer = serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(empty_data);
    T::deserialize(empty_deserializer).ok()
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub debug_mode: bool,

    #[serde(default)]
    pub disabled_hooks: FnvHashSet<String>,

    #[cfg(target_os = "windows")]
    #[serde(flatten)]
    pub windows: hachimi_impl::Config,

    #[cfg(target_os = "android")]
    #[serde(flatten)]
    pub android: hachimi_impl::Config
}

impl Config {
}

impl Default for Config {
    fn default() -> Self {
        default_serde_instance().expect("default instance")
    }
}