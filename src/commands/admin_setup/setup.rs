use std::{collections::HashMap, fs, thread::sleep};
use color_eyre::eyre::Report;
use fluxer_rs::{api::channels::Channel, command, serde::types::message::EmbedBuilder};
use ::serde::{Deserialize, Serialize};
use serde_json::json;

#[command(SetupCommand)]
pub async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let channel = Channel::new(&data.channel_id, fluxer_api);
    
    let args = &feedback.args;

    let help_embed = vec![
        EmbedBuilder::default()
            .title(String::from("gd!setup"))
            .description("".to_owned()
        +   "**Usage:**\n"
        +   "`gd!setup <channel> <features>`: Set up a channel, where `<channel>` is a channel in this guild "
        +   "and <features> is a comma-separated (without spaces) set of features listed below.")
            .color(0xffff00)
            .build()
            .unwrap(),

        EmbedBuilder::default()
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
            .unwrap()
    ];

    if args.is_empty() {
        let _ = channel.send_embed_message(help_embed).await?;
    } else if args.len() < 2 {
    
    } else {
        let target_channel = args.get(0).unwrap();
        let guild = data.guild_id.as_ref().unwrap();
        let features_buffer: Vec<&str> = args.get(1).unwrap().split(',').collect();
        
        let mut features: Vec<GDServerActivity> = vec![];
        for feature in features_buffer {
            features.push(feature.into())
        };

        let mut new_config: HashMap<String, FluxDashChannelConfig>;

        if tokio::fs::try_exists(format!("{guild}.json")).await.is_err() {
            new_config = HashMap::new();
            new_config.insert(target_channel.to_owned().to_owned(), FluxDashChannelConfig { features });
        } else {
            let existing_config: HashMap<String, FluxDashChannelConfig> = serde_json::from_slice(&tokio::fs::read(format!("{guild}.json")).await.unwrap()).unwrap(); // fix: it's doing this anyway
            new_config = existing_config;
            for feature in features {
                new_config.get_mut("features").unwrap().features.push(feature);
            };
        };

        println!("{:?}", new_config);

        let _ = tokio::fs::write(format!("{guild}.json"), serde_json::ser::to_string(&new_config).unwrap()).await;
    };



    Ok(())
}

pub async fn setup_server_updates_channel(community: &str, channel: &str, activity: GDServerActivity) -> Result<(), Report> {
    if !fs::exists("communities_config.json")? {
        fs::write("communities_config.json", "")?;
    }

    Ok(())
}

pub async fn get_daily() {
    let cooldown = std::time::Duration::from_secs(5);

    sleep(cooldown);
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
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
            "NewRate" => GDServerActivity::NewRate,
            "NewDerate" => GDServerActivity::NewDerate,
            "NewDaily" => GDServerActivity::NewDaily,
            "NewWeekly" => GDServerActivity::NewWeekly,
            "NewEvent" => GDServerActivity::NewEvent,
            "StarChange" => GDServerActivity::StarChange,
            "DemonChange" => GDServerActivity::DemonChange,
            "QualityChange" => GDServerActivity::QualityChange,
            "AREDLUpdate" => GDServerActivity::AREDLUpdate,
            _ => GDServerActivity::Unknown
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluxDashChannelConfig {
    features: Vec<GDServerActivity>
}
// TODO: REMEMBER VERY IMPORTANT add a check to see if a community has already added a feature to a channel, if so prevent it from being added to more channels.
// no 2 channels that both have daily level updates. this will reduce server load and abuse potential