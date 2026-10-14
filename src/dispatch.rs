use fluxer_rs::{
    api::{FluxerApiHandler, channels::Channel},
    error::{CommandHandlerError, FluxerRsError},
    gateway::dispatch::DispatchHandlerTrait,
    high_level::command_handler::CommandHandler,
    register_commands,
    serde::types::message::{EmbedBuilder, MessageData},
};

use crate::commands::{
    admin_setup::setup::SetupCommand, aredl::AREDLCommand, daily::DailyCommand, event::EventCommand, help::{HELP_MESSAGE_DESCRIPTION, HELP_MESSAGE_TITLE, HelpCommand}, level::LevelCommand, user::UserCommand, weekly::WeeklyCommand,
};

pub struct FluxDashDispatchHandler {}

impl DispatchHandlerTrait for FluxDashDispatchHandler {
    async fn handle_message_create_dispatch(
        &self,
        data: MessageData,
        api: &FluxerApiHandler,
    ) -> Result<(), FluxerRsError> {
        let mut cmd_handler = CommandHandler::init(String::from("gd!"));

        register_commands!(cmd_handler,[
            {"user", UserCommand}
        ,   {"level", LevelCommand}
        ,   {"daily", DailyCommand}
        ,   {"weekly", WeeklyCommand}
        ,   {"event", EventCommand}
        ,   {"help", HelpCommand}
        ,   {"aredl", AREDLCommand}
        ,   {"", HelpCommand}
        ]);

        match cmd_handler.handle(&data, api).await {
            Ok(_) => Ok(()),
            Err(error) => {
                match error {
                    FluxerRsError::CommandHandlerError(CommandHandlerError::UnknownCommand(_)) => {
                        let _ = Channel::new(&data.channel_id, api).send_embed_message(vec![
                            EmbedBuilder::default()
                                .title(String::from("Unknown or invalid command"))
                                .color(0xff0000)
                                .build()
                                .unwrap(),
                            
                            EmbedBuilder::default()
                                .title(String::from(HELP_MESSAGE_TITLE))
                                .description(String::from(HELP_MESSAGE_DESCRIPTION))
                                .color(0xffff00)
                                .build()
                                .unwrap()
                        ]).await?;

                        Err(error)
                    }

                    _ => Err(error)
                }
            }
        }
    }
}