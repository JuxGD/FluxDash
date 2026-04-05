use gdutils::users::get_user_info;
use fluxer_rs::{
    api::{common::send_reply},
    command,
};

#[command(UserCommand)]
async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;

    let args = &feedback.args;

    let help_message: &str = "Command usage: `gd!user <user>` where `<user>` is an account ID or username.";

    if *args.first().unwrap() == "" {
        send_reply(fluxer_api, &data.channel_id, &data.id, help_message).await?;
    } else {
        let user_info = get_user_info(*args.first().unwrap()).await;
        
        let message = if user_info.is_valid == true {
            "Info for user: **".to_owned() + &user_info.username + "**\n"
        +   "Account ID: " + &user_info.account_id.to_string() + "\n"
        +   "Player ID: " + &user_info.player_id.to_string() + "\n"
        +   "Stars: " + &user_info.stars.to_string() + "\n"
        +   "Moons: " + &user_info.moons.to_string() + "\n"
        +   "Diamonds: " + &user_info.diamonds.to_string() + "\n"
        +   "User Coins: " + &user_info.usercoins.to_string() + "\n"
        +   "Secret Coins: " + &user_info.secretcoins.to_string() + "\n"
        +   "Demons: " + &user_info.demons.to_string() + "\n"
        +   "Leaderboard Spot: " + &user_info.top.to_string() + "\n"
        +   "Creator Points: " + &user_info.ctpoints.to_string()
        } else {
            String::from("User invalid or not found. Make sure it's spelled correctly or that it's the right accountID")
        };

        send_reply(fluxer_api, &data.channel_id, &data.id, &message).await?;
    }





    Ok(())
}