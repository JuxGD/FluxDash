use crate::utils::levels::make_timely_level_embed;

use gdutils::gd::{levels::{get_timely_info, get_timely_level}, utils::TimelyType};


use fluxer_rs::{
    api::channels::Channel, command, serde::types::message::Embed,
};

#[command(DailyCommand)]
async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let channel = Channel::new(&data.channel_id, fluxer_api);

    match get_timely_level(TimelyType::Daily).await {
        Ok(daily) => {
            let level_info = daily.level;

            let embed = vec![
                make_timely_level_embed(TimelyType::Daily, level_info, get_timely_info(TimelyType::Daily).await.unwrap()).await.unwrap()
            ];
            
            channel.send_embed_message(embed).await?;
        },
        Err(_) => {
            channel.send_embed_message(vec![Embed::builder()
                .description("
                    **Something went wrong while getting daily level info.**

                    If there is an active daily level, please report this to @JuxGD#7774
                ")
                .color(0xff0000).build()
            ]).await?;
        }
    };

    Ok(())
}