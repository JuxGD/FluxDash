use fluxer_rs::{
    api::{common::send_reply},
    command,
};

#[command(UserCommand)]
async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;

    let args = &feedback.args;

    let user: &str;

    const HELP_MESSAGE: &str = "Command usage: `gd!user <user>` where `<user>` is a user ID or username.";

    if *args.first().unwrap() == "" {
        send_reply(fluxer_api, &data.channel_id, &data.id, HELP_MESSAGE).await?;
    } else {
        user = args.first().unwrap();
        send_reply(fluxer_api, &data.channel_id, &data.id, "still not implemented :p\n - Jux, the developer").await?;
    }
    
    Ok(())
}