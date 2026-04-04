use fluxer_rs::{
    api::{common::send_reply},
    command,
};

#[command(WrongCommand)]
async fn execute(api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;

    let message =
        "### Available Commands:\n".to_owned()
    +  &"- `gd!level <levelID>` - Returns requested level info\n"
    +   "  - **PLEASE MAKE SURE THE ID IS VALID**, invalid IDs crash the bot\n"
    +   "- `gd!daily` - Returns daily level\n"
    +   "- `gd!weekly` - Returns weekly level\n"
    +   "- `gd!event` - Returns event level\n"
    +   "- `gd!user <userID>` - Returns requested user stats\n"
    +   "\n"
    +   "**DO NOT USE COMMANDS THAT DON'T EXIST, this is still not handled and will crash the bot**"
    ;

    send_reply(api, &data.channel_id, &data.id, &message).await?;
    Ok(())
}