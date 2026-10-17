use std::env;

use gdutils::{aredl::classic::{get_aredl_gd_info, get_aredl_level_info_by_position}};

use fluxer_rs::{
    api::channels::Channel, command, serde::types::message::Embed
};

use crate::utils::levels::make_level_embed;

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

                            let embed: Vec<Embed> = vec![make_level_embed(level_info).await.unwrap()];

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