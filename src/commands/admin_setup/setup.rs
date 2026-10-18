use std::{collections::HashMap, hash::Hash, sync::Arc};
use color_eyre::eyre::{self, eyre};
use fluxer_rs::{api::{FluxerApiHandler, channels::Channel}, command, serde::types::message::Embed};
use gdutils::gd::{levels::{build_level, download_level_by_id, get_timely_info, level_object_to_hashmap}, utils::{Level, TimelyInfo, TimelyType}};
use serde::{Deserialize, Serialize};
use itertools::Itertools;

use crate::utils::levels::make_timely_level_embed;
#[command(SetupCommand)]
pub async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let role = data.member.as_ref().unwrap().roles.as_ref().unwrap().first().unwrap();

    let channel = Channel::new(&data.channel_id, fluxer_api);
    
    let args = &feedback.args;

    let help_embed = vec![
        Embed::builder()
            .title(String::from("gd!setup"))
            .description("".to_owned()
        +   "**Usage:**\n"
        +   "`gd!setup <channel> <features>`: Set up a channel, where `<channel>` is a channel in this guild"
        +   "and <features> is a comma-separated (without spaces) set of features listed below.")
            .color(0xffff00)
            .build(),

        Embed::builder()
            .title(String::from("Features"))
            .description("".to_owned()
        +   "`NewRates`\n"
        +   "`NewDerate`\n"
        +   "`NewDaily`\n"
        +   "`NewWeekly`\n"
        +   "`NewEvent`\n"
        +   "`StarChange`\n"
        +   "`DemonChange`\n"
        +   "`QualityChange`\n"
        +   "`AREDLUpdate`\n")
            .color(0xffff00)
            .build()
    ];

    if args.len() == 2 {
        let target_channel = args.first().unwrap();
        let target_channel = target_channel.replace(['<', '#', '>'], "");
        let guild = data.guild_id.as_ref().unwrap();
        let features_buffer: Vec<&str> = args.get(1).unwrap().split(',').unique().collect();
        
        let mut features: Vec<GDServerActivity> = vec![];
        for feature in features_buffer {
            features.push(feature.into());
        };

        let mut new_config = FluxDashGuildConfig { global_features: vec![], channels: HashMap::new() };

        if tokio::fs::try_exists(format!("guilds_configs/{guild}.json")).await.unwrap() {
            let existing_config: FluxDashGuildConfig = serde_json::from_slice(&tokio::fs::read(format!("guilds_configs/{guild}.json")).await.unwrap())?;

            new_config = existing_config.clone();

            for feature in features {
                if !existing_config.global_features.contains(&feature) {
                    new_config.channels.get_mut(&target_channel).unwrap().features.push(feature);
                    new_config.global_features.push(feature);
                };
            };
        } else {
            new_config.channels.insert(target_channel.to_owned().to_owned(), FluxDashChannelConfig { features: features.clone() });
            new_config.global_features = features;
        };

        tokio::fs::write(format!("guilds_configs/{guild}.json"), serde_json::ser::to_string_pretty(&new_config)?).await.unwrap();
    } else {
        channel.send_embed_message(help_embed).await?;
    };

    Ok(())
}

