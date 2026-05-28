pub fn feature_score(feature_score: i32) -> String {
    if feature_score > 0 {
        format!(" (Feature score: {})", feature_score)
    } else {
        String::from("")
    }
}

pub fn verified_coins(verified_coins: bool) -> String {
    if verified_coins {
        String::from(" (Verified coins)")
    } else {
        String::from("")
    }
}