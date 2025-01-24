use crate::{Error, Result};
use std::{env, sync::OnceLock};


pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> =     OnceLock::new();

    INSTANCE.get_or_init(|| -> Config{
        Config::load_from_env().unwrap_or_else(|ex| -> Config{
            panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
        })
    })
}

#[allow(non_snake_case)]
pub struct Config{
    // -- DB
    pub DB_URL:String,
    // -- Web
    pub WEB_FOLDER:String,


}

impl  Config {
    
    fn load_from_env() -> Result<Self> {
        Ok(Config {
            WEB_FOLDER:get_name("SERVICE_WEB_FOLDER")?,
            DB_URL:get_name("SERVICE_DB_URL")?,
        })
    }
}

fn get_name(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))

}