use worker::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct Config {
    pub uuid: Uuid,
    pub main_page_url: String,
    pub sub_page_url: String,
}

impl Config {
    pub fn from_env(env: &Env) -> Result<Self> {
        Ok(Self {
            uuid: Uuid::parse_str(
                &env.var("UUID")?.to_string()
            ).unwrap(),

            main_page_url: env
                .var("MAIN_PAGE_URL")?
                .to_string(),

            sub_page_url: env
                .var("SUB_PAGE_URL")?
                .to_string(),
        })
    }
}
