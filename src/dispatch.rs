use fluxer_rs::{
    api::channels::Channel, error::{CommandHandlerError, FluxerRsError}, gateway::dispatch::DispatchHandlerTrait, high_level::command_handler::CommandHandler, register_commands, serde::types::message::{Embed, MessageData},
};

use crate::commands::{
    admin_setup::setup::{FluxDashGuildConfig, GDServerActivity, SetupCommand, send_gd_server_update}, aredl::AREDLCommand, daily::DailyCommand, event::EventCommand, help::{HELP_MESSAGE_DESCRIPTION, HELP_MESSAGE_TITLE, HelpCommand}, level::LevelCommand, user::UserCommand, weekly::WeeklyCommand,
};

pub struct FluxDashDispatchHandler;

impl DispatchHandlerTrait for FluxDashDispatchHandler {

    async fn handle_message_create_dispatch(&self, data: MessageData, context: &fluxer_rs::context::FluxerClientContext, _original_text: String) -> Result<(), FluxerRsError> {
        let api = &context.api;
        let mut cmd_handler = CommandHandler::init(String::from("gd!"));

        register_commands!(cmd_handler,[
            {"user", UserCommand}
        ,   {"level", LevelCommand}
        ,   {"daily", DailyCommand}
        ,   {"weekly", WeeklyCommand}
        ,   {"event", EventCommand}
        ,   {"help", HelpCommand}
        ,   {"aredl", AREDLCommand}
        ,   {"setup", SetupCommand}
        ,   {"", HelpCommand}
        ]);

        match cmd_handler.handle(&data, api).await {
            Ok(()) => Ok(()),
            Err(error) => {
                match error {
                    FluxerRsError::CommandHandlerError(CommandHandlerError::UnknownCommand(_)) => {
                        let _ = Channel::new(&data.channel_id, api).send_embed_message(vec![
                            Embed::builder()
                                .title(String::from("Unknown or invalid command"))
                                .color(0xff0000)
                                .build(),
                            
                            Embed::builder()
                                .title(String::from(HELP_MESSAGE_TITLE))
                                .description(String::from(HELP_MESSAGE_DESCRIPTION))
                                .color(0xffff00)
                                .build()
                        ]).await?;

                        Err(error)
                    }

                    _ => Err(error)
                }
            }
        }
    }

async fn handle_guild_create_dispatch(&self, data: fluxer_rs::serde::types::guild::GuildCreateData, context: &fluxer_rs::context::FluxerClientContext, _original_text: String) -> Result<(), FluxerRsError> {
        let guild = data.id;
        let api = &context.api;

        loop {
            if tokio::fs::try_exists("guilds_configs").await.unwrap() {
                let mut guilds_configs = tokio::fs::read_dir("guilds_configs").await.unwrap();

                while let Some(file) = guilds_configs.next_entry().await.unwrap() {
                    println!("{}", file.file_name().display().to_string() == format!("{guild}.json"));
                    if file.file_name().display().to_string() == format!("{guild}.json") {
                        let guild: FluxDashGuildConfig = serde_json::from_slice(&tokio::fs::read(&file.path().display().to_string()).await.unwrap())?;
                        send_gd_server_update(guild, GDServerActivity::NewDaily, api).await.unwrap();
                    };
                };
            };
        }
    }
}