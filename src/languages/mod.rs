pub mod zh;
pub mod pt;
pub mod tr;
pub mod ja;
pub mod ar;
pub mod pl;
pub mod it;
pub mod es;
pub mod ru;
pub mod de;
pub mod en;
pub mod fr;



pub fn available_modes() -> &'static [&'static str] {
    &["love", "hate"]
}

pub fn get_messages(lang: &str, target_sex: &str, mode:&str) -> Option<&'static [&'static str]> {
    match (lang, target_sex, mode) {
        ("fr", "female", "love") => Some(fr::LOVE_FEMALE),
        ("fr", "male", "love") => Some(fr::LOVE_MALE),
        ("fr", "neutral", "love") => Some(fr::LOVE_NEUTRAL),

        ("fr", "female", "hate") => Some(fr::HATE_FEMALE),
        ("fr", "male", "hate") => Some(fr::HATE_MALE),
        ("fr", "neutral", "hate") => Some(fr::HATE_NEUTRAL), 


        ("en", "female", "love") => Some(en::LOVE_FEMALE),
        ("en", "male", "love") => Some(en::LOVE_MALE),
        ("en", "neutral", "love") => Some(en::LOVE_NEUTRAL),

        ("en", "female", "hate") => Some(en::HATE_FEMALE),
        ("en", "male", "hate") => Some(en::HATE_MALE),
        ("en", "neutral", "hate") => Some(en::HATE_NEUTRAL),


        ("de", "female", "love") => Some(de::LOVE_FEMALE),
        ("de", "male", "love") => Some(de::LOVE_MALE),
        ("de", "neutral", "love") => Some(de::LOVE_NEUTRAL),

        ("de", "female", "hate") => Some(de::HATE_FEMALE),
        ("de", "male", "hate") => Some(de::HATE_MALE),
        ("de", "neutral", "hate") => Some(de::HATE_NEUTRAL),


        ("ru", "female", "love") => Some(ru::LOVE_FEMALE),
        ("ru", "male", "love") => Some(ru::LOVE_MALE),
        ("ru", "neutral", "love") => Some(ru::LOVE_NEUTRAL),

        ("ru", "female", "hate") => Some(ru::HATE_FEMALE),
        ("ru", "male", "hate") => Some(ru::HATE_MALE),
        ("ru", "neutral", "hate") => Some(ru::HATE_NEUTRAL),


        ("es", "female", "love") => Some(es::LOVE_FEMALE),
        ("es", "male", "love") => Some(es::LOVE_MALE),
        ("es", "neutral", "love") => Some(es::LOVE_NEUTRAL),

        ("es", "female", "hate") => Some(es::HATE_FEMALE),
        ("es", "male", "hate") => Some(es::HATE_MALE),
        ("es", "neutral", "hate") => Some(es::HATE_NEUTRAL),


        ("it", "female", "love") => Some(it::LOVE_FEMALE),
        ("it", "male", "love") => Some(it::LOVE_MALE),
        ("it", "neutral", "love") => Some(it::LOVE_NEUTRAL),

        ("it", "female", "hate") => Some(it::HATE_FEMALE),
        ("it", "male", "hate") => Some(it::HATE_MALE),
        ("it", "neutral", "hate") => Some(it::HATE_NEUTRAL),


        ("pl", "female", "love") => Some(pl::LOVE_FEMALE),
        ("pl", "male", "love") => Some(pl::LOVE_MALE),
        ("pl", "neutral", "love") => Some(pl::LOVE_NEUTRAL),

        ("pl", "female", "hate") => Some(pl::HATE_FEMALE),
        ("pl", "male", "hate") => Some(pl::HATE_MALE),
        ("pl", "neutral", "hate") => Some(pl::HATE_NEUTRAL),

        
        ("ar", "female", "love") => Some(ar::LOVE_FEMALE),
        ("ar", "male", "love") => Some(ar::LOVE_MALE),
        ("ar", "neutral", "love") => Some(ar::LOVE_NEUTRAL),

        ("ar", "female", "hate") => Some(ar::HATE_FEMALE),
        ("ar", "male", "hate") => Some(ar::HATE_MALE),
        ("ar", "neutral", "hate") => Some(ar::HATE_NEUTRAL),


        ("ja", "female", "love") => Some(ja::LOVE_FEMALE),
        ("ja", "male", "love") => Some(ja::LOVE_MALE),
        ("ja", "neutral", "love") => Some(ja::LOVE_NEUTRAL),

        ("ja", "female", "hate") => Some(ja::HATE_FEMALE),
        ("ja", "male", "hate") => Some(ja::HATE_MALE),
        ("ja", "neutral", "hate") => Some(ja::HATE_NEUTRAL),


        ("tr", "female", "love") => Some(tr::LOVE_FEMALE),
        ("tr", "male", "love") => Some(tr::LOVE_MALE),
        ("tr", "neutral", "love") => Some(tr::LOVE_NEUTRAL),     

        ("tr", "female", "hate") => Some(tr::HATE_FEMALE),
        ("tr", "male", "hate") => Some(tr::HATE_MALE),
        ("tr", "neutral", "hate") => Some(tr::HATE_NEUTRAL), 


        ("pt", "female", "love") => Some(pt::LOVE_FEMALE),
        ("pt", "male", "love") => Some(pt::LOVE_MALE),
        ("pt", "neutral", "love") => Some(pt::LOVE_NEUTRAL),   

        ("pt", "female", "hate") => Some(pt::HATE_FEMALE),
        ("pt", "male", "hate") => Some(pt::HATE_MALE),
        ("pt", "neutral", "hate") => Some(pt::HATE_NEUTRAL), 


        ("zh", "female", "love") => Some(zh::LOVE_FEMALE),
        ("zh", "male", "love") => Some(zh::LOVE_MALE),
        ("zh", "neutral", "love") => Some(zh::LOVE_NEUTRAL),   

        ("zh", "female", "hate") => Some(zh::HATE_FEMALE),
        ("zh", "male", "hate") => Some(zh::HATE_MALE),
        ("zh", "neutral", "hate") => Some(zh::HATE_NEUTRAL), 

        _ => None,
    }
}

pub fn available_languages() -> &'static [&'static str] {
    &[
        "fr", // French
        "en", // English
        "de", // German
        "ru", // Russian
        "es", // Spanish
        "it", // Italian
        "pl", // Polish
        "ar", // Arabic
        "ja", // Japanese
        "tr", // Turkish
        "pt", // Portuguese
        "zh", // Chinese
    ]
}

pub fn available_target_sexes() -> &'static [&'static str] {
    &["female", "male", "neutral"]
}