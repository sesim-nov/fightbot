use poise::serenity_prelude::{self as serenity};
use crate::{Context, Error};


#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Registers a commander for a fight
#[poise::command(slash_command)]
pub async fn reg(
    ctx: Context<'_>,
    #[description = "CMDRs per team (i.e. for 4v4 say '4')"] team_size: u8,
    #[description = "Selected user, leave blank to register yourself."] user: Option<serenity::User>, 
) -> Result<(), Error> {
    todo!();
}

/// Cancels a given queue
#[poise::command(slash_command)]
pub async fn cancel(
    ctx: Context<'_>,
    #[description = "CMDRs per team (i.e. for 4v4 say '4')"] team_size: u8,
) -> Result<(), Error> {
    todo!();
}

/// Force-starts queue even if a queue is not full. 
#[poise::command(slash_command)]
pub async fn start(
    ctx: Context<'_>,
    #[description = "CMDRs per team (i.e. for 4v4 say '4')"] team_size: u8,
) -> Result<(), Error> {
    todo!();
}

/// Removes target user from queue 
#[poise::command(slash_command)]
pub async fn rm(
    ctx: Context<'_>,
    #[description = "CMDRs per team (i.e. for 4v4 say '4')"] team_size: u8,
    #[description = "Selected user, leave blank to krill yourself."] user: Option<serenity::User>, 
) -> Result<(), Error> {
    todo!();
}