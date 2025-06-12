use crate::helpers::save_preferences;
use serde::{Deserialize, Serialize};
use teloxide::{prelude::*, utils::command::BotCommands};
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported: "
)]
pub enum Command {
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Enable twitter embeds.")]
    EnableTwitter,
    #[command(description = "Disable twitter embeds.")]
    DisableTwitter,
    #[command(description = "Enable tiktok embeds.")]
    EnableTiktok,
    #[command(description = "Disable tiktok embeds.")]
    DisableTiktok,
    #[command(description = "Enable instagram embeds.")]
    EnableInstagram,
    #[command(description = "Disable instagram embeds.")]
    DisableInstagram,
    #[command(description = "Enable 𝕏 embeds.")]
    EnableX,
    #[command(description = "Disable 𝕏 embeds.")]
    DisableX,
    #[command(description = "Use ddinstagram.com for Instagram embeds.")]
    UseDdInstagram,
    #[command(description = "Use kkinstagram.com for Instagram embeds.")]
    UseKkInstagram,
    #[command(description = "Use instagramez.com for Instagram embeds.")]
    UseInstagramez,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum InstagramDomainChoice {
    Ddinstagram,
    Kkinstagram,
    Instagramez,
}

impl Default for InstagramDomainChoice {
    fn default() -> Self {
        InstagramDomainChoice::Kkinstagram
    }
}

#[derive(Serialize, Deserialize)]
pub struct ChatPreferences {
    pub embed_twitter: bool,
    pub embed_tiktok: bool,
    pub embed_instagram: bool,
    pub embed_x: bool,
    #[serde(default)] // Ensure new field has a default for old stored preferences
    pub instagram_domain: InstagramDomainChoice,
}

pub async fn handler(
    bot: Bot,
    msg: Message,
    preferences: &mut ChatPreferences,
) -> ResponseResult<()> {
    let text = msg.text().unwrap_or_default();
    let chat_id = msg.chat.id.0;

    if let Ok(command) = Command::parse(text, "embeds") {
        match command {
            Command::Help => {
                let _ = bot
                    .send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
            }
            Command::EnableTwitter => {
                preferences.embed_twitter = true;
                let _ = bot
                    .send_message(msg.chat.id, "Twitter embeds enabled.")
                    .await?;
            }
            Command::DisableTwitter => {
                preferences.embed_twitter = false;
                let _ = bot
                    .send_message(msg.chat.id, "Twitter embeds disabled.")
                    .await?;
            }
            Command::EnableTiktok => {
                preferences.embed_tiktok = true;
                let _ = bot
                    .send_message(msg.chat.id, "Tiktok embeds enabled.")
                    .await?;
            }
            Command::DisableTiktok => {
                preferences.embed_tiktok = false;
                let _ = bot
                    .send_message(msg.chat.id, "Tiktok embeds disabled.")
                    .await?;
            }
            Command::EnableInstagram => {
                preferences.embed_instagram = true;
                let _ = bot
                    .send_message(msg.chat.id, "Instagram embeds enabled.")
                    .await?;
            }
            Command::DisableInstagram => {
                preferences.embed_instagram = false;
                let _ = bot
                    .send_message(msg.chat.id, "Instagram embeds disabled.")
                    .await?;
            }
            Command::EnableX => {
                preferences.embed_x = true;
                let _ = bot.send_message(msg.chat.id, "𝕏 embeds enabled.").await?;
            }
            Command::DisableX => {
                preferences.embed_x = false;
                let _ = bot.send_message(msg.chat.id, "𝕏 embeds disabled.").await?;
            }
            Command::UseDdInstagram => {
                preferences.instagram_domain = InstagramDomainChoice::Ddinstagram;
                let _ = bot
                    .send_message(msg.chat.id, "Instagram embeds will use ddinstagram.com.")
                    .await?;
            }
            Command::UseKkInstagram => {
                preferences.instagram_domain = InstagramDomainChoice::Kkinstagram;
                let _ = bot
                    .send_message(msg.chat.id, "Instagram embeds will use kkinstagram.com.")
                    .await?;
            }
            Command::UseInstagramez => {
                preferences.instagram_domain = InstagramDomainChoice::Instagramez;
                let _ = bot
                    .send_message(msg.chat.id, "Instagram embeds will use instagramez.com.")
                    .await?;
            }
        }
        save_preferences(chat_id, preferences)?;
    }

    Ok(())
}
