
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
    ping::PingCommand
,   user::UserCommand
,   daily::DailyCommand
,   weekly::WeeklyCommand
,   event::EventCommand
,   wrong::WrongCommand
,   level::LevelCommand
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
            {"ping", PingCommand}
        ,   {"user", UserCommand}
        ,   {"level", LevelCommand}
        ,   {"daily", DailyCommand}
        ,   {"weekly", WeeklyCommand}
        ,   {"event", EventCommand}
        ,   {"", WrongCommand}
        ]);

        match cmd_handler.handle(&data, api).await {
            Ok(_) => Ok(()),
            Err(_) => Ok(())
        }
    }
}