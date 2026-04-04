use gdutils::levels::get_weekly;
use fluxer_rs::{
    api::{common::send_reply},
    command,
};

#[command(WeeklyCommand)]
async fn execute(api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;

    let weekly = get_weekly().await;

    send_reply(api, &data.channel_id, &data.id, format!("{:?}", weekly).as_str()).await?;
    Ok(())
}