extern crate core;

use bar_bot::config::Env;
use bar_bot::TelegrammBuilder;

fn main() {
    let _ = dotenv::dotenv();
    let env = match envy::from_env::<Env>() {
        Ok(env) => env,
        Err(err) => {
            log::error!("Error occurs {}", err);
            panic!();
        }
    };
    TelegrammBuilder::run(env);
}

