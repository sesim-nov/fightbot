use crate::{Context, Error, FightId};
use poise::serenity_prelude::{self as serenity, GuildId, Mentionable, UserId};
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::sync::MutexGuard;

static VALID_FIGHT_TYPES: [usize; 4] = [1, 2, 3, 4];

/// Registers a commander for a fight
#[poise::command(slash_command)]
pub async fn reg(
    ctx: Context<'_>,
    #[description = "CMDRs per team (i.e. for 4v4 say '4')"] team_size: usize,
    #[description = "Selected user, leave blank to register yourself."] user: Option<
        serenity::User,
    >,
) -> Result<(), Error> {
    // Generate response message for our bot
    let response = {
        // RNG for randomizing the roster. 
        let mut our_rng = rand::thread_rng();

        // If a target user was not provided by the user, assume self-registration. 
        let user = user.unwrap_or(ctx.author().to_owned());
        let user_id = UserId::from(&user);
        let guild_id = ctx.guild_id().unwrap();
        let fight_id = FightId {
            guild_id,
            size: team_size,
        };

        // Acquire Mutex for the list of fight queues. 
        let mut fights = ctx
            .data()
            .queues
            .lock()
            .expect("Failed to acquire lock on Mutex");

        let is_registered = is_already_registered(&user_id, &guild_id, &fights);

        if is_registered {
            "You're already registered you fucking melt.".to_owned()
        } else {
            // Find the fight the user requested registration for. If it diesn't exist, create it. 
            let fight = match fights.get_mut(&fight_id) {
                Some(fight) => fight,
                None => {
                    fights.insert(fight_id.clone(), std::collections::HashSet::new());
                    fights.get_mut(&fight_id).unwrap()
                }
            };
            // Add the user to the selected fight. This should always succeed if 
            // is_already_registered is working correctly. 
            fight.insert(UserId::from(&user));
            let men = user.mention();
            let mut resp =
                format!("Successfully registered {men} for a {team_size}v{team_size}").to_string();

            // Check if the fight is full
            if fight.len() >= team_size * 2 {
                // If the fight is full, print out the teams and clear the queue. 
                let mut combatants: Vec<UserId> = fight.iter().map(|x| x.to_owned()).collect();
                combatants.shuffle(&mut our_rng);
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
    fight_list: &MutexGuard<'_, HashMap<FightId, HashSet<UserId>>>,
) -> bool {
    VALID_FIGHT_TYPES.iter().any(|fight_type| -> bool {
        let tmp_match_id = FightId {
            guild_id: guild_id.to_owned(),
            size: *fight_type,
        };
        match fight_list.get(&tmp_match_id) {
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
    #[description = "CMDRs per team (i.e. for 4v4 say '4')"] team_size: usize,
) -> Result<(), Error> {
    {
        let mut fights = ctx
            .data()
            .queues
            .lock()
            .expect("Failed to acquire lock on mutex.");
        let guild_id = ctx.guild_id().expect("No Guild ID??");
        let fight_id = FightId {
            guild_id,
            size: team_size,
        };

        match fights.get_mut(&fight_id) {
            None => (),
            Some(fight) => {
                fight.clear();
            }
        }
    }
    ctx.say(format!("{team_size}v{team_size} queue cleared."))
        .await?;
    Ok(())
}

/// Force-starts queue even if a queue is not full.
#[poise::command(slash_command)]
pub async fn start(
    ctx: Context<'_>,
    #[description = "CMDRs per team (i.e. for 4v4 say '4')"] team_size: usize,
) -> Result<(), Error> {
    // Build response string. 
    let response = {
        // RNG for randomizing the roster. 
        let mut our_rng = rand::thread_rng();

        let fight_id = FightId{
            guild_id: ctx.guild_id().ok_or("No Guild ID in context.")?,
            size: team_size,
        };

        let mut resp = String::new();

        // I don't want this to panic, so I convert the error instead of using an expect. 
        let mut fights = match ctx.data().queues.lock(){
            Ok(a) => Ok(a),
            Err(_) => Err("Failed to acquire mutex."),
        }?;

        let fight = fights.get_mut(&fight_id).ok_or("No fight found with those parameters.")?;

        let mut combatants: Vec<UserId> = fight.iter().map(|x| x.to_owned()).collect();
        combatants.shuffle(&mut our_rng);
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

        resp
    };
    ctx.say(response).await?;
    Ok(())
}

/// Removes target user from all server queues
#[poise::command(slash_command)]
pub async fn rm(
    ctx: Context<'_>,
    #[description = "Selected user, leave blank to krill yourself."] user: Option<serenity::User>,
) -> Result<(), Error> {
    todo!();
}
