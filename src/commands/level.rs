use crate::utils::levels::*;

use gdutils::gd::levels::*;

use fluxer_rs::{
    api::channels::Channel,
    command,
    serde::types::message::EmbedBuilder
};

#[command(LevelCommand)]
async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let channel = Channel::new(&data.channel_id, fluxer_api);

    let args = &feedback.args;

    let help_embed = vec![
        EmbedBuilder::default()
            .title(String::from("gd!level"))
            .description("".to_owned()
        +   "**Usage:*"
        +   "`gd!level <levelID/levelname>`. If the level has a space, use double quotes (e.g. `gd!level \"Blade of Justice\"`)"
        )
        .build()
        .unwrap()
    ];

    if args.first().unwrap().is_empty() {
        let _ = channel.send_embed_message(help_embed).await;
    } else {

        let level_info = get_level_info(args.first().unwrap()).await;

        let embed = vec![if level_info.is_valid {
            EmbedBuilder::default()
                .title(String::from("Level Info"))
                .description("".to_owned()
                +   "Info for level: **" + &level_info.name + "**\n"
                +   "ID: " + &level_info.id.to_string() + "\n"
                +   "Author: " + &level_info.author + "\n"
                +   "Difficulty: " + &level_info.rating + "\n"
                +   "Quality: " + &level_info.quality.to_string() + &feature_score(level_info.feature_score) + "\n"
                +   "Stars: " + &level_info.stars.to_string() + "\n"
                +   "Coins: " + &level_info.coins.to_string() + &verified_coins(level_info.verified_coins) + "\n"
                +   "Likes: " + &level_info.likes.to_string() + "\n"
                +   "Downloads: " + &level_info.downloads.to_string() + "\n"
                +   "Song ID: " + &level_info.song_id.to_string() + "\n"
            )
            .color(0x0055ff)
            .build()
            .unwrap()
        } else {
            EmbedBuilder::default()
                .description(String::from("**Level invalid or not found. Make sure it's spelled correctly or that it's the right levelID.**"))
                .color(0xff0000)
                .build()
                .unwrap()
        }];

        let _ = channel.send_embed_message(embed).await;
    }

    Ok(())
}