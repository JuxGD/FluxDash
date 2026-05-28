use gdutils::gd::users::get_user_info;
use fluxer_rs::{
    api::channels::Channel,
    command,
    serde::types::message::EmbedBuilder
};

#[command(UserCommand)]
async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let channel = Channel::new(&data.channel_id, fluxer_api);

    let args = &feedback.args;

    if args.first().unwrap().is_empty() {
        let _ = channel.send_embed_message(vec![
            EmbedBuilder::default()
                .description(String::from("**Command usage**: `gd!user <user>` where `<user>` is an account ID or username."))
                .color(0xff0000)
                .build()
                .unwrap()
        ]).await;
    } else {
        let user_info = get_user_info(args.first().unwrap()).await;
        
        let embed = vec![if user_info.is_valid {
            EmbedBuilder::default()
                .title(String::from("User Info"))
                .description("".to_owned()
                +   "**" + &user_info.username + "**\n"
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
                )
                .color(0x0055ff)
                .build()
                .unwrap()
        } else {
            EmbedBuilder::default()
                .description(String::from("**User invalid or not found. Make sure it's spelled correctly or that it's the right accountID**"))
                .color(0xff0000)
                .build()
                .unwrap()
        }];

        let _ = channel.send_embed_message(embed).await;
    }

    Ok(())
}