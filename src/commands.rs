use poise::serenity_prelude::{self as serenity, GuildId, Mentionable, UserId};
use tokio::time::timeout_at;
use crate::{Context, Error, FightId};
use std::collections::{HashMap, HashSet};
use std::sync::MutexGuard;

static valid_fight_types: [usize; 4] = [1,2,3,4];

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
        let user_id = UserId::from(&user);
        let guild_id = ctx.guild_id().unwrap();
        let fight_id = FightId {
            guild_id,
            size: team_size,
        };
        let mut fights = ctx.data().queues.lock().expect("Failed to acquire lock on Mutex");

        let is_registered = is_already_registered(&user_id, &guild_id, &fights);

        if is_registered {
            "You're already registered you fucking melt.".to_owned()
        } else {
            let fight = match fights.get_mut(&fight_id) {
                Some(fight) => fight,
                None => {
                    fights.insert(fight_id.clone(), std::collections::HashSet::new());
                    fights.get_mut(&fight_id).unwrap()
                }
            };
            fight.insert(UserId::from(user));
            
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
                // Clear queue
                fight.clear();
            }
            resp
        }
    };
    ctx.say(response).await?;
    Ok(())
}

fn is_already_registered(
    u: &UserId, 
    guild_id: &GuildId, 
    fight_list: &MutexGuard<'_, HashMap<FightId, HashSet<UserId>>> 
) -> bool {
    valid_fight_types.iter().any(|fight_type| -> bool {
        let tmp_match_id = FightId {
            guild_id: guild_id.to_owned(),
            size: *fight_type,
        };
        match fight_list.get(&tmp_match_id){
            None => false,
            Some(combat_list) => combat_list.contains(u),
        }
    })
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