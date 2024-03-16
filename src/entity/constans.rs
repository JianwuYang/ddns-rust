use crate::utils::file_utils::{self, Args};

lazy_static! {
    static ref CONFIG: Args = file_utils::handle_path();
    pub static ref CONFIG_PATH: String = CONFIG.config_path.as_ref().unwrap().to_owned();
    pub static ref CACHE_PATH: String = CONFIG.cache_path.as_ref().unwrap().to_owned();
}