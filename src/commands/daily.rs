use gdutils::levels::get_daily;

use fluxer_rs::{
    api::{common::send_reply},
    command,
};

#[command(DailyCommand)]
async fn execute(api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;

    let daily = get_daily().await;

    send_reply(api, &data.channel_id, &data.id, format!("{:?}", daily).as_str()).await?;
    Ok(())
}