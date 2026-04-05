use gdutils::users::get_user;
use fluxer_rs::{
    api::{common::send_reply},
    command,
};

#[command(UserCommand)]
async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;

    let args = &feedback.args;

    if *args.first().unwrap() == "" {
        send_reply(fluxer_api, &data.channel_id, &data.id, HELP_MESSAGE).await?;
    };

    let user = get_user(*args.first().unwrap()).await;

    const HELP_MESSAGE: &str = "Command usage: `gd!user <user>` where `<user>` is a user ID.";

    let message =
        "Information for user: **".to_owned() + &user.username + "**\n"
    +   "Account ID: " + &user.account_id.to_string() + "\n"
    +   "Player ID: " + &user.player_id.to_string() + "\n"
    +   "Stars: " + &user.stars.to_string() + "\n"
    +   "Moons: " + &user.moons.to_string() + "\n"
    +   "Diamonds: " + &user.diamonds.to_string() + "\n"
    +   "User Coins: " + &user.usercoins.to_string() + "\n"
    +   "Secret Coins: " + &user.secretcoins.to_string() + "\n"
    +   "Demons: " + &user.demons.to_string() + "\n"
    +   "Leaderboard Spot: " + &user.top.to_string() + "\n"
    +   "Creator Points: " + &user.ctpoints.to_string()
    ;

    send_reply(fluxer_api, &data.channel_id, &data.id, &message).await?;

    Ok(())
}