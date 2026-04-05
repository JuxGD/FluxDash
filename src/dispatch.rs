
use fluxer_rs::{
    api::{FluxerApiHandler},
    error::FluxerRsError,
    gateway::dispatch::DispatchHandlerTrait,
    high_level::{
        command_handler::{
            CommandHandler
        }
    },
    register_commands,
    serde::types::{
        message::{
            MessageData
        },
    },
};

use crate::commands::{
    daily::DailyCommand, event::EventCommand, level::LevelCommand, user::UserCommand, weekly::WeeklyCommand, wrong::HelpCommand
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
        ,   {"", HelpCommand}
        ]);

        match cmd_handler.handle(&data, api).await {
            Ok(_) => Ok(()),
            Err(_) => Ok(())
        }
    }
}