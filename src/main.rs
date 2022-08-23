extern crate core;

use std::alloc::alloc;
use std::collections::HashMap;

use serde::{Deserialize, Deserializer};
use serde_json::Value;
use Bar_Bot::TelegrammBuilder;
use Bar_Bot::config::Env;


mod telegramm;
mod coctails_api;
mod error;
mod config;
mod utils;

#[tokio::main]
async fn main() {
dotenv::dotenv();
  let env =  match envy::from_env::<Env>() {
        Ok(env) => env,
        Err(err) => {
            panic!("{}", err)
        }
    };
    TelegrammBuilder::run(env).await;

}
