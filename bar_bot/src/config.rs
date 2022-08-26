extern crate envy;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Env {
    pub bot_id: String,
    pub rust_log: String,
    pub workers_number: usize,
    pub blocking_treads: usize,
}
