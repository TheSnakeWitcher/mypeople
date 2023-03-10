use config::{self, Config};
use serde::Deserialize;

const APP_NAME: &str = "mypeople";
// config file
const CONFIG_FILE_DIR: &str = APP_NAME;
const CONFIG_FILE_NAME: &str = APP_NAME;
const CONFIG_FILE_EXTENSION: &str = "toml";
// dbfile
const DBFILE_KEY: &str = "dbfile";
const DBFILE_DIR: &str = APP_NAME;
const DBFILE_NAME: &str = APP_NAME;
const DBFILE_EXTENSION: &str = "db";

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub dbfile: String,
}

pub fn init() -> Option<Conf> {
    let config_dir = dirs::config_dir().expect("failed to load XDG_CONFIG_DIR");
    let cache_dir = dirs::cache_dir().expect("failed to load XDG_CACHE_DIR");

    let app_config_file = {
        let mut app_config_dir = config_dir.join(CONFIG_FILE_DIR);
        app_config_dir.push(CONFIG_FILE_NAME);
        app_config_dir.set_extension(CONFIG_FILE_EXTENSION);
        app_config_dir
    };

    let default_dbfile = {
        let mut app_cache_dir = cache_dir.join(DBFILE_DIR);
        app_cache_dir.push(DBFILE_NAME);
        app_cache_dir.set_extension(DBFILE_EXTENSION);
        app_cache_dir
    };

    let conf_builder = Config::builder()
        .set_default(DBFILE_KEY, default_dbfile.to_str())
        .expect("failod to load default config")
        .add_source(config::File::from(app_config_file).required(false))
        .add_source(config::Environment::with_prefix(APP_NAME));

    match conf_builder.build() {
        Ok(cfg) => {
            return Some(
                cfg.try_deserialize::<Conf>()
                    .expect("error deserializing config"),
            );
        }
        Err(e) => {
            println!("configuration error: {}", e);
            None
        }
    }
}
