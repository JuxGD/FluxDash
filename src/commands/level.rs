use crate::utils::levels::make_level_embed;

use gdutils::gd::levels::{download_level_by_id, download_level_by_name, build_level, level_object_to_hashmap};

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
            vec![make_level_embed(level_info).await.unwrap()]
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