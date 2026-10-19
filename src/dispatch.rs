use fluxer_rs::{
    api::channels::Channel, error::{CommandHandlerError, FluxerRsError}, gateway::dispatch::DispatchHandlerTrait, high_level::command_handler::CommandHandler, register_commands, serde::types::message::{Embed, MessageData},
};

use crate::commands::{
    admin_setup::setup::SetupCommand, aredl::AREDLCommand, daily::DailyCommand, event::EventCommand, help::{HELP_MESSAGE_DESCRIPTION, HELP_MESSAGE_TITLE, HelpCommand}, level::LevelCommand, user::UserCommand, weekly::WeeklyCommand,
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

    async fn handle_ready_dispatch<'a>(&self, data: fluxer_rs::serde::types::user::ReadyData, _context: &'a fluxer_rs::context::FluxerClientContext, _original_text: String) -> Result<(), FluxerRsError> {
        // data.sessions.first().unwrap().status = String::from(format!("Dashing along {} guilds", &data.guilds.len().to_string()));

        Ok(())
    }
}