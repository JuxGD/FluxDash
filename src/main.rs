use fluxer_rs::{error::FluxerRsError, fluxerbot::FluxerBot};
use std::env;
use std::process::exit;

use crate::dispatch::FluxDashDispatchHandler;

pub mod commands;
pub mod dispatch;

#[tokio::main]
async fn main() -> Result<(), FluxerRsError> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let bot_token = env::var("FLUXER_BOT_TOKEN");

    if bot_token.is_err() {
        println!("ERROR: No bot token! Please set FLUXER_BOT_TOKEN.");
        exit(1);
    } else if bot_token.clone().expect("REASON").len() != 63 {
        println!("ERROR: Bot token must be 63 characters long! Please set correct FLUXER_BOT_TOKEN.");
        exit(1);
    }

    let bot_token_real = bot_token.unwrap();
    println!("{}", bot_token_real);

    let bot = FluxerBot::init(
        bot_token_real,
        "wss://gateway.fluxer.app?v=1&encoding=json&compress=none",
        "https://api.fluxer.app/v1",
    )?;

    bot.start(FluxDashDispatchHandler {}).await;

    Ok(())
}