use fluxer_rs::{
    api::{common::send_reply},
    command,
};

#[command(HelpCommand)]
async fn execute(api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;

    let message =
        "### Available Commands:\n".to_owned()
    +  &"- `gd!level <levelID>` - Returns requested level info\n"
    +   "- `gd!user <userID>` - Returns requested user stats\n"
    +   "- `gd!daily` - Returns daily level\n"
    +   "- `gd!weekly` - Returns weekly level\n"
    +   "- `gd!event` - Returns event level\n"
    +   "\n"
    ;

    send_reply(api, &data.channel_id, &data.id, &message).await?;
    Ok(())
}