use crate::utils::levels::make_level_embed;

use gdutils::gd::{levels::get_timely_level, utils::TimelyType};

use fluxer_rs::{
    api::channels::Channel, command, serde::types::message::Embed
};

#[command(EventCommand)]
async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let channel = Channel::new(&data.channel_id, fluxer_api);

    match get_timely_level(TimelyType::Daily).await {
        Ok(event) => {
            let level_info = event.level;
            let time_left = event.info.time_left as u64;

            let embed = vec![
                make_level_embed(level_info).await.unwrap(),
                Embed::builder()
                    .description(String::from("\n")
                    +   "Event #" + &event.info.timely_index.to_string() + "\n"
                )
                .build()
            ];
            
            channel.send_embed_message(embed).await?;
        },
        Err(_) => {
            channel.send_embed_message(vec![Embed::builder()
                .description("
                    **Something went wrong, while getting event level info.**
                    
                    If there is an active event level, please report this to @JuxGD#7774
                ")
                .color(0xff0000).build()
            ]).await?;
        }
    };

    Ok(())
}