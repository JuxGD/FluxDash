use fluxer_rs::serde::types::message::Embed;
use gdutils::gd::{levels::{calculate_difficulty, calculate_quality, search_levels}, utils::{GDQuery, Level, TimelyLevel, TimelyType}};
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

pub async fn make_timely_level_embed(timely_type: TimelyType, timely_level: TimelyLevel) -> Result<Embed, color_eyre::eyre::Report> {
    let level_info = timely_level.level;

    let author = search_levels(GDQuery::new(&level_info.level_id.to_string())).await.unwrap().split('#').nth(1).unwrap().split(':').nth(1).unwrap().to_owned();
    let difficulty = calculate_difficulty(level_info.difficulty_numerator.into(), level_info.difficulty_denominator.into(), level_info.auto, level_info.demon).unwrap();
    let quality = calculate_quality(level_info.feature_score, level_info.epic.into()).unwrap();

    let timely_type_str = match timely_type {
        TimelyType::Daily => "Daily",
        TimelyType::Weekly => "Weekly",
        TimelyType::Event => "Event",
        TimelyType::Unknown => "",
    };

    let time_left: Result<String, color_eyre::eyre::Report> = match timely_type {
        TimelyType::Daily | TimelyType::Weekly => Ok(format!("Time Left: {}", TimeHms::new(timely_level.time_left as u64))),
        _ => Ok(String::new())

    };

    return Ok(Embed::builder()
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
        +   &format!("{} #", timely_type_str).to_string() + &timely_level.timely_index.to_string() + "\n"
        +   &time_left.unwrap()
        )
        .color(0x0055ff)
        .build()
    )
}

pub async fn make_level_embed(level_info: Level) -> Result<Embed, color_eyre::eyre::Report> {
    let author = search_levels(GDQuery::new(&level_info.level_id.to_string())).await.unwrap().split('#').nth(1).unwrap().split(':').nth(1).unwrap().to_owned();
    let difficulty = calculate_difficulty(level_info.difficulty_numerator.into(), level_info.difficulty_denominator.into(), level_info.auto, level_info.demon).unwrap();
    let quality = calculate_quality(level_info.feature_score, level_info.epic.into()).unwrap();
    return Ok(Embed::builder()
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