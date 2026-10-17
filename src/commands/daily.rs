use crate::utils::levels::{feature_score, verified_coins};

use gdutils::gd::{levels::{calculate_difficulty, calculate_quality, get_timely_level, search_levels}, utils::{GDQuery, TimelyType}};

use time_hms::TimeHms;

use fluxer_rs::{
    api::channels::Channel, command, serde::types::message::Embed
};

#[command(DailyCommand)]
async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let channel = Channel::new(&data.channel_id, fluxer_api);

    let daily = get_timely_level(TimelyType::Daily).await.unwrap();
    let level_info = daily.level;
    let time_left = daily.time_left as u64;

    let author = search_levels(GDQuery::new(&level_info.level_id.to_string())).await.unwrap().split('#').nth(1).unwrap().split(':').nth(1).unwrap().to_owned();
    let difficulty = calculate_difficulty(level_info.difficulty_numerator.into(), level_info.difficulty_denominator.into(), level_info.auto, level_info.demon).unwrap();
    let quality = calculate_quality(level_info.feature_score, level_info.epic.into()).unwrap();

    let embed = vec![Embed::builder()
        .title(String::from("Daily Level"))
        .description(String::new()
        +   "**" + &level_info.level_name + "**\n"
        +   "ID: " + &level_info.level_id.to_string() + "\n"
        +   "Author: " + &author + "\n"
        +   "Difficulty: " + &difficulty + "\n"
        +   "Quality: " + &quality + &feature_score(level_info.feature_score) + "\n"
        +   "Stars: " + &level_info.stars.to_string() + "\n"
        +   "Coins: " + &level_info.coins.to_string() + &verified_coins(level_info.verified_coins) + "\n"
        +   "Likes: " + &level_info.likes.to_string() + "\n"
        +   "Downloads: " + &level_info.downloads.to_string() + "\n"
        +   "Song ID: " + &level_info.custom_song_id.to_string() + "\n"
        +   "\n"
        +   "Daily #" + &daily.timely_index.to_string() + "\n"
        +   "Time Left: " + &TimeHms::new(time_left).to_string()
        )
        .color(0x0055ff)
        .build()
    ];
    
    channel.send_embed_message(embed).await?;
    
    Ok(())
}