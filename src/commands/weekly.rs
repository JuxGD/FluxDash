use crate::utils::levels::make_level_embed;

use gdutils::gd::{levels::get_timely_level, utils::TimelyType};

use time_hms::TimeHms;

use fluxer_rs::{
    api::channels::Channel, command, serde::types::message::Embed
};

#[command(WeeklyCommand)]
async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let channel = Channel::new(&data.channel_id, fluxer_api);

    let weekly = get_timely_level(TimelyType::Weekly).await.unwrap();
    let level_info = weekly.level;
    let time_left = weekly.time_left as u64;

    let embed = vec![
        make_level_embed(level_info).await.unwrap(),
        Embed::builder()
            .description(String::from("\n")
            +   "Weekly #" + &weekly.timely_index.to_string() + "\n"
            +   "Time Left" + &TimeHms::new(time_left).to_string() + "\n"
        )
        .build()
    ];
    
    channel.send_embed_message(embed).await?;
    
    Ok(())
}