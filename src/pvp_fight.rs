use std::collections::HashSet;

use poise::serenity_prelude::{CreateEmbed, Mentionable, UserId};
use rand::seq::SliceRandom;
use uuid::Uuid;

use crate::Error;

pub struct PVPTeams(Vec<UserId>, Vec<UserId>);

pub struct PVPFight {
    id: Uuid,
    pool_size: usize,
    team_pool: HashSet<UserId>,
}

impl PVPFight {
    /// Instance a new fight
    pub fn new(team_size: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            pool_size: 2 * team_size,
            team_pool: HashSet::new(),
        }
    }

    /// Register a participant
    pub fn reg(&mut self, add_id: UserId) -> Result<(), Error> {
        if self.pool_size <= self.team_pool.len() {
            Err("Team Full".into())
        } else {
            self.team_pool.insert(add_id);
            Ok(())
        }
    }

    /// Remove a partiipant
    pub fn rm(&mut self, rm_id: &UserId) -> bool {
        self.team_pool.remove(rm_id)
    }

    /// Generate 2 random teams from the current pool.
    fn get_teams(&self) -> PVPTeams {
        let mut rng = rand::thread_rng();
        let mut combatants: Vec<UserId> = self.team_pool.iter().map(|i| i.to_owned()).collect();
        combatants.shuffle(&mut rng);
        let center = combatants.len() / 2;
        let other_combatants = combatants.split_off(center);
        PVPTeams(combatants, other_combatants)
    }

    /// Get a list of current participants as a newline separated string.
    fn get_pool_list(&self) -> String {
        self.team_pool
            .iter()
            .map(|x| x.mention().to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    // Check if we're ready to start a match.
    pub fn ready_to_start(&self) -> bool {
        self.pool_size <= self.team_pool.len()
    }

    // Generate and embed showing the progress of this PVP fight
    pub fn get_progress_embed(&self) -> CreateEmbed {
        let team_size = self.pool_size / 2;
        let team_names = self.get_pool_list();
        CreateEmbed::new().fields(vec![
            (
                format!("PvP Match: {team_size}v{team_size}"),
                "Welcome to this PVP Match.",
                false,
            ),
            (
                format!(
                    "Registered CMDRs ({}/{})",
                    self.team_pool.len(),
                    self.pool_size
                ),
                &team_names,
                true,
            ),
            (
                "".to_string(),
                "Use the buttons below to manage this match",
                false,
            ),
        ])
    }

    // Get the embed that lists the details for a match ready to start.
    pub fn get_start_embed(&self) -> CreateEmbed {
        todo!();
    }
}
