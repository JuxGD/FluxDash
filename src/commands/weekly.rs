use gdutils::levels::get_weekly;
use fluxer_rs::{
    api::{common::send_reply},
    command,
};

#[command(WeeklyCommand)]
async fn execute(api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;

    let response = get_weekly();

    send_reply(api, &data.channel_id, &data.id, response.await.as_str()).await?;
    Ok(())
}