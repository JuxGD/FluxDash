use std::{collections::HashMap, hash::Hash, thread::sleep};
use fluxer_rs::{api::{FluxerApiHandler, channels::Channel}, command, serde::types::message::{self, Embed}};
use gdutils::gd::{levels::get_timely_level, utils::TimelyType};
use serde::{Deserialize, Serialize};
use itertools::Itertools;
#[command(SetupCommand)]
pub async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let channel = Channel::new(&data.channel_id, fluxer_api);
    
    let args = &feedback.args;

    let help_embed = vec![
        Embed::builder()
            .title(String::from("gd!setup"))
            .description("".to_owned()
        +   "**Usage:**\n"
        +   "`gd!setup <channel> <features>`: Set up a channel, where `<channel>` is a channel in this guild "
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

pub async fn send_gd_server_update(guild: FluxDashGuildConfig, feature: GDServerActivity, api: &FluxerApiHandler) -> Result<(), color_eyre::eyre::Report> {
    for (channel, channel_config) in guild.channels {
        let channel = Channel::new(channel, api);

        println!("{}", channel.channel_id);

        let action = match feature {
            GDServerActivity::NewDaily => {
                get_timely_level(TimelyType::Daily).await?
            },
            _ => return Ok(())
        };

        channel.send_message("test message please ignore").await?;
    }

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