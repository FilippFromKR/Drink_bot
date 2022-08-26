extern crate core;

use Bar_Bot::config::Env;
use Bar_Bot::TelegrammBuilder;

fn main() {
    dotenv::dotenv();
    let env = match envy::from_env::<Env>() {
        Ok(env) => env,
        Err(err) => {
            log::error!("Error occurs {}", err);
            panic!();
        }
    };
    TelegrammBuilder::run(env);
}
