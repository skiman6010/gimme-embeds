use crate::commands::ChatPreferences;
use crate::commands::InstagramDomainChoice;
use std::fs::{DirBuilder, File};
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub fn save_preferences(chat_id: i64, preferences: &ChatPreferences) -> std::io::Result<()> {
    let dir = Path::new("preferences");
    if !dir.exists() {
        DirBuilder::new().create(dir)?;
    }
    let filename = format!("preferences/{}.json", chat_id);
    let json = serde_json::to_string(preferences).unwrap();
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_preferences(chat_id: i64) -> ChatPreferences {
    let filename = format!("preferences/{}.json", chat_id);
    match File::open(&filename) {
        Ok(mut file) => {
            let mut json = String::new();
            file.read_to_string(&mut json).unwrap();
            serde_json::from_str(&json).unwrap()
        }
        Err(_) => {
            // Return a new ChatPreferences with default values
            ChatPreferences {
                embed_twitter: true,
                embed_tiktok: true,
                embed_instagram: true,
                embed_x: true,
                instagram_domain: InstagramDomainChoice::Kkinstagram,
            }
        }
    }
}
