pub fn feature_score(feature_score: i32) -> String {
    if feature_score > 0 {
        return format!(" (Feature score: {})", feature_score);
    } else {
        return String::from("");
    }
}

pub fn verified_coins(verified_coins: bool) -> String {
    if verified_coins == true {
        return String::from(" (Verified coins)");
    } else {
        return String::from("");
    }
}