use crate::utils::levels::{feature_score, verified_coins};

use gdutils::gd::{levels::{download_level_by_id, download_level_by_name, build_level, level_object_to_hashmap, search_levels, calculate_difficulty, calculate_quality}, utils::GDQuery};

use fluxer_rs::{
    api::channels::Channel, command, serde::types::message::Embed
};

#[command(LevelCommand)]
async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let channel = Channel::new(&data.channel_id, fluxer_api);

    let args = &feedback.args;

    let help_embed = vec![
        Embed::builder()
            .title(String::from("gd!level"))
            .description(String::new()
        +   "**Usage:**\n"
        +   "`gd!level <levelID/levelname>`. If the level name has a space, use double quotes (e.g. `gd!level \"Blade of Justice\"`)"
        )
        .color(0xffff00)
        .build()
    ];

    if args.is_empty() {
        let _ = channel.send_embed_message(help_embed).await?;
    } else {
        let query = args[0];

        let level = if query.parse::<i32>().is_ok() { download_level_by_id(query).await.unwrap() } else { download_level_by_name(query).await.unwrap() };
        
        let level_info = build_level(&level_object_to_hashmap(level).unwrap());

        let embed: Vec<Embed> = if let Ok(level_info) = level_info {

            let author = search_levels(GDQuery::new(&level_info.level_id.to_string())).await.unwrap().split('#').nth(1).unwrap().split(':').nth(1).unwrap().to_owned();
            let difficulty = calculate_difficulty(level_info.difficulty_numerator.into(), level_info.difficulty_denominator.into(), level_info.auto, level_info.demon).unwrap();
            let quality = calculate_quality(level_info.feature_score, level_info.epic.into()).unwrap();

            vec![
                Embed::builder()
                    .title(String::from("Level Info"))
                    .description(String::new()
                    +   "Info for level: **" + &level_info.level_name + "**\n"
                    +   "ID: " + &level_info.level_id.to_string() + "\n"
                    +   "Author: " + &author + "\n"
                    +   "Difficulty: " + &difficulty + "\n"
                    +   "Quality: " + &quality + &feature_score(level_info.feature_score) + "\n"
                    +   "Stars: " + &level_info.stars.to_string() + "\n"
                    +   "Coins: " + &level_info.coins.to_string() + &verified_coins(level_info.verified_coins) + "\n"
                    +   "Likes: " + &level_info.likes.to_string() + "\n"
                    +   "Downloads: " + &level_info.downloads.to_string() + "\n"
                    +   "Song ID: " + &level_info.custom_song_id.to_string() + "\n"
                    )
                    .color(0x0055ff)
                    .build()
            ]
        } else {
            vec![Embed::builder()
                .description(String::from("**Level invalid or not found. Make sure it's spelled correctly or that it's the right levelID.**"))
                .color(0xff0000)
                .build()
            ]
        };

        channel.send_embed_message(embed).await?;
    }

    Ok(())
}