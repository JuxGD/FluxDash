use fluxer_rs::{
    api::channels::Channel, command, serde::types::message::Embed
};

pub const HELP_MESSAGE_TITLE: &str = "Available Commands";
pub const HELP_MESSAGE_DESCRIPTION: &str = "
- `gd!level <levelID/levelname>` - Returns requested level info
- `gd!user <userID/username>` - Returns requested user's stats
- `gd!daily` - Returns daily level info
- `gd!weekly` - Returns weekly level info
- `gd!event` - Returns event level info
- `gd!aredl` - AREDL commands
- `gd!setup` - Set up server updates channels (for guild owner only)
";

#[command(HelpCommand)]
async fn execute(api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let channel = Channel::new(&data.channel_id, api);
    let embeds = vec![Embed::builder()
            .title(String::from(HELP_MESSAGE_TITLE))
            .description(String::from(HELP_MESSAGE_DESCRIPTION))
            .color(0xffff00)
            .build()
    ];

    channel.send_embed_message(embeds).await?;

    Ok(())
}