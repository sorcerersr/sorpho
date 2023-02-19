use dirs;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{fs::read_to_string, fs::File, path::PathBuf, str::FromStr};

/// a representation of the applications configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    photos_dir: String,
}

/// the errors that can be returned by the config functions
#[derive(Debug)]
pub enum ConfigError {
    Path,
    DefaultCreate,
    DefaultWrite,
    Write,
    Read,
    Deserialize,
}

pub type ConfigResult = Result<Config, ConfigError>;

impl Config {
    /// Initializes the Configuration by either reading an existing configuration file
    /// (config.toml) or by creating a new configuration file with default values (leveraging
    /// xdg to find the path). On Linux the configuration path and filename will most likely
    /// be something like /home/username/.config/sorpho/config.toml
    pub fn init() -> ConfigResult {
        // use xdg crate to get the directories provided by the xdg standard
        let xdg_dirs = match xdg::BaseDirectories::with_prefix("sorpho") {
            Ok(base_directories) => base_directories,
            Err(error) => {
                log::error!("failed to get xdg base directories: {}", error);
                return Err(ConfigError::Path);
            }
        };

        // further use xdg to construct the path to the config.toml according to the xdg standard
        // note: when xdg will create directories if they do not exists
        let config_file = match xdg_dirs.place_config_file("config.toml") {
            Ok(pathbuf) => pathbuf.to_owned(),
            Err(error) => {
                log::error!("failed to place config file: {}", error);
                return Err(ConfigError::Path);
            }
        };

        // check if the config file already exists and when id is missing create
        // a file with default values
        Config::create_default_config_when_it_does_not_exist(&config_file)?;

        // read the whole content of the file
        let toml_content = match read_to_string(config_file) {
            Err(error) => {
                log::error!("failed read config file: {}", error);
                return Err(ConfigError::Read);
            }
            Ok(result) => result,
        };

        // finally deserialize the content from the config file to the Config struct
        let config: Config = match toml::from_str(&toml_content) {
            Err(error) => {
                log::error!("failed deserialize to Config struct: {}", error);
                return Err(ConfigError::Deserialize);
            }
            Ok(result) => result,
        };

        Ok(config)
    }

    // Checks if the file given as a &PathBuf already exists and if not create
    // it using the default values of the Config struct
    fn create_default_config_when_it_does_not_exist(path: &PathBuf) -> Result<(), ConfigError> {
        if path.exists() {
            return Ok(());
        };
        let config = Config::default();
        let toml = toml::to_string(&config).unwrap();

        let mut file = match File::create(&path) {
            Err(why) => {
                log::error!("couldn't create {}: {}", path.display(), why);
                return Err(ConfigError::DefaultCreate);
            }
            Ok(file) => file,
        };

        match file.write_all(toml.as_bytes()) {
            Err(why) => {
                log::error!("couldn't write {}: {}", path.display(), why);
                return Err(ConfigError::DefaultWrite);
            }
            Ok(_) => Ok(()),
        }
    }
}

impl Default for Config {
    // Providing defaults for the Config struct that are also used as the
    // default application configuration when there is no existing config file.
    fn default() -> Self {
        let mut photos_dir = dirs::home_dir().unwrap();
        photos_dir.push("sorpho_photos");
        Config {
            photos_dir: String::from_str(photos_dir.to_str().unwrap()).unwrap(),
        }
    }
}
