use fluxer_rs::serde::types::message::Embed;
use gdutils::gd::{levels::search_levels, utils::{Difficulty, GDQuery, Level, Quality, TimelyInfo, TimelyType}};
use time_hms::TimeHms;

pub fn feature_score(feature_score: i32) -> String {
    if feature_score > 0 {
        format!(" (Feature score: {feature_score})")
    } else {
        String::new()
    }
}

pub fn verified_coins(verified_coins: bool) -> String {
    if verified_coins {
        String::from(" (Verified coins)")
    } else {
        String::new()
    }
}

pub async fn make_level_embed(level_info: Level) -> Result<Embed, color_eyre::eyre::Report> {
    let author = search_levels(GDQuery::new(&level_info.level_id.to_string())).await.unwrap().split('#').nth(1).unwrap().split(':').nth(1).unwrap().to_owned();

    let difficulty = match level_info.calculate_difficulty()? {
        Difficulty::NA => "N/A",
        Difficulty::Unrated => "Unrated",
        Difficulty::Easy => "Easy",
        Difficulty::Normal => "Normal",
        Difficulty::Hard => "Hard",
        Difficulty::Harder => "Harder",
        Difficulty::Insane => "Insane",
        Difficulty::EasyDemon => "Easy Demon",
        Difficulty::MediumDemon => "Medium Demon",
        Difficulty::HardDemon => "Hard Demon",
        Difficulty::InsaneDemon => "Insane Demon",
        Difficulty::ExtremeDemon => "Extreme Demon",
        Difficulty::Auto => "Auto"

    };

    let quality = match level_info.calculate_quality() {
        Quality::Normal => "Normal",
        Quality::Featured => "Featured",
        Quality::Epic => "Epic",
        Quality::Legendary => "Legendary",
        Quality::Mythic => "Mythic"
    };
    
    Ok(Embed::builder()
        .title(String::from(&level_info.level_name))
        .description(String::new()
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
    )
}

pub async fn make_timely_level_embed(timely_type: TimelyType, level_info: Level, timely_info: TimelyInfo) -> Result<Embed, color_eyre::eyre::Report> {
    let author = search_levels(GDQuery::new(&level_info.level_id.to_string())).await.unwrap().split('#').nth(1).unwrap().split(':').nth(1).unwrap().to_owned();
    
    let difficulty = match level_info.calculate_difficulty()? {
        Difficulty::NA => "N/A",
        Difficulty::Unrated => "Unrated",
        Difficulty::Easy => "Easy",
        Difficulty::Normal => "Normal",
        Difficulty::Hard => "Hard",
        Difficulty::Harder => "Harder",
        Difficulty::Insane => "Insane",
        Difficulty::EasyDemon => "Easy Demon",
        Difficulty::MediumDemon => "Medium Demon",
        Difficulty::HardDemon => "Hard Demon",
        Difficulty::InsaneDemon => "Insane Demon",
        Difficulty::ExtremeDemon => "Extreme Demon",
        Difficulty::Auto => "Auto"

    };

    let quality = match level_info.calculate_quality() {
        Quality::Normal => "Normal",
        Quality::Featured => "Featured",
        Quality::Epic => "Epic",
        Quality::Legendary => "Legendary",
        Quality::Mythic => "Mythic"
    };
    let timely_type_str: Result<&str, color_eyre::eyre::Report> = match timely_type {
        TimelyType::Daily => Ok("Daily"),
        TimelyType::Weekly => Ok("Weekly"),
        TimelyType::Event => Ok("Event")
    };

    let time_left: Result<String, color_eyre::eyre::Report> = match timely_type {
        TimelyType::Daily | TimelyType::Weekly => Ok(format!("Time Left: {}", TimeHms::new(timely_info.time_left as u64))),
        TimelyType::Event => Ok(String::new())
    };

    let timely_index: Result<i32, color_eyre::eyre::Report> = match timely_type {
        TimelyType::Daily => Ok(timely_info.timely_index),
        TimelyType::Weekly => Ok(timely_info.timely_index - 100000),
        TimelyType::Event => Ok(timely_info.timely_index - 200000)
    };

    Ok(Embed::builder()
        .title(String::from(&level_info.level_name))
        .description(String::new()
        +   "ID: " + &level_info.level_id.to_string() + "\n"
        +   "Author: " + &author + "\n"
        +   "Difficulty: " + &difficulty + "\n"
        +   "Quality: " + &quality + &feature_score(level_info.feature_score) + "\n"
        +   "Stars: " + &level_info.stars.to_string() + "\n"
        +   "Coins: " + &level_info.coins.to_string() + &verified_coins(level_info.verified_coins) + "\n"
        +   "Likes: " + &level_info.likes.to_string() + "\n"
        +   "Downloads: " + &level_info.downloads.to_string() + "\n"
        +   "Song ID: " + &level_info.custom_song_id.to_string() + "\n"
        +   "\n"
        +   &format!("{} #", timely_type_str?).to_string() + &timely_index?.to_string() + "\n"
        +   &time_left?
        )
        .color(0x0055ff)
        .build()
    )
}