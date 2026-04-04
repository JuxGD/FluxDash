use gdutils::levels::get_level;
use fluxer_rs::{
    api::common::send_reply,
    command
};

#[command(LevelCommand)]
async fn execute(api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;

    let args = &feedback.args;

    let level = get_level(String::from(*args.first().unwrap())).await;

    let message =
        "Information for level: **".to_owned() + &level.name + "**\n"
    +   "ID: " + &level.id.to_string() + "\n"
    +   "Song ID: " + &level.song + "\n"
    +   "Difficulty: " + &level.rating + "\n"
    +   "Stars: " + &level.stars.to_string() + "\n"
    +   "Likes: " + &level.likes.to_string() + "\n"
    +   "Downloads: " + &level.downloads.to_string() + "\n"
    ;

    send_reply(api, &data.channel_id, &data.id, &message).await?;
    Ok(())
}