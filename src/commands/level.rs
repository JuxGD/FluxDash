use gdutils::levels::get_level;
use fluxer_rs::{
    api::{common::send_reply},
    command,
};

#[command(LevelCommand)]
async fn execute(api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;

    let args = &feedback.args;

    let level = get_level(String::from(*args.first().unwrap())).await;

    send_reply(api, &data.channel_id, &data.id, &format!("**{}**", level.name)).await?;
    Ok(())
}