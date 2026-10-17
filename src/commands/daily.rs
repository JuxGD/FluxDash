use crate::utils::levels::make_level_embed;

use gdutils::gd::{levels::get_timely_level, utils::TimelyType};


use fluxer_rs::{
    api::channels::Channel, command, serde::types::message::Embed,
};
use time_hms::TimeHms;

#[command(DailyCommand)]
async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let channel = Channel::new(&data.channel_id, fluxer_api);

    let daily = get_timely_level(TimelyType::Daily).await.unwrap();
    let level_info = daily.level;
    let time_left = daily.time_left as u64;

    let embed = vec![
        make_level_embed(level_info).await.unwrap(),
        Embed::builder()
            .description(String::from("\n")
            +   "Daily #" + &daily.timely_index.to_string() + "\n"
            +   "Time Left" + &TimeHms::new(time_left).to_string() + "\n"
        )
        .build()
    ];
    
    channel.send_embed_message(embed).await?;
    
    Ok(())
}