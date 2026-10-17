use std::env;

use gdutils::{aredl::classic::{get_aredl_gd_info, get_aredl_level_info_by_position}, gd::{levels::{calculate_difficulty, calculate_quality, search_levels}, utils::GDQuery}};

use fluxer_rs::{
    api::channels::Channel, command, serde::types::message::Embed
};

use crate::utils::levels::{feature_score, verified_coins};

#[command(AREDLCommand)]
async fn execute(fluxer_api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let aredl_api_token = env::var("AREDL_API_TOKEN").unwrap();

    let args = &feedback.args;
    let data = feedback.data;

    let channel = Channel::new(&data.channel_id, fluxer_api);

    let base_help = Embed::builder()
        .title(String::from("Usage"))
        .description("".to_owned()
        +   "`gd!aredl [listinfo/levelinfo] <number>`: where `number` is an AREDL placement.\n"
        +   "- `listinfo` will return info shown on AREDL\n"
        +   "- `levelinfo` will return GD level info")
        .color(0xffff00)
        .build();

    let listinfo_help = Embed::builder()
        .title(String::from("Usage"))
        .description(String::from("`gd!aredl listinfo <number>` where `number` is an AREDL placement."))
        .color(0xffff00)
        .build();

    let levelinfo_help = Embed::builder()
        .title(String::from("Usage"))
        .description(String::from("`gd!aredl levelinfo <number>` where `number` is a placement on AREDL"))
        .color(0xffff00)
        .build();
        
    if args.is_empty() {
        channel.send_embed_message(vec![base_help]).await?;
    } else {
        match args[0] {
            "listinfo" => {
                if args.len() >= 2 {
                    let number = args[1].parse::<usize>();
                    match number {
                        Ok(number) => {
                            if number == 0 {
                                channel.send_embed_message(vec![
                                    Embed::builder()
                                        .title(String::from("Error"))
                                        .description(String::from("That number is too low!"))
                                        .color(0xff0000)
                                        .build(),
                                ]).await?;

                                return Ok(())
                            }

                            let Ok(level_info) = get_aredl_level_info_by_position(number, &aredl_api_token).await else {
                                channel.send_embed_message(vec![
                                    Embed::builder()
                                        .title(String::from("Error"))
                                        .description(String::from("There's not that many levels in the list!"))
                                        .color(0xff0000)
                                        .build(),
                                ]).await?;

                                return Ok(())
                            };

                            channel.send_embed_message(vec![
                                Embed::builder()
                                    .title(String::from("AREDL List Info"))
                                    .description(String::new()
                                    +   "**" + &level_info.name + "**\n"
                                    +   "ID:" + &level_info.level_id.to_string() + "\n"
                                    +   ""
                                    )
                                    .build()
                            ]).await?;
                        },
                        Err(_) => {
                            channel.send_embed_message(vec![
                                Embed::builder()
                                    .title(String::from("Error"))
                                    .description(String::from("Something went wrong. Please check that your input is a positive whole number."))
                                    .color(0xff0000)
                                    .build(),
                                
                                listinfo_help
                            ]).await?;
                        }
                    }
                } else {
                    channel.send_embed_message(vec![levelinfo_help]).await?;
                }
            },

            "levelinfo" => {
                if args.len() >= 2 {
                    let number = args[1].parse::<usize>();
                    match number {
                        Ok(number) => {
                            if number == 0 {
                                channel.send_embed_message(vec![
                                    Embed::builder()
                                        .title(String::from("Error"))
                                        .description(String::from("That number is too low!"))
                                        .color(0xff0000)
                                        .build(),
                                ]).await?;

                                return Ok(())
                            }

                            let Ok(level_info) = get_aredl_gd_info(number, &aredl_api_token).await else {
                                channel.send_embed_message(vec![
                                    Embed::builder()
                                        .title(String::from("Error"))
                                        .description(String::from("There's not that many levels in the list!"))
                                        .color(0xff0000)
                                        .build(),
                                ]).await?;

                                return Ok(())
                            };

                            let author = search_levels(GDQuery::new(&level_info.level_id.to_string())).await.unwrap().split('#').nth(1).unwrap().split(':').nth(1).unwrap().to_owned();
                            let difficulty = calculate_difficulty(level_info.difficulty_numerator.into(), level_info.difficulty_denominator.into(), level_info.auto, level_info.demon).unwrap();
                            let quality = calculate_quality(level_info.feature_score, level_info.epic.into()).unwrap();

                            let embed: Vec<Embed> = vec![
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
                            ];

                            channel.send_embed_message(embed).await?;
                        }

                        Err(_) => {
                            channel.send_embed_message(vec![
                                Embed::builder()
                                    .title(String::from("Error"))
                                    .description(String::from("Something went wrong. Please check that your input is a positive whole number."))
                                    .color(0xff0000)
                                    .build(),
                                
                                listinfo_help
                            ]).await?;
                        }
                    }
                } else {
                    channel.send_embed_message(vec![listinfo_help]).await?;
                }
            },

            _ => {
                channel.send_embed_message(vec![
                    Embed::builder()
                        .title(String::from("Invalid Arguments"))
                        .color(0xff0000)
                        .build(),
                    
                    base_help
                ]).await?;
            }
        }
    }

    Ok(())
}