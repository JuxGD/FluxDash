use fluxer_rs::{
    api::{common::send_reply},
    command,
};

#[command(PingCommand)]
async fn execute(api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;

    send_reply(api, &data.channel_id, &data.id, "Pong!").await?;
    Ok(())
}