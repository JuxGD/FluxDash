use color_eyre::eyre::{self, eyre};
use fluxer_rs::{context::BotContext, client::FluxerClient, high_level::command_handler::CommandHandler};
use std::env;
use std::process::exit;
use dotenvy::dotenv;

use crate::commands::admin_setup::setup::send_gd_server_update;
use crate::dispatch::FluxDashDispatchHandler;

pub mod commands;
pub mod dispatch;
pub mod utils;

#[deny(clippy::pedantic, clippy::nursery)]

#[tokio::main]
async fn main() -> Result<(), eyre::Report> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let wss_endpoint = "wss://gateway.fluxer.app?v=1&encoding=json&compress=none";
    let api_endpoint = "https://api.fluxer.app/v1";

    let cmd_handler = CommandHandler::init(String::from("gd!"));

    dotenv().ok();

    let fluxer_bot_token = env::var("FLUXER_BOT_TOKEN");

    let fluxer_bot_token = if fluxer_bot_token.is_err() {
        println!("ERROR: No bot token! Please set FLUXER_BOT_TOKEN. If it's set, something else might have gone wrong.");
        exit(1);
    } else if fluxer_bot_token.clone()?.len() != 63 {
        println!("ERROR: Bot token must be 63 characters long! Please set correct FLUXER_BOT_TOKEN.");
        exit(1);
    } else {
        fluxer_bot_token?
    };

    let aredl_api_token = env::var("AREDL_API_TOKEN");

    if aredl_api_token.is_err() {
        let _: Result<_, color_eyre::eyre::Error> = Err::<(), color_eyre::eyre::Report>(eyre!("ERROR: No AREDL API token! Please set AREDL_API_TOKEN."));
        exit(1);
    } else if aredl_api_token.clone()?.len() != 268 {
        let _: Result<_, color_eyre::eyre::Error> = Err::<(), color_eyre::eyre::Report>(eyre!("ERROR: AREDL API token must be 63 characters long! Please set correct AREDL_API_TOKEN."));
        exit(1);
    }
    
    println!("Bot token accepted. If \"Invalid Token\" appears in the debug output, please double check that the token is correct.");

    let bot_context = BotContext::new(fluxer_bot_token.clone(), wss_endpoint, api_endpoint, cmd_handler).await?;

    FluxerClient::start(bot_context, FluxDashDispatchHandler, async |api| {let _ = tokio::spawn(send_gd_server_update(api)).await; }).await;

    Ok(())
}