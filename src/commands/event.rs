use crate::utils::levels::*;

use gdutils::gd::{levels::{calculate_difficulty, calculate_quality, get_timely_level, search_levels}, utils::{GDQuery, TimelyType}};

use fluxer_rs::{
    api::channels::Channel,
    command,
    serde::types::message::EmbedBuilder
};

#[command(EventCommand)]
async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let channel = Channel::new(&data.channel_id, fluxer_api);

    let event = get_timely_level(TimelyType::Event).await.unwrap();
    let level_info = event.level;
    
    let author = search_levels(GDQuery::new(&level_info.level_id.to_string())).await.unwrap().split('#').collect::<Vec<_>>()[1].split(':').collect::<Vec<_>>()[1].to_owned();
    let difficulty = calculate_difficulty(level_info.difficulty_numerator.into(), level_info.difficulty_denominator.into(), level_info.auto, level_info.demon).unwrap();
    let quality = calculate_quality(level_info.feature_score, level_info.epic.into()).unwrap();

    let embed = vec![EmbedBuilder::default()
        .title(String::from("Event Level"))
        .description("".to_owned()
        +   "**" + &level_info.level_name + "**\n"
        +   "ID: " + &level_info.level_id.to_string() + "\n"
        +   "Author: " + &author + "\n"
        +   "Difficulty: " + &difficulty + "\n"
        +   "Quality: " + &quality + &feature_score(level_info.feature_score) + "\n"
        +   "Stars: " + &level_info.stars.to_string() + "\n"
        +   "Coins: " + &level_info.coins.to_string() + &verified_coins(level_info.verified_coins) + "\n"
        +   "Likes: " + &level_info.likes.to_string() + "\n"
        +   "Downloads: " + &level_info.downloads.to_string() + "\n"
        +   "Song ID: " + &level_info.song_ids.unwrap()[0].to_string() + "\n"
        +   "\n"
        +   "Event #" + &event.timely_index.to_string() + "\n"
        )
        .color(0x0055ff)
        .build()
        .unwrap()
    ];
    
    let _ = channel.send_embed_message(embed).await;
    
    Ok(())
}