use std::sync::Arc;

use serenity::{
    client::Context,
    model::{
        id::{ChannelId, GuildId},
        interactions::application_command::ApplicationCommandInteraction,
    },
};
use songbird::{Event, TrackEvent};
type SlashCommandResult = Result<String, String>;
use crate::TrackEndNotifier;
use anyhow::{anyhow, Result};
use tokio::sync::Mutex;

pub async fn join(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    read_channel_id: &Arc<Mutex<Option<ChannelId>>>,
) -> Result<String> {
    let guild_id = command.guild_id.unwrap();
    let author_id = command.member.as_ref().unwrap().user.id;
    let text_channel_id = command.channel_id;
    let channel_id = command
        .guild_id
        .unwrap()
        .to_guild_cached(&ctx.cache)
        .await
        .unwrap()
        .voice_states
        .get(&author_id)
        .and_then(|voice_state| voice_state.channel_id)
        .unwrap();
    let connect_to = channel_id;
    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let (handle_lock, _) = manager.join(guild_id, connect_to).await;
    let mut handle = handle_lock.lock().await;
    handle.deafen(true).await.unwrap();
    handle.add_global_event(Event::Track(TrackEvent::End), TrackEndNotifier);
    *read_channel_id.lock().await = Some(text_channel_id);
    Ok("おはよ！".to_string())
}

pub async fn leave(ctx: &Context, guild_id: GuildId) -> Result<String> {
    //let guild_id = command.guild_id.unwrap();
    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();
    if has_handler {
        manager
            .remove(guild_id)
            .await
            .map_err(|e| anyhow!("{}", e))?;
        Ok("ばいばい".to_string())
    } else {
        Err(anyhow!("ボイスチャンネルに入ってないよ"))
    }
}

pub async fn mute(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<String> {
    let guild_id = command.guild_id.unwrap();
    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let handler_lock = manager
        .get(guild_id)
        .ok_or(anyhow!("ボイスチャンネルに入ってないよ"))?;
    let mut handler = handler_lock.lock().await;

    if handler.is_mute() {
        Err(anyhow!("もうミュートしてるよ"))
    } else {
        if let Err(e) = handler.mute(true).await {
            Err(e.into())
        } else {
            Ok("ミュートしたよ".to_string())
        }
    }
}

pub async fn unmute(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<String> {
    let guild_id = command.guild_id.unwrap();
    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let handler_lock = manager
        .get(guild_id)
        .ok_or(anyhow!("ボイスチャンネルに入ってないよ"))?;
    let mut handler = handler_lock.lock().await;
    if let Err(e) = handler.mute(false).await {
        Err(e.into())
    } else {
        Ok("ミュート解除したよ".to_string())
    }
}
