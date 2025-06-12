mod commands;
mod helpers;

use crate::commands::{ChatPreferences, InstagramDomainChoice};
use dotenv::dotenv;
use regex::Regex;
use std::collections::HashMap;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting embeds bot....");

    let bot = Bot::from_env();

    teloxide::repl(bot, answer).await;

    async fn answer(bot: Bot, msg: Message) -> ResponseResult<()> {
        let text = msg.text().unwrap_or_default();
        let chat_id = msg.chat.id.0;
        helpers::load_preferences(chat_id);
        let mut chat_preferences: HashMap<i64, ChatPreferences> = HashMap::new();
        let preferences = chat_preferences
            .entry(chat_id)
            .or_insert_with(|| helpers::load_preferences(chat_id));

        log::info!("Received message: {}", text);

        commands::handler(bot.clone(), msg.clone(), preferences).await?;

        let instagram_domain_str = match preferences.instagram_domain {
            InstagramDomainChoice::Ddinstagram => "ddinstagram.com",
            InstagramDomainChoice::Kkinstagram => "kkinstagram.com",
            InstagramDomainChoice::Instagramez => "instagramez.com",
        };
        let instagram_replacement = format!("{}/{}", instagram_domain_str, "${chunk}");

        let url_patterns = vec![
            (
                Regex::new(r"(?P<url>twitter.com/)(?P<chunk>\S+)").unwrap(),
                "vxtwitter.com/${chunk}",
                &preferences.embed_twitter,
            ),
            (
                Regex::new(r"(?P<url>tiktok.com/)(?P<chunk>\S+)").unwrap(),
                "vxtiktok.com/${chunk}",
                &preferences.embed_tiktok,
            ),
            (
                Regex::new(r"(?P<url>instagram.com/)(?P<chunk>\S+)").unwrap(),
                &instagram_replacement,
                &preferences.embed_instagram,
            ),
            (
                Regex::new(r"(?P<url>\bx.com\b/)(?P<chunk>\S+)").unwrap(),
                "fixvx.com/${chunk}",
                &preferences.embed_x,
            ),
        ];

        let anti_url_patterns = vec![
            Regex::new(r"(?P<url>twitter.com/i/events/\S+)").unwrap(),
            Regex::new(r"(?P<url>twitter.com/i/spaces/\S+)").unwrap(),
        ];

        for (re, replacement, enabled) in url_patterns {
            if *enabled
                && re.is_match(&text)
                && !anti_url_patterns.iter().any(|re| re.is_match(&text))
                && !text.starts_with("https://vx")
                && !text.starts_with("https://dd")
            {
                log::info!("URL detected: {}", text);
                let url_without_tracking = text.split('?').next().unwrap_or(&text);
                let new_url = re.replace(url_without_tracking, replacement);
                log::info!("New URL: {}", new_url);
                let _ = bot
                    .send_message(msg.chat.id, new_url)
                    .reply_to_message_id(msg.id)
                    .await?;
                break;
            }
        }

        Ok(())
    }
}