pub async fn send_gd_server_update(api: Arc<FluxerApiHandler>) -> Result<(), eyre::Report> {
    if tokio::fs::try_exists("guilds_configs").await? {
        loop {
            let daily = get_timely_info(TimelyType::Daily).await;
			let mut new_daily = false;

            let weekly = get_timely_info(TimelyType::Weekly).await;
			let mut new_weekly = false;

            let event = get_timely_info(TimelyType::Event).await;
			let mut new_event = false;

			let mut new_rate = false;

			let mut new_derate = false;

			let mut new_star_change = false;

			let mut new_quality_change = false;

			let mut new_demon_change = false;

			let mut new_aredl_change = false;

            if daily.is_ok() {
                if tokio::fs::try_exists("cache/daily.txt").await? {
                    let daily = daily.as_ref().unwrap();
                    if tokio::fs::read_to_string("cache/daily.txt").await? != daily.timely_index.to_string() {
                        tokio::fs::write("cache/daily.txt", daily.timely_index.to_string()).await?;

                        new_daily = true;
                    };
                };
            } else {

            };

            if weekly.is_ok() {
                if tokio::fs::try_exists("cache/weekly.txt").await? {
                    let weekly = weekly.as_ref().unwrap();
                    if tokio::fs::read_to_string("cache/weekly.txt").await? != weekly.timely_index.to_string() {
                        tokio::fs::write("cache/weekly.txt", weekly.timely_index.to_string()).await?;

                        new_weekly = true;
                    };
                };
            } else {

            };

            if event.is_ok() {
                if tokio::fs::try_exists("cache/event.txt").await? {
                    let event = event.as_ref().unwrap();
                    if tokio::fs::read_to_string("cache/event.txt").await? != event.timely_index.to_string() {
                        tokio::fs::write("cache/event.txt", event.timely_index.to_string()).await?;

                        new_event = true;
                    };
                };
            } else {

            };

			let mut guilds_configs = tokio::fs::read_dir("guilds_configs").await?;
            
            if new_daily { send_timely_update(daily.unwrap(), TimelyType::Daily, &mut guilds_configs, &api).await?; };

            if new_weekly { send_timely_update(weekly.unwrap(), TimelyType::Weekly, &mut guilds_configs, &api).await?; };

            if new_event { send_timely_update(event.unwrap(), TimelyType::Event, &mut guilds_configs, &api).await?; };

            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        };
    };

    Ok(())
}

async fn send_timely_update(timely_info: TimelyInfo, timely_type: TimelyType, guilds_configs: &mut tokio::fs::ReadDir, api: &Arc<FluxerApiHandler>) -> Result<(), eyre::Report> {
    let level: Result<Level, eyre::Report> = match timely_type {
        TimelyType::Daily => Ok(build_level(&level_object_to_hashmap(&download_level_by_id("-1").await?)?)?),
        TimelyType::Weekly => Ok(build_level(&level_object_to_hashmap(&download_level_by_id("-2").await?)?)?),
        TimelyType::Event => Ok(build_level(&level_object_to_hashmap(&download_level_by_id("-3").await?)?)?),
        TimelyType::Unknown => Err(eyre!("Invalid TimelyType")),
    };

    let timely_type_str: Result<&str, color_eyre::eyre::Report> = match timely_type {
        TimelyType::Daily => Ok("Daily"),
        TimelyType::Weekly => Ok("Weekly"),
        TimelyType::Event => Ok("Event"),
        TimelyType::Unknown => Err(eyre!("Invalid TimelyType")),
    };

    let timely_type_str = timely_type_str?;

    let level = level?;
    let embed = make_timely_level_embed(timely_type, level, timely_info).await?;
    
    while let Some(file) = guilds_configs.next_entry().await? {
        let guild: FluxDashGuildConfig = serde_json::from_slice(&tokio::fs::read(&file.path().display().to_string()).await?)?;

        for (channel, config) in guild.channels {
            if config.features.contains(&GDServerActivity::NewEvent) {
                let channel = Channel::new(channel, &api);

                channel.send_embed_message(vec![Embed::builder().title(format!("New {timely_type_str} level!")).build(), embed.clone()]).await?;
            };
        };
    };

    Ok(())
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GDServerActivity {
    NewRate,
    NewDerate,
    NewDaily,
    NewWeekly,
    NewEvent,
    StarChange,
    DemonChange,
    QualityChange,
    AREDLUpdate,
    Unknown
}

impl From<&str> for GDServerActivity {
    fn from(value: &str) -> Self {
        match value {
            "NewRate" => Self::NewRate,
            "NewDerate" => Self::NewDerate,
            "NewDaily" => Self::NewDaily,
            "NewWeekly" => Self::NewWeekly,
            "NewEvent" => Self::NewEvent,
            "StarChange" => Self::StarChange,
            "DemonChange" => Self::DemonChange,
            "QualityChange" => Self::QualityChange,
            "AREDLUpdate" => Self::AREDLUpdate,
            _ => Self::Unknown
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluxDashGuildConfig {
    pub global_features: Vec<GDServerActivity>,
    pub channels: HashMap<String, FluxDashChannelConfig>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluxDashChannelConfig {
    pub features: Vec<GDServerActivity>
}
// TODO: REMEMBER VERY IMPORTANT add a check to see if a community has already added a feature to a channel, if so prevent it from being added to more channels.
// no 2 channels that both have daily level updates. this will reduce server load and abuse potential