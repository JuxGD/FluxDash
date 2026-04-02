use gdutils::levels::get_daily;

use fluxer_rs::{
    api::{common::send_reply},
    command,
};

#[command(DailyCommand)]
async fn execute(api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;

    let response = get_daily();

    send_reply(api, &data.channel_id, &data.id, response.await.as_str()).await?;
    Ok(())
}