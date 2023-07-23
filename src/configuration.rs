use config::{self, Config};
use serde::Deserialize;

const APP_NAME: &str = "mypeople";

const USER_CONFIG_FILE_KEY: &str = "user_config_file";
const CONFIG_FILE_DIR: &str = APP_NAME;
const CONFIG_FILE_NAME: &str = APP_NAME;
const CONFIG_FILE_EXTENSION: &str = "toml";

const DBFILE_KEY: &str = "dbfile";
const DBFILE_DIR: &str = APP_NAME;
const DBFILE_NAME: &str = APP_NAME;
const DBFILE_EXTENSION: &str = "db";

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub dbfile: String,
    pub user_config_file: String,
}

pub fn init() -> Option<Conf> {
    let (config_dir, data_dir) = (
        dirs::config_dir().expect("failed to load XDG_CONFIG_DIR"),
        dirs::data_dir().expect("failed to load XDG_DATA_DIR"),
    );

    let user_config_file = {
        let mut user_config_dir = config_dir.join(CONFIG_FILE_DIR);
        user_config_dir.push(CONFIG_FILE_NAME);
        user_config_dir.set_extension(CONFIG_FILE_EXTENSION);
        user_config_dir
    };

    let default_dbfile = {
        let mut app_data_dir = data_dir.join(DBFILE_DIR);
        app_data_dir.push(DBFILE_NAME);
        app_data_dir.set_extension(DBFILE_EXTENSION);
        app_data_dir
    };

    let conf_builder = Config::builder()
        .set_default(DBFILE_KEY, default_dbfile.to_str())
        .expect("failod to load default dbfile from config")
        .set_default(
            USER_CONFIG_FILE_KEY,
            user_config_file
                .to_str()
                .expect("failed to convert to str config file"),
        )
        .expect("failod to load default user config file")
        .add_source(config::File::from(user_config_file).required(false))
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
