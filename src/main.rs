use color_eyre::eyre::eyre;
use fluxer_rs::{error::FluxerRsError, fluxerbot::FluxerBot};
use std::env;
use std::process::exit;
use dotenvy::dotenv;

use crate::dispatch::FluxDashDispatchHandler;

pub mod commands;
pub mod dispatch;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), FluxerRsError> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    dotenv().ok();

    let fluxer_bot_token = env::var("FLUXER_BOT_TOKEN");

    if fluxer_bot_token.is_err() {
        println!("ERROR: No bot token! Please set FLUXER_BOT_TOKEN.");
        exit(1);
    } else if fluxer_bot_token.clone().unwrap_or(String::from("0")).len() != 63 {
        println!("ERROR: Bot token must be 63 characters long! Please set correct FLUXER_BOT_TOKEN.");
        exit(1);
    }

    let aredl_api_token = env::var("AREDL_API_TOKEN");

    if aredl_api_token.is_err() {
        let _ = eyre!("ERROR: No AREDL API token! Please set AREDL_API_TOKEN.");
        exit(1);
    } else if aredl_api_token.clone().unwrap_or(String::from("0")).len() != 268 {
        let _ = eyre!("ERROR: AREDL API token must be 63 characters long! Please set correct AREDL_API_TOKEN.");
        exit(1);
    }
    
    println!("Bot token accepted. If \"Invalid Token\" appears in the debug output, please double check that the token is correct.");

    let bot = FluxerBot::init(
        fluxer_bot_token.unwrap(),
        "wss://gateway.fluxer.app?v=1&encoding=json&compress=none",
        "https://api.fluxer.app/v1",
    )?;

    bot.start(FluxDashDispatchHandler {}).await;

    Ok(())
}