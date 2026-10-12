use gdutils::gd::{users::get_user, utils::GDQuery};
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
        let user_info = get_user(GDQuery::new(args.first().unwrap())).await;
        
        let embed = vec![if user_info.is_ok() {
            let user_info = user_info.unwrap();

            EmbedBuilder::default()
                .title(String::from("User Info"))
                .description("".to_owned()
                +   "**" + &user_info.username + "**\n"
                +   "Account ID: " + &user_info.account_id.to_string() + "\n"
                +   "User ID: " + &user_info.user_id.to_string() + "\n"
                +   "Stars: " + &user_info.stars.to_string() + "\n"
                +   "Moons: " + &user_info.moons.to_string() + "\n"
                +   "Diamonds: " + &user_info.diamonds.to_string() + "\n"
                +   "User Coins: " + &user_info.user_coins.to_string() + "\n"
                +   "Secret Coins: " + &user_info.secret_coins.to_string() + "\n"
                +   "Demons: " + &user_info.demon_count.to_string() + "\n"
                +   "Leaderboard Spot: " + &user_info.global_rank.unwrap_or_default().to_string() + "\n"
                +   "Creator Points: " + &user_info.creator_points.to_string()
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