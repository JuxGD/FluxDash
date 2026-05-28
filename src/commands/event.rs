use crate::utils::levels::*;

use gdutils::gd::levels::get_event;

use time_hms::TimeHms;

use fluxer_rs::{
    api::channels::Channel,
    command,
    serde::types::message::EmbedBuilder
};

#[command(EventCommand)]
async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let channel = Channel::new(&data.channel_id, fluxer_api);

    let event = get_event().await;
    let level_info = event.info;
    let time_left = event.time_left as u64;

    let embed = vec![EmbedBuilder::default()
        .title(String::from("Event Level"))
        .description("".to_owned()
        +   "**" + &level_info.name + "**\n"
        +   "Info for event level: **" + &level_info.name + "**\n"
        +   "ID: " + &level_info.id.to_string() + "\n"
        +   "Author: " + &level_info.author + "\n"
        +   "Difficulty: " + &level_info.rating + "\n"
        +   "Quality: " + &level_info.quality.to_string() + &feature_score(level_info.feature_score) + "\n"
        +   "Stars: " + &level_info.stars.to_string() + "\n"
        +   "Coins: " + &level_info.coins.to_string() + &verified_coins(level_info.verified_coins) + "\n"
        +   "Likes: " + &level_info.likes.to_string() + "\n"
        +   "Downloads: " + &level_info.downloads.to_string() + "\n"
        +   "Song ID: " + &level_info.song_id.to_string() + "\n"
        +   "\n"
        +   "Event #" + &event.timely_index.to_string() + "\n"
        +   "Time Left: " + &TimeHms::new(time_left).to_string()
        )
        .color(0x0055ff)
        .build()
        .unwrap()
    ];
    
    let _ = channel.send_embed_message(embed).await;
    
    Ok(())
}