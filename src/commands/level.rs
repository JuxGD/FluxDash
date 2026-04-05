use crate::utils::levels::*;

use gdutils::levels::*;

use fluxer_rs::{
    api::common::send_reply,
    command
};

#[command(LevelCommand)]
async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;

    let args = &feedback.args;

    let help_message: String =
        "**Empty argument detected**: Returning info for most liked level!\n".to_owned()
    +  &"To return info for a specific level, do:\n"
    +  &"`gd!level <level>` where `<level>` is a level ID or name"
    ;

    if *args.first().unwrap() == "" {
        send_reply(fluxer_api, &data.channel_id, &data.id, &help_message).await?;
    };

    let level_info = get_level_info(*args.first().unwrap()).await;

    let message = if level_info.is_valid == true {
        "Info for level: **".to_owned() + &level_info.name + "**\n"
    +   "ID: " + &level_info.id.to_string() + "\n"
    +   "Author: " + &level_info.author + "\n"
    +   "Difficulty: " + &level_info.rating + "\n"
    +   "Quality: " + &level_info.quality.to_string() + &feature_score(level_info.feature_score) + "\n"
    +   "Stars: " + &level_info.stars.to_string() + "\n"
    +   "Coins: " + &level_info.coins.to_string() + &verified_coins(level_info.verified_coins) + "\n"
    +   "Likes: " + &level_info.likes.to_string() + "\n"
    +   "Downloads: " + &level_info.downloads.to_string() + "\n"
    +   "Song ID: " + &level_info.song_id.to_string() + "\n"
    } else {
        String::from("Level invalid or not found. Make sure it's spelled correctly or that it's the right levelID")
    };

    send_reply(fluxer_api, &data.channel_id, &data.id, &message).await?;
    
    Ok(())
}