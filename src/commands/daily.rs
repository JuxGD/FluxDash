use crate::utils::levels::*;

use gdutils::levels::get_daily;

use time_hms::TimeHms;

use fluxer_rs::{
    api::{common::send_reply},
    command,
};

#[command(DailyCommand)]
async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;

    let daily = get_daily().await;
    let level_info = daily.info;
    let time_left = daily.time_left as u64;

    let message =
        "Info for daily level: **".to_owned() + &level_info.name + "**\n"
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
    +   "Daily #" + &daily.timely_index.to_string() + "\n"
    +   "Time Left: " + &TimeHms::new(time_left).to_string()
    ;

    send_reply(fluxer_api, &data.channel_id, &data.id, &message).await?;
    Ok(())
}