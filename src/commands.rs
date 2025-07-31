use poise::serenity_prelude::{self as serenity, Mentionable, UserId};
use crate::{Context, Error, FightId};


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
    #[description = "CMDRs per team (i.e. for 4v4 say '4')"] team_size: usize,
    #[description = "Selected user, leave blank to register yourself."] user: Option<serenity::User>, 
) -> Result<(), Error> {
    let response = {
        let user = match user {
            Some(u) => u,
            None => ctx.author().clone(),
        };
        let mut map = ctx.data().queues.lock().expect("Failed to acquire lock on Mutex");
        let fight_id = FightId {
            guild_id: ctx.guild_id().unwrap(),
            size: team_size,
        };
        let fight = match map.get_mut(&fight_id) {
            Some(fight) => fight,
            None => {
                map.insert(fight_id.clone(), std::collections::HashSet::new());
                map.get_mut(&fight_id).unwrap()
            }
        };
        if fight.insert(UserId::from(user)) {
            let mut resp = "Insertion successful".to_string();
            if fight.len() >= team_size * 2 {
                let mut combatants: Vec<UserId> = fight.iter().map(|x| x.to_owned()).collect();
                let center = combatants.len() / 2;
                let other_combatants = combatants.split_off(center);
                resp.push_str("\nMATCH START: \nTeam 1\n");
                for x in combatants {
                    let m = x.mention();
                    resp.push_str(&format!("{m}\n"));
                }
                resp.push_str("TEAM 2:\n");
                for x in other_combatants {
                    let m = x.mention();
                    resp.push_str(&format!("{m}\n"));
                }
            }
            resp
        } else {
            "You're already registered you fucking melt.".to_owned()
        }
    };
    ctx.say(response).await?;
    Ok(())
}

//fn handle_match_start

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