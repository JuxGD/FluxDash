use crate::utils::levels::make_level_embed;

use gdutils::gd::{levels::get_timely_level, utils::TimelyType};

use fluxer_rs::{
    api::channels::Channel, command, serde::types::message::Embed
};

#[command(EventCommand)]
async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let channel = Channel::new(&data.channel_id, fluxer_api);

    let event = get_timely_level(TimelyType::Event).await.unwrap();
    let level_info = event.level;

    let embed = vec![
        make_level_embed(level_info).await.unwrap(),
        Embed::builder()
            .description(String::from("\n")
            +   "Event #" + &event.timely_index.to_string() + "\n"
        )
        .build()
    ];    

    channel.send_embed_message(embed).await?;
    
    Ok(())
}