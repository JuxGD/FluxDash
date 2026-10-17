use color_eyre::eyre::{self, eyre};
use fluxer_rs::api::FluxerApiHandler;
use fluxer_rs::api::channels::Channel;
use fluxer_rs::context::FluxerClientContext;
use fluxer_rs::high_level::command_handler::CommandHandler;
use fluxer_rs::serde::types::message::Embed;
use fluxer_rs::{error::FluxerRsError, context::BotContext, client::FluxerClient};
use gdutils::gd::levels::get_timely_level;
use gdutils::gd::utils::TimelyType;
use itertools::Itertools;
use std::env;
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;
use dotenvy::dotenv;

use crate::commands::admin_setup::setup::{FluxDashGuildConfig, GDServerActivity};
use crate::dispatch::FluxDashDispatchHandler;
use crate::utils::levels::make_level_embed;

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

    FluxerClient::start(bot_context, FluxDashDispatchHandler, async |api| {println!("starting thing"); let _ = tokio::spawn(daily_thingy(api)).await; }).await;

    Ok(())
}

async fn daily_thingy(api: Arc<FluxerApiHandler>) -> Result<(), eyre::Report> {
    if tokio::fs::try_exists("guilds_configs").await? {

        loop {
            tokio::time::sleep(Duration::from_secs(30)).await;

            let daily = get_timely_level(TimelyType::Daily).await?;

            if tokio::fs::try_exists("cache/daily.txt").await? {
                if tokio::fs::read_to_string("cache/daily.txt").await? != daily.timely_index.to_string() {
                    tokio::fs::write("cache/daily.txt", daily.timely_index.to_string()).await?;
                    
                    let mut guilds_configs = tokio::fs::read_dir("guilds_configs").await?;
                    
                    while let Some(file) = guilds_configs.next_entry().await? {
                        let guild: FluxDashGuildConfig = serde_json::from_slice(&tokio::fs::read(&file.path().display().to_string()).await?)?;

                        for (channel, config) in guild.channels {
                            let channel = Channel::new(channel, &api);

                            channel.send_embed_message(vec![Embed::builder().title("New daily level!").build(), make_level_embed(daily.level.clone()).await?]).await?;
                        }
                    };
                };
            } else {
                tokio::fs::write("cache/daily.txt", daily.timely_index.to_string()).await?;
            }
        };
    };

    Ok(())
}