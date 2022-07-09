use anyhow::{anyhow, Result};
use serenity::{client::Context, model::guild::Member};
use std::io::Write;

use crate::{handler::Command, UserConfig};

pub async fn simple_wolfram_alpha(input: &str) -> Result<String> {
    dotenv::dotenv().ok();
    let url = "http://api.wolframalpha.com/v2/simple";
    let app_id = std::env::var("WOLFRAM_ALPHA_APP_ID")?;

    let params = [("i", input), ("appid", &app_id)];
    let client = reqwest::Client::new();
    let res = client.get(url).query(&params).send().await?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let path = format!("temp/{}.gif", now);
    let mut file = std::fs::File::create(&path)?;
    file.write(&res.bytes().await?).ok();
    Ok(path)
}

pub async fn rand_member(command: &Command, ctx: &Context) -> Result<Member> {
    let guild_id = command.guild_id.ok_or(anyhow!("guild does not exist"))?;
    let guild = ctx
        .cache
        .guild(guild_id)
        .await
        .ok_or(anyhow!("guild does not exist"))?;
    let voice_states = guild.voice_states;
    let vc_members = voice_states.keys().collect::<Vec<_>>();
    let len = vc_members.len();
    let i: usize = rand::random();
    let user_id = vc_members[i % len];
    ctx.cache
        .member(guild_id, user_id)
        .await
        .ok_or(anyhow!("member not found"))
}