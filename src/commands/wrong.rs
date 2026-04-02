use fluxer_rs::{
    api::{common::send_reply},
    command,
};

#[command(WrongCommand)]
async fn execute(api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;

    send_reply(api, &data.channel_id, &data.id, format!(
        "### Available Commands:\n{}{}{}{}",
        "`gd!daily` - Returns daily level\n",
        "`gd!weekly` - Returns weekly level\n",
        "`gd!event` - Returns event level\n",
        "`gd!user` - Returns requested user stats\n").as_str()).await?;
    Ok(())
}